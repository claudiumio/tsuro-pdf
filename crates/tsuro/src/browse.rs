#[cfg(test)]
use std::cell::RefCell;
use std::path::{Path, PathBuf};

use crate::kiri::Theme;

pub const RECENTS_CAP: usize = 12;

#[cfg(test)]
thread_local! {
    static RECENTS_PATH: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

#[cfg(test)]
pub fn with_recents_path<R>(path: PathBuf, f: impl FnOnce() -> R) -> R {
    RECENTS_PATH.with(|slot| *slot.borrow_mut() = Some(path));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    RECENTS_PATH.with(|slot| *slot.borrow_mut() = None);
    match result {
        Ok(value) => value,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

#[derive(Debug, Clone, Default)]
pub struct EmptyState {
    pub cwd: Option<PathBuf>,
    pub listing: Vec<FsEntry>,
    pub listing_error: Option<String>,
    pub recents: Vec<PathBuf>,
    /// Última geração de `Opened`; sobrevive ao Close para recusar resultado atrasado.
    pub open_gen: u64,
    /// Tema Kiri — `Default` é Dark; `Session::empty()` carrega do prefs.
    pub theme: Theme,
    /// DPR da janela; 0.0 (`Default`) = desconhecido, `page_scale` trata como 1.0.
    pub render_scale: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
}

pub fn is_pdf(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
}

pub fn recents_file() -> PathBuf {
    #[cfg(test)]
    {
        if let Some(p) = RECENTS_PATH.with(|slot| slot.borrow().clone()) {
            return p;
        }
    }
    if let Some(p) = std::env::var_os("TSURO_RECENTS") {
        return PathBuf::from(p);
    }
    if cfg!(target_os = "macos") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join("Library/Application Support/Tsuro/recents");
        }
    }
    if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(xdg).join("tsuro/recents");
    }
    if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
        return PathBuf::from(home).join(".local/share/tsuro/recents");
    }
    std::env::temp_dir().join("tsuro-recents")
}

pub fn read_recents() -> Vec<PathBuf> {
    read_recents_from(&recents_file())
}

fn read_recents_from(file: &Path) -> Vec<PathBuf> {
    let Ok(raw) = std::fs::read_to_string(file) else {
        return Vec::new();
    };
    let mut out: Vec<PathBuf> = Vec::new();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let path = PathBuf::from(line);
        if !is_pdf(&path) {
            continue;
        }
        let key = normalize_recent(&path);
        if !out.iter().any(|p| normalize_recent(p) == key) {
            out.push(path);
        }
        if out.len() == RECENTS_CAP {
            break;
        }
    }
    out
}

pub fn save_recents(paths: &[PathBuf]) -> std::io::Result<()> {
    let file = recents_file();
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut body = String::new();
    for path in paths.iter().take(RECENTS_CAP) {
        body.push_str(&path.to_string_lossy());
        body.push('\n');
    }
    std::fs::write(file, body)
}

pub fn push_recent(mut recents: Vec<PathBuf>, path: PathBuf) -> Vec<PathBuf> {
    let key = normalize_recent(&path);
    recents.retain(|p| normalize_recent(p) != key);
    recents.insert(0, path);
    recents.truncate(RECENTS_CAP);
    recents
}

/// Chave de comparação: absolutiza (cwd atual) e colapsa `.`/`..` por
/// via léxica, sem tocar o disco. Só deduplica — o original é guardado
/// intacto (`a/../b.pdf` abre igual a `b.pdf`).
fn normalize_recent(path: &Path) -> PathBuf {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|cwd| cwd.join(path))
            .unwrap_or_else(|_| path.to_path_buf())
    };
    let mut clean = PathBuf::new();
    for comp in absolute.components() {
        match comp {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if !clean.pop() && clean.as_os_str().is_empty() {
                    clean.push("..");
                }
            }
            other => clean.push(other.as_os_str()),
        }
    }
    if clean.as_os_str().is_empty() {
        return absolute;
    }
    clean
}

pub fn drop_recent(mut recents: Vec<PathBuf>, path: &Path) -> Vec<PathBuf> {
    let key = normalize_recent(path);
    recents.retain(|p| normalize_recent(p) != key);
    recents
}

pub fn merge_recents(primary: Vec<PathBuf>, secondary: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    for path in primary.into_iter().chain(secondary) {
        let key = normalize_recent(&path);
        if !out.iter().any(|p| normalize_recent(p) == key) {
            out.push(path);
        }
        if out.len() == RECENTS_CAP {
            break;
        }
    }
    out
}

