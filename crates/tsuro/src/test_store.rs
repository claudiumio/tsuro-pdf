//! Caminho descartável por thread para os stores persistidos (`positions`,
//! `recents`) nos testes de unidade. Sem ele, `cargo test` lê e grava os
//! arquivos reais do usuário: o store global vaza entre testes (a página
//! restaurada por um teste derruba o assert de outro) e sobra lixo na máquina.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Arquivo temporário exclusivo da thread de teste; some quando ela termina.
pub(crate) struct ScratchStore(PathBuf);

impl ScratchStore {
    pub(crate) fn new(prefix: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let order = NEXT.fetch_add(1, Ordering::Relaxed);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        Self(std::env::temp_dir().join(format!("{prefix}-{}-{nanos}-{order}", std::process::id())))
    }

    pub(crate) fn path(&self) -> PathBuf {
        self.0.clone()
    }
}

impl Drop for ScratchStore {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
