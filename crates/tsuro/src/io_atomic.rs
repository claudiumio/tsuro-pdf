//! Atomic file replace: write temp → sync → rename → sync parent dir.
//!
//! Mirrors the durable pattern in `recovery.rs`, without private modes.
//! A crash between write and rename leaves the previous target intact.

use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Write `bytes` to `path` so a crash mid-write cannot truncate the old file.
///
/// Steps: sibling `.tmp` → `write_all` → `sync_all` → `rename` → `sync_dir`.
/// Any failure after creating the temp removes the `.tmp` before returning.
pub(crate) fn write_file(path: &Path, bytes: impl AsRef<[u8]>) -> io::Result<()> {
    let tmp = unique_tmp_path(path)?;
    let result = (|| {
        let mut file = File::create(&tmp)?;
        file.write_all(bytes.as_ref())?;
        file.sync_all()?;
        drop(file);
        std::fs::rename(&tmp, path)?;
        if let Some(parent) = path.parent() {
            let parent = if parent.as_os_str().is_empty() {
                Path::new(".")
            } else {
                parent
            };
            sync_dir(parent)?;
        }
        Ok(())
    })();

    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}

static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_tmp_path(path: &Path) -> io::Result<PathBuf> {
    let name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "atomic write needs a file name",
        )
    })?;
    let seq = TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    Ok(path.with_file_name(format!(
        "{}.{}.{}.tmp",
        name.to_string_lossy(),
        std::process::id(),
        seq
    )))
}

#[cfg(test)]
fn tmp_path(path: &Path) -> io::Result<PathBuf> {
    let name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "atomic write needs a file name",
        )
    })?;
    Ok(path.with_file_name(format!("{}.tmp", name.to_string_lossy())))
}

/// Without fsyncing the directory, a rename may not survive power loss.
/// Windows cannot open a directory as a file; NTFS journals the rename.
fn sync_dir(dir: &Path) -> io::Result<()> {
    #[cfg(unix)]
    File::open(dir)?.sync_all()?;
    #[cfg(not(unix))]
    let _ = dir;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::io_atomic::write_file;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "tsuro-io-atomic-{}-{}-{name}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
        ))
    }

    #[test]
    fn write_file_roundtrips_bytes() {
        let path = temp_path("roundtrip.txt");
        let _ = std::fs::remove_file(&path);
        write_file(&path, b"alpha\n").unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"alpha\n");
        assert!(!any_tmp_exists(&path));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn orphan_tmp_does_not_touch_existing_target() {
        let path = temp_path("keep.txt");
        let _ = fs::remove_file(&path);
        write_file(&path, b"stable\n").unwrap();

        // A leftover old-format .tmp from a previous version must not affect the target.
        let legacy_tmp = path.with_file_name(format!("{}.tmp", path.file_name().unwrap().to_string_lossy()));
        fs::write(&legacy_tmp, b"crash-garbage\n").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "stable\n");

        let _ = fs::remove_file(&legacy_tmp);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn rename_failure_removes_tmp() {
        let dir = temp_path("as-dir");
        let _ = fs::remove_file(&dir);
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        // `dir` is a directory, so rename(tmp → dir) fails; tmp must go away.
        let err = write_file(&dir, b"nope\n").unwrap_err();
        assert!(err.kind() != std::io::ErrorKind::NotFound);
        assert!(!any_tmp_exists(&dir));

        let _ = fs::remove_dir_all(&dir);
    }

    /// Check that no temp file matching the new unique naming pattern exists for `path`.
    fn any_tmp_exists(path: &std::path::Path) -> bool {
        let parent = match path.parent() {
            Some(p) if !p.as_os_str().is_empty() => p,
            _ => return false,
        };
        let stem = match path.file_name() {
            Some(n) => n.to_string_lossy().into_owned(),
            None => return false,
        };
        let prefix = format!("{stem}.");
        fs::read_dir(parent)
            .map(|entries| {
                entries.filter_map(|e| e.ok()).any(|e| {
                    let name = e.file_name().to_string_lossy().into_owned();
                    name.starts_with(&prefix) && name.ends_with(".tmp")
                })
            })
            .unwrap_or(false)
    }
}
