//! Idioma da interface. `Lang::pick` exige os dois textos: uma tela sem
//! tradução não compila.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Lang {
    #[default]
    Pt,
    En,
}

impl Lang {
    pub fn pick(self, pt: &'static str, en: &'static str) -> &'static str {
        match self {
            Lang::Pt => pt,
            Lang::En => en,
        }
    }
}
