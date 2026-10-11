//! Preferências do Tsuro (`theme=dark|light`, `lang=pt|en`).
//!
//! Mesmo padrão de `recents_file()` em `browse.rs`: `TSURO_PREFS` vence,
//! depois macOS `~/Library/Application Support/Tsuro/prefs`, `$XDG_DATA_HOME`,
//! `~/.local/share`, por fim o temp dir. Leitura cega: qualquer erro → padrão.
//! Arquivo antigo só com `theme=` continua válido (`lang` cai para PT).

#[cfg(test)]
use std::cell::RefCell;
use std::path::PathBuf;

use crate::kiri::Theme;

#[cfg(test)]
thread_local! {
    static PREFS_PATH: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

#[cfg(test)]
pub fn with_prefs_path<R>(path: PathBuf, f: impl FnOnce() -> R) -> R {
    PREFS_PATH.with(|slot| *slot.borrow_mut() = Some(path));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    PREFS_PATH.with(|slot| *slot.borrow_mut() = None);
    match result {
        Ok(value) => value,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

pub fn prefs_file() -> PathBuf {
    #[cfg(test)]
    {
        if let Some(p) = PREFS_PATH.with(|slot| slot.borrow().clone()) {
            return p;
        }
    }
    if let Some(p) = std::env::var_os("TSURO_PREFS") {
        return PathBuf::from(p);
    }
    if cfg!(target_os = "macos") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join("Library/Application Support/Tsuro/prefs");
        }
    }
    if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(xdg).join("tsuro/prefs");
    }
    if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
        return PathBuf::from(home).join(".local/share/tsuro/prefs");
    }
    std::env::temp_dir().join("tsuro-prefs")
}

pub fn read_theme() -> Theme {
    pairs()
        .into_iter()
        .find(|(k, _)| k == "theme")
        .map(|(_, v)| v)
        .map(|v| match v.as_str() {
            "light" => Theme::Light,
            _ => Theme::default(),
        })
        .unwrap_or_default()
}

pub fn save_theme(theme: Theme) -> std::io::Result<()> {
    write_prefs(theme, read_lang())
}

/// Idioma da interface (infra da #settings: persistido, sem troca de strings
/// ainda — a migração dos textos é fase própria).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Lang {
    #[default]
    Pt,
    En,
}

pub fn read_lang() -> Lang {
    pairs()
        .into_iter()
        .find(|(k, _)| k == "lang")
        .map(|(_, v)| v)
        .map(|v| match v.as_str() {
            "en" => Lang::En,
            _ => Lang::Pt,
        })
        .unwrap_or_default()
}

pub fn save_lang(lang: Lang) -> std::io::Result<()> {
    write_prefs(read_theme(), lang)
}

fn pairs() -> Vec<(String, String)> {
    let Ok(raw) = std::fs::read_to_string(prefs_file()) else {
        return Vec::new();
    };
    raw.lines()
        .filter_map(|line| {
            let (k, v) = line.split_once('=')?;
            Some((k.trim().to_string(), v.trim().to_string()))
        })
        .collect()
}

fn write_prefs(theme: Theme, lang: Lang) -> std::io::Result<()> {
    let file = prefs_file();
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let theme = match theme {
        Theme::Dark => "dark",
        Theme::Light => "light",
    };
    let lang = match lang {
        Lang::Pt => "pt",
        Lang::En => "en",
    };
    std::fs::write(file, format!("theme={theme}\nlang={lang}\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_prefs(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "tsuro-prefs-unit-{}-{}-{name}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
        ))
    }

    #[test]
    fn missing_or_garbage_prefs_read_dark() {
        let path = temp_prefs("missing");
        let _ = std::fs::remove_file(&path);
        with_prefs_path(path.clone(), || {
            assert_eq!(read_theme(), Theme::Dark);
            std::fs::write(&path, "theme=banana\n").unwrap();
            assert_eq!(read_theme(), Theme::Dark);
        });
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn theme_roundtrips_through_file() {
        let path = temp_prefs("roundtrip");
        with_prefs_path(path.clone(), || {
            save_theme(Theme::Light).unwrap();
            assert_eq!(read_theme(), Theme::Light);
            save_theme(Theme::Dark).unwrap();
            assert_eq!(read_theme(), Theme::Dark);
        });
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn lang_roundtrips_and_preserves_theme() {
        let path = temp_prefs("lang");
        with_prefs_path(path.clone(), || {
            assert_eq!(read_lang(), Lang::Pt);
            save_theme(Theme::Light).unwrap();
            save_lang(Lang::En).unwrap();
            assert_eq!(read_lang(), Lang::En);
            assert_eq!(read_theme(), Theme::Light);
            save_theme(Theme::Dark).unwrap();
            assert_eq!(read_lang(), Lang::En);
            assert_eq!(read_theme(), Theme::Dark);
        });
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn legacy_theme_only_file_reads_pt() {
        let path = temp_prefs("legacy");
        with_prefs_path(path.clone(), || {
            std::fs::write(&path, "theme=light\n").unwrap();
            assert_eq!(read_theme(), Theme::Light);
            assert_eq!(read_lang(), Lang::Pt);
        });
        let _ = std::fs::remove_file(&path);
    }
}
