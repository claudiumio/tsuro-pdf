//! Marcadores pessoais de página (#134, F-MARC-07).
//!
//! Persistência local independente de `positions`, `recents` e diários de
//! recuperação. Cada marcador é `(path, page, note?)` identificado pela
//! mesma identidade `(size, mtime_ns)` usada em `positions.rs`: se o PDF
//! muda externamente, os marcadores não são reaplicados em silêncio.
//!
//! Formato (uma entrada por linha, mais recente primeiro, máx 50):
//! `página\tnota\ttamanho\tmtime\tcaminho` com `\`/`\t`/`\n` escapados
//! no caminho e na nota. Linha malformada é ignorada, nunca derruba a leitura.
//!
//! Toggle policy: criar marcador numa página que já tem um remove o existente
//! (sem duplicata silenciosa). Nota vazia ou só-espaço é tratada como `None`.

use std::path::{Path, PathBuf};

use crate::page::PageNo;
use crate::positions::{escape_field, escape_path, file_identity, unescape_field, unescape_path};

/// Entradas guardadas por arquivo (cap LRU).
pub const MAX_BOOKMARKS: usize = 50;

/// Um marcador pessoal: página + nota opcional.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookmark {
    pub page: PageNo,
    /// Nota curta do usuário; `None` ou vazio = sem nota.
    pub note: Option<String>,
}

/// Entrada persistida: caminho + identidade + marcador.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookmarkEntry {
    pub path: PathBuf,
    pub size: u64,
    pub mtime: u64,
    pub bookmark: Bookmark,
}

#[cfg(test)]
use std::cell::RefCell;

#[cfg(test)]
thread_local! {
    static BOOKMARKS_PATH: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
    static SCRATCH: crate::test_store::ScratchStore =
        crate::test_store::ScratchStore::new("tsuro-bookmarks-test");
}

#[cfg(test)]
pub fn with_bookmarks_path<R>(path: PathBuf, f: impl FnOnce() -> R) -> R {
    let previous = BOOKMARKS_PATH.with(|slot| slot.borrow_mut().replace(path));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    BOOKMARKS_PATH.with(|slot| *slot.borrow_mut() = previous);
    match result {
        Ok(value) => value,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

pub fn bookmarks_file() -> PathBuf {
    #[cfg(test)]
    {
        if let Some(p) = BOOKMARKS_PATH.with(|slot| slot.borrow().clone()) {
            return p;
        }
    }
    if let Some(p) = std::env::var_os("TSURO_BOOKMARKS") {
        return PathBuf::from(p);
    }
    #[cfg(test)]
    {
        SCRATCH.with(crate::test_store::ScratchStore::path)
    }
    #[cfg(not(test))]
    {
        if cfg!(target_os = "macos") {
            if let Some(home) = std::env::var_os("HOME") {
                return PathBuf::from(home).join("Library/Application Support/Tsuro/bookmarks");
            }
        }
        if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
            return PathBuf::from(xdg).join("tsuro/bookmarks");
        }
        if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            return PathBuf::from(home).join(".local/share/tsuro/bookmarks");
        }
        std::env::temp_dir().join("tsuro-bookmarks")
    }
}

fn encode_entry(entry: &BookmarkEntry) -> String {
    let note = entry.bookmark.note.as_deref().unwrap_or("");
    format!(
        "{}\t{}\t{}\t{}\t{}",
        entry.bookmark.page.index(),
        escape_field(note),
        entry.size,
        entry.mtime,
        escape_path(&entry.path),
    )
}

fn decode_entry(line: &str) -> Option<BookmarkEntry> {
    // 5 campos: página, nota, tamanho, mtime, caminho.
    let fields: Vec<&str> = line.split('\t').collect();
    let [page, note, size, mtime, path] = match fields[..] {
        [page, note, size, mtime, path] => [page, note, size, mtime, path],
        _ => return None,
    };
    let page_index = page.parse::<u32>().ok()?;
    let size = size.parse::<u64>().ok()?;
    let mtime = mtime.parse::<u64>().ok()?;
    let path = unescape_path(path);
    if path.as_os_str().is_empty() {
        return None;
    }
    let note_raw = unescape_field(note);
    let trimmed = note_raw.trim();
    let note = if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    };
    Some(BookmarkEntry {
        path,
        size,
        mtime,
        bookmark: Bookmark {
            page: PageNo::from_index(page_index),
            note,
        },
    })
}

