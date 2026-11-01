//! Inter (OFL 1.1) — the app typeface.
//!
//! The four static faces are embedded from `assets/fonts/` and registered
//! into iced's font system at boot (`Session::boot` → `register`). Every
//! face declares family "Inter" (Medium/SemiBold via their typographic
//! family, which fontdb prefers over the legacy name), so a
//! `Family::Name("Inter")` query with a weight resolves to the matching
//! face: fontdb matches weights exactly first (CSS font matching), and the
//! embedded faces carry weight classes 400/500/600/700.

use iced::font::{self, Family, Font, Weight};
use iced::Task;

use crate::session::Message;

/// The app default font: Inter Regular. Wired via
/// `iced::application(...).default_font(INTER)` so every `text()` without
/// an explicit font renders in Inter instead of the system fallback.
pub const INTER: Font = Font::with_name("Inter");

/// Inter at `weight`. The font database picks the embedded face whose
/// weight class matches: Normal→Regular, Medium→Medium, Semibold→SemiBold,
/// Bold→Bold.
pub(crate) fn inter(weight: Weight) -> Font {
    Font {
        family: Family::Name("Inter"),
        weight,
        ..Font::DEFAULT
    }
}

const REGULAR: &[u8] = include_bytes!("../assets/fonts/Inter-Regular.ttf");
const MEDIUM: &[u8] = include_bytes!("../assets/fonts/Inter-Medium.ttf");
const SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/Inter-SemiBold.ttf");
const BOLD: &[u8] = include_bytes!("../assets/fonts/Inter-Bold.ttf");

/// Registers the four embedded weights in iced's font system. Called from
/// `Session::boot`; each face reports back as `Message::FontsLoaded` once
/// registered.
pub(crate) fn register() -> Task<Message> {
    Task::batch([
        font::load(REGULAR).map(|_| Message::FontsLoaded(Weight::Normal)),
        font::load(MEDIUM).map(|_| Message::FontsLoaded(Weight::Medium)),
        font::load(SEMIBOLD).map(|_| Message::FontsLoaded(Weight::Semibold)),
        font::load(BOLD).map(|_| Message::FontsLoaded(Weight::Bold)),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_inter_regular() {
        assert!(matches!(INTER.family, Family::Name("Inter")));
        assert_eq!(INTER.weight, Weight::Normal);
    }

    #[test]
    fn weights_select_inter_faces() {
        for weight in [
            Weight::Normal,
            Weight::Medium,
            Weight::Semibold,
            Weight::Bold,
        ] {
            let font = inter(weight);
            assert!(matches!(font.family, Family::Name("Inter")));
            assert_eq!(font.weight, weight);
        }
    }

    /// The embedded faces really declare family "Inter" (Medium/SemiBold via
    /// the typographic family, which fontdb indexes) and their own weight
    /// class — so `inter` above resolves to the right file at rendering time.
    #[test]
    fn embedded_faces_declare_inter_family_and_weights() {
        assert_face(REGULAR, "Inter", None, 400);
        assert_face(MEDIUM, "Inter Medium", Some("Inter"), 500);
        assert_face(SEMIBOLD, "Inter SemiBold", Some("Inter"), 600);
        assert_face(BOLD, "Inter", None, 700);
    }

    fn assert_face(bytes: &[u8], family: &str, typographic: Option<&str>, weight: u16) {
        let (legacy, typo, weight_class) = face_info(bytes);
        assert_eq!(legacy.as_deref(), Some(family));
        assert_eq!(typo.as_deref(), typographic);
        assert_eq!(weight_class, weight);
    }

    /// Reads the `name` table (legacy + typographic family) and the `OS/2`
    /// usWeightClass of an sfnt. Minimal std-only parser.
    fn face_info(bytes: &[u8]) -> (Option<String>, Option<String>, u16) {
        let mut name_off = 0usize;
        let mut os2_off = 0usize;
        for rec in bytes[12..].chunks_exact(16) {
            match &rec[0..4] {
                b"name" => name_off = be32(rec, 8) as usize,
                b"OS/2" => os2_off = be32(rec, 8) as usize,
                _ => {}
            }
        }
        (
            read_name(bytes, name_off, 1),
            read_name(bytes, name_off, 16),
            be16(bytes, os2_off + 4),
        )
    }

    /// First Windows-platform (UTF-16BE) string for `id` in the `name` table.
    fn read_name(bytes: &[u8], name_off: usize, id: u16) -> Option<String> {
        let count = be16(bytes, name_off + 2) as usize;
        let strings = be16(bytes, name_off + 4) as usize;
        for rec in bytes[name_off + 6..].chunks_exact(12).take(count) {
            if be16(rec, 0) != 3 || be16(rec, 6) != id {
                continue;
            }
            let len = be16(rec, 8) as usize;
            let off = be16(rec, 10) as usize;
            let start = name_off + strings + off;
            return Some(
                bytes[start..start + len]
                    .chunks(2)
                    .map(|c| char::from_u32(be16(c, 0) as u32).unwrap_or('\u{fffd}'))
                    .collect(),
            );
        }
        None
    }

    fn be16(b: &[u8], i: usize) -> u16 {
        u16::from_be_bytes([b[i], b[i + 1]])
    }

    fn be32(b: &[u8], i: usize) -> u32 {
        u32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
    }
}
