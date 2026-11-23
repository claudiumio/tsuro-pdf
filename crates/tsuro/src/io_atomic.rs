//! Atomic file replace: write temp → sync → rename → sync parent dir.
//!
//! Mirrors the durable pattern in `recovery.rs`, without private modes.
//! A crash between write and rename leaves the previous target intact.

use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Write `bytes` to `path` so a crash mid-write cannot truncate the old file.
///
/// Steps: sibling `.tmp` → `write_all` → `sync_all` → `rename` → `sync_dir`.
/// Any failure after creating the temp removes the `.tmp` before returning.
pub(crate) fn write_file(path: &Path, bytes: impl AsRef<[u8]>) -> io::Result<()> {
    let tmp = tmp_path(path)?;
    let result = (|| {
        let mut file = File::create(&tmp)?;
        file.write_all(bytes.as_ref())?;
        file.sync_all()?;
        drop(file);
        std::fs::rename(&tmp, path)?;
        if let Some(parent) = path.parent() {
            sync_dir(parent)?;
        }
        Ok(())
    })();

    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}

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
    use super::*;
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
        assert!(!tmp_path(&path).unwrap().exists());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn orphan_tmp_does_not_touch_existing_target() {
        let path = temp_path("keep.txt");
        let _ = std::fs::remove_file(&path);
        write_file(&path, b"stable\n").unwrap();

        let tmp = tmp_path(&path).unwrap();
        std::fs::write(&tmp, b"crash-garbage\n").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "stable\n");

        let _ = std::fs::remove_file(&tmp);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn rename_failure_removes_tmp() {
        let dir = temp_path("as-dir");
        let _ = std::fs::remove_file(&dir);
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // `dir` is a directory, so rename(tmp → dir) fails; tmp must go away.
        let err = write_file(&dir, b"nope\n").unwrap_err();
        assert!(err.kind() != io::ErrorKind::NotFound);
        assert!(!tmp_path(&dir).unwrap().exists());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