/// Lê todas as entradas (mais recente primeiro); erro ou lixo → ignora.
pub fn read_bookmarks() -> Vec<BookmarkEntry> {
    let Ok(raw) = std::fs::read_to_string(bookmarks_file()) else {
        return Vec::new();
    };
    raw.lines().filter_map(decode_entry).collect()
}

pub fn save_bookmarks(entries: &[BookmarkEntry]) -> std::io::Result<()> {
    let file = bookmarks_file();
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut body = String::new();
    for entry in entries.iter().take(MAX_BOOKMARKS) {
        body.push_str(&encode_entry(entry));
        body.push('\n');
    }
    crate::io_atomic::write_file(&file, body)
}

/// Identidade atual do arquivo confere com a entrada salva.
fn identity_matches(entry: &BookmarkEntry, current: Option<(u64, u64)>) -> bool {
    match current {
        Some((size, mtime)) => {
            entry.size == size && crate::positions::mtime_matches_pub(entry.mtime, mtime)
        }
        None => false,
    }
}

/// Busca marcador válido para `path`: identidade precisa conferir.
pub fn find_bookmarks<'a>(entries: &'a [BookmarkEntry], path: &Path) -> Vec<&'a Bookmark> {
    let identity = file_identity(path);
    entries
        .iter()
        .filter(|e| e.path == path && identity_matches(e, identity))
        .map(|e| &e.bookmark)
        .collect()
}

/// Verifica se já existe marcador para esta página neste arquivo.
pub fn has_bookmark_at(entries: &[BookmarkEntry], path: &Path, page: PageNo) -> bool {
    let identity = file_identity(path);
    entries
        .iter()
        .any(|e| e.path == path && e.bookmark.page == page && identity_matches(e, identity))
}

/// Toggle: adiciona se não existe, remove se já existe na mesma página.
/// Retorna `true` se adicionou, `false` se removeu.
pub fn toggle_bookmark(
    mut entries: Vec<BookmarkEntry>,
    path: PathBuf,
    bookmark: Bookmark,
    identity: (u64, u64),
) -> (Vec<BookmarkEntry>, bool) {
    // Remove entrada existente na mesma página e mesmo arquivo (toggle).
    let before = entries.len();
    entries.retain(|e| !(e.path == path && e.bookmark.page == bookmark.page));
    let removed = entries.len() < before;

    if removed {
        // Toggle off: já removido acima.
        entries.truncate(MAX_BOOKMARKS);
        (entries, false)
    } else {
        // Toggle on: insere no topo (mais recente primeiro).
        entries.insert(
            0,
            BookmarkEntry {
                path,
                size: identity.0,
                mtime: identity.1,
                bookmark,
            },
        );
        entries.truncate(MAX_BOOKMARKS);
        (entries, true)
    }
}