pub fn special_folders() -> Vec<FsEntry> {
    let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) else {
        return Vec::new();
    };
    let home = PathBuf::from(home);
    let candidates = [
        ("Início", home.clone()),
        ("Documentos", home.join("Documents")),
        ("Downloads", home.join("Downloads")),
        ("Desktop", home.join("Desktop")),
    ];
    candidates
        .into_iter()
        .filter(|(_, path)| path.is_dir())
        .map(|(name, path)| FsEntry {
            name: name.to_string(),
            path,
            is_dir: true,
        })
        .collect()
}

pub fn list_dir(path: &Path) -> Result<Vec<FsEntry>, String> {
    let mut entries = Vec::new();
    let reader = std::fs::read_dir(path).map_err(friendly_list_error)?;
    for ent in reader {
        let ent = ent.map_err(friendly_list_error)?;
        let path = ent.path();
        let name = ent.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let is_dir = path.is_dir();
        if is_dir || is_pdf(&path) {
            entries.push(FsEntry { path, name, is_dir });
        }
    }
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });
    Ok(entries)
}

pub fn parent_of(cwd: &Path) -> Option<PathBuf> {
    if special_folders().iter().any(|e| e.path == cwd) {
        return None;
    }
    cwd.parent().map(|p| p.to_path_buf())
}

/// Returns whether `read_dir` can open the directory iterator.
/// Entries may still fail during iteration. Opening may request OS consent.
pub fn dir_accessible(path: &Path) -> bool {
    dir_access(path).is_ok()
}

pub(crate) fn dir_access(path: &Path) -> Result<(), std::io::ErrorKind> {
    std::fs::read_dir(path)
        .map(|_| ())
        .map_err(|error| error.kind())
}

/// Downloads directory used by the settings permission check.
pub fn downloads_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(|home| PathBuf::from(home).join("Downloads"))
}

/// PermissionDenied does not distinguish folder permissions from OS consent.
fn friendly_list_error(e: std::io::Error) -> String {
    if e.kind() != std::io::ErrorKind::PermissionDenied {
        return e.to_string();
    }
    if cfg!(target_os = "macos") {
        "Sem acesso a esta pasta. Confira as permissões da pasta. Se o macOS bloqueou o acesso, confira Ajustes do Sistema › Privacidade e Segurança › Arquivos e Pastas › TsuroPDF.".to_string()
    } else if cfg!(target_os = "windows") {
        "Sem acesso a esta pasta. Confira as permissões da pasta no Windows.".to_string()
    } else {
        "Sem acesso a esta pasta (permissão negada).".to_string()
    }
}

pub fn display_path(cwd: Option<&Path>) -> String {
    match cwd {
        None => "Pastas".to_string(),
        Some(path) => path.display().to_string(),
    }
}

pub async fn list_path(path: Option<PathBuf>) -> Result<Vec<FsEntry>, String> {
    tokio::task::spawn_blocking(move || match &path {
        None => Ok(special_folders()),
        Some(p) => list_dir(p),
    })
    .await
    .map_err(|e| e.to_string())?
}

pub async fn load_recents() -> Vec<PathBuf> {
    // Captura o path nesta thread: `thread_local` de teste não atravessa o worker.
    let file = recents_file();
    tokio::task::spawn_blocking(move || read_recents_from(&file))
        .await
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_dir_keeps_only_folders_and_pdfs() {
        let root = std::env::temp_dir().join(format!(
            "tsuro-list-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(root.join("a.pdf"), b"%PDF").unwrap();
        std::fs::write(root.join("a.PDF"), b"%PDF").unwrap();
        std::fs::write(root.join("note.txt"), b"hi").unwrap();
        std::fs::write(root.join(".hidden.pdf"), b"%PDF").unwrap();
        let entries = list_dir(&root).unwrap();
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"sub"));
        assert!(names.iter().any(|n| n.eq_ignore_ascii_case("a.pdf")));
        assert!(!names.contains(&"note.txt"));
        assert!(!names.contains(&".hidden.pdf"));
        assert!(entries.iter().any(|e| e.name == "sub" && e.is_dir));
        assert!(entries.iter().any(|e| !e.is_dir && is_pdf(&e.path)));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn push_recent_dedups_caps_and_persists() {
        let path = std::env::temp_dir().join(format!(
            "tsuro-recents-unit-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        with_recents_path(path.clone(), || {
            let mut recents = Vec::new();
            recents = push_recent(recents, PathBuf::from("/tmp/a.pdf"));
            recents = push_recent(recents, PathBuf::from("/tmp/b.pdf"));
            recents = push_recent(recents, PathBuf::from("/tmp/a.pdf"));
            assert_eq!(recents[0], PathBuf::from("/tmp/a.pdf"));
            assert_eq!(recents.len(), 2);
            for i in 0..20 {
                recents = push_recent(recents, PathBuf::from(format!("/tmp/n{i}.pdf")));
            }
            assert_eq!(recents.len(), RECENTS_CAP);
            save_recents(&recents).unwrap();
            let loaded = read_recents();
            assert_eq!(loaded, recents);
        });
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn read_recents_skips_non_pdf() {
        let path = std::env::temp_dir().join(format!(
            "tsuro-recents-filter-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        with_recents_path(path.clone(), || {
            save_recents(&[
                PathBuf::from("/tmp/ok.pdf"),
                PathBuf::from("/tmp/note.txt"),
                PathBuf::from("/tmp/also.PDF"),
            ])
            .unwrap();
            let loaded = read_recents();
            assert_eq!(
                loaded,
                vec![PathBuf::from("/tmp/ok.pdf"), PathBuf::from("/tmp/also.PDF")]
            );
        });
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn push_recent_dedups_across_dotdot_spellings() {
        let recents = push_recent(Vec::new(), PathBuf::from("/tmp/sub/../a.pdf"));
        let recents = push_recent(recents, PathBuf::from("/tmp/a.pdf"));
        assert_eq!(recents, vec![PathBuf::from("/tmp/a.pdf")]);
    }

    #[test]
    fn push_recent_dedups_relative_against_absolute() {
        let abs = std::env::current_dir().unwrap().join("relativa.pdf");
        let recents = push_recent(Vec::new(), PathBuf::from("relativa.pdf"));
        let recents = push_recent(recents, abs.clone());
        assert_eq!(recents, vec![abs]);
    }

    #[test]
    fn read_recents_dedups_across_dotdot_spellings() {
        let path = std::env::temp_dir().join(format!(
            "tsuro-recents-clean-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        with_recents_path(path.clone(), || {
            save_recents(&[
                PathBuf::from("/tmp/x/../ok.pdf"),
                PathBuf::from("/tmp/ok.pdf"),
            ])
            .unwrap();
            assert_eq!(read_recents(), vec![PathBuf::from("/tmp/x/../ok.pdf")]);
        });
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn merge_recents_keeps_primary_order_then_disk() {
        let merged = merge_recents(
            vec![PathBuf::from("/tmp/new.pdf")],
            vec![PathBuf::from("/tmp/old.pdf"), PathBuf::from("/tmp/new.pdf")],
        );
        assert_eq!(
            merged,
            vec![PathBuf::from("/tmp/new.pdf"), PathBuf::from("/tmp/old.pdf")]
        );
    }

    #[test]
    fn list_error_denied_does_not_assume_os_consent_is_the_cause() {
        let denied = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "os error 1");
        let msg = friendly_list_error(denied);
        assert!(msg.starts_with("Sem acesso a esta pasta"), "{msg}");
        if cfg!(target_os = "macos") {
            assert!(msg.contains("permissões da pasta"), "{msg}");
            assert!(msg.contains("Se o macOS"), "{msg}");
            assert!(msg.contains("Arquivos e Pastas"), "{msg}");
        }
    }

    #[test]
    fn list_error_other_passes_through() {
        let missing = std::io::Error::new(std::io::ErrorKind::NotFound, "sem tal pasta");
        assert_eq!(friendly_list_error(missing), "sem tal pasta");
        assert!(list_dir(Path::new("/tsuro-nao-existe-xyz")).is_err());
    }

    #[test]
    fn dir_accessible_matches_readable_tmp() {
        let root = std::env::temp_dir().join(format!(
            "tsuro-access-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        assert!(dir_accessible(&root));
        assert!(!dir_accessible(&root.join("nao-existe")));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn downloads_dir_ends_with_downloads() {
        assert!(downloads_dir()
            .map(|p| p.ends_with("Downloads"))
            .unwrap_or(true));
    }

    #[test]
    fn dir_access_preserves_missing_path_error_without_creating_folders() {
        let missing = std::env::temp_dir().join(format!(
            "tsuro-missing-downloads-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        assert_eq!(dir_access(&missing), Err(std::io::ErrorKind::NotFound));
        assert!(!missing.exists());
        assert_eq!(dir_access(Path::new(env!("CARGO_MANIFEST_DIR"))), Ok(()));
    }
}