/// Remove marcador específico por página. Retorna as entradas atualizadas.
pub fn remove_bookmark(
    mut entries: Vec<BookmarkEntry>,
    path: &Path,
    page: PageNo,
) -> Vec<BookmarkEntry> {
    entries.retain(|e| !(e.path == path && e.bookmark.page == page));
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_bookmarks(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "tsuro-bookmarks-unit-{}-{}-{name}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
        ))
    }

    fn entry(path: &str, page: u32, note: Option<&str>) -> BookmarkEntry {
        BookmarkEntry {
            path: PathBuf::from(path),
            size: 100,
            mtime: 200,
            bookmark: Bookmark {
                page: PageNo::from_index(page),
                note: note.map(|s| s.to_string()),
            },
        }
    }

    #[test]
    fn missing_or_garbage_reads_empty() {
        let path = temp_bookmarks("missing");
        let _ = std::fs::remove_file(&path);
        with_bookmarks_path(path.clone(), || {
            assert!(read_bookmarks().is_empty());
            std::fs::write(&path, "lixo\n1\t\n").unwrap();
            assert!(read_bookmarks().is_empty());
        });
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn roundtrip_with_and_without_note() {
        let path = temp_bookmarks("roundtrip");
        let _ = std::fs::remove_file(&path);
        with_bookmarks_path(path.clone(), || {
            let entries = vec![
                entry("/doc/a.pdf", 5, Some("Capítulo importante")),
                entry("/doc/b.pdf", 0, None),
            ];
            save_bookmarks(&entries).unwrap();
            let back = read_bookmarks();
            assert_eq!(back.len(), 2);
            assert_eq!(back[0].bookmark.page, PageNo::from_index(5));
            assert_eq!(
                back[0].bookmark.note.as_deref(),
                Some("Capítulo importante")
            );
            assert_eq!(back[1].bookmark.page, PageNo::from_index(0));
            assert_eq!(back[1].bookmark.note, None);
        });
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn toggle_adds_and_removes() {
        let entries = vec![entry("/doc/a.pdf", 3, Some("Nota"))];
        let identity = (100u64, 200u64);

        // Toggle off: mesma página → remove.
        let (result, added) = toggle_bookmark(
            entries.clone(),
            PathBuf::from("/doc/a.pdf"),
            Bookmark {
                page: PageNo::from_index(3),
                note: Some("Outra".to_string()),
            },
            identity,
        );
        assert!(!added);
        assert!(result.is_empty());

        // Toggle on: página nova → adiciona no topo.
        let (result, added) = toggle_bookmark(
            entries,
            PathBuf::from("/doc/a.pdf"),
            Bookmark {
                page: PageNo::from_index(7),
                note: None,
            },
            identity,
        );
        assert!(added);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].bookmark.page, PageNo::from_index(7));
    }

    #[test]
    fn cap_truncates_at_max() {
        let path = temp_bookmarks("cap");
        let _ = std::fs::remove_file(&path);
        with_bookmarks_path(path.clone(), || {
            let mut entries = Vec::new();
            let identity = (10u64, 20u64);
            for i in 0..(MAX_BOOKMARKS + 5) {
                let (new_entries, _) = toggle_bookmark(
                    entries,
                    PathBuf::from(format!("/doc/{i}.pdf")),
                    Bookmark {
                        page: PageNo::from_index(i as u32),
                        note: None,
                    },
                    identity,
                );
                entries = new_entries;
            }
            assert_eq!(entries.len(), MAX_BOOKMARKS);
            save_bookmarks(&entries).unwrap();
            let back = read_bookmarks();
            assert_eq!(back.len(), MAX_BOOKMARKS);
        });
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn stale_identity_is_rejected() {
        let entries = vec![entry("/doc/stale.pdf", 2, Some("Velho"))];
        // Identidade diferente → não encontra.
        assert!(find_bookmarks(&entries, Path::new("/doc/stale.pdf")).is_empty());
    }

    #[test]
    fn escape_note_roundtrip() {
        let path = temp_bookmarks("escape-note");
        let _ = std::fs::remove_file(&path);
        with_bookmarks_path(path.clone(), || {
            let entries = vec![BookmarkEntry {
                path: PathBuf::from("/doc/tab\tnl\nslash\\.pdf"),
                size: 50,
                mtime: 60,
                bookmark: Bookmark {
                    page: PageNo::from_index(1),
                    note: Some("nota\tcom\ttabs\ne\nquebras".to_string()),
                },
            }];
            save_bookmarks(&entries).unwrap();
            let back = read_bookmarks();
            assert_eq!(back.len(), 1);
            assert_eq!(
                back[0].bookmark.note.as_deref(),
                Some("nota\tcom\ttabs\ne\nquebras")
            );
            assert_eq!(back[0].path, PathBuf::from("/doc/tab\tnl\nslash\\.pdf"));
        });
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_bookmarks_leaves_no_tmp() {
        let path = temp_bookmarks("no-tmp");
        let _ = std::fs::remove_file(&path);
        with_bookmarks_path(path.clone(), || {
            save_bookmarks(&[entry("/doc/a.pdf", 1, None)]).unwrap();
            assert_eq!(read_bookmarks().len(), 1);
            let tmp = path.with_file_name(format!(
                "{}.tmp",
                path.file_name().unwrap().to_string_lossy()
            ));
            assert!(!tmp.exists());
        });
        let _ = std::fs::remove_file(&path);
    }
}
