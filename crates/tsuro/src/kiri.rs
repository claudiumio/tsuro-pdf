//! Kiri (きり) — design tokens do chrome Tsuro.
//!
//! Dark mode é o padrão; light é opt-in via menu ⋯ → Aparência.
//! Folha PDF é sempre branca (`Tokens::page`), independente do tema.
//! Ícones Ori têm cor fixa no SVG — nunca aplique `svg::Style { color }`.

use iced::widget::{button, container, svg, text_input, Space};
use iced::{Background, Border, Color, Length, Shadow, Vector};

use crate::i18n::Lang;

/// Tema do chrome. Vive no estado da app, não no `Theme` global do iced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    pub fn is_dark(self) -> bool {
        matches!(self, Theme::Dark)
    }
}

/// Tokens nomeados do Kiri. 15 campos — não mapeiam 1:1 ao `Seed` do
/// `iced::Theme::custom`, por isso vivem aqui + closures `.style()`.
#[derive(Debug, Clone, Copy)]
pub struct Tokens {
    pub is_dark: bool,
    pub bg: Color,
    pub surface: Color,
    pub chrome: Color,
    pub elevated: Color,
    pub ink: Color,
    pub muted: Color,
    pub line: Color,
    pub accent: Color,
    pub accent_bg: Color,
    pub hover: Color,
    pub ok: Color,
    pub warn: Color,
    pub danger: Color,
    /// Folha PDF — sempre branca, qualquer tema.
    pub page: Color,
    /// Highlight de busca.
    pub mark: Color,
    /// Hit atual da busca (Enter/F3) — âmbar cheio sobre o amarelo claro.
    pub mark_current: Color,
    /// Idioma da interface: textos do chrome saem de `lang.pick(pt, en)`.
    pub lang: Lang,
}

impl Tokens {
    pub fn for_theme(theme: Theme, lang: Lang) -> Self {
        match theme {
            Theme::Dark => Self {
                is_dark: true,
                lang,
                bg: hex(0x18, 0x18, 0x18),
                surface: hex(0x22, 0x22, 0x22),
                chrome: hex(0x1a, 0x1a, 0x1a),
                elevated: hex(0x26, 0x26, 0x26),
                ink: hex(0xec, 0xec, 0xea),
                muted: hex(0x9a, 0x98, 0x90),
                line: hex(0x33, 0x33, 0x31),
                accent: hex(0x4d, 0xb8, 0xb0),
                accent_bg: hex(0x1e, 0x33, 0x31),
                hover: hex(0x2a, 0x2a, 0x28),
                ok: hex(0x5c, 0xb8, 0x5c),
                warn: hex(0xd4, 0xa0, 0x17),
                danger: hex(0xc0, 0x39, 0x2b),
                page: Color::WHITE,
                mark: hex(0xfd, 0xe6, 0x8a),
                mark_current: hex(0xf5, 0x9e, 0x0b),
            },
            Theme::Light => Self {
                is_dark: false,
                lang,
                bg: hex(0xef, 0xef, 0xed),
                surface: Color::WHITE,
                chrome: hex(0xf7, 0xf7, 0xf6),
                elevated: Color::WHITE,
                ink: hex(0x1c, 0x1c, 0x1a),
                muted: hex(0x78, 0x76, 0x70),
                line: hex(0xe0, 0xde, 0xd9),
                accent: hex(0x1f, 0x7d, 0x76),
                accent_bg: hex(0xe0, 0xf2, 0xf0),
                hover: hex(0xe8, 0xe8, 0xe5),
                ok: hex(0x5c, 0xb8, 0x5c),
                warn: hex(0xd4, 0xa0, 0x17),
                danger: hex(0xc0, 0x39, 0x2b),
                page: Color::WHITE,
                mark: hex(0xfd, 0xe6, 0x8a),
                mark_current: hex(0xf5, 0x9e, 0x0b),
            },
        }
    }
}

fn hex(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb8(r, g, b)
}

// ── Visual foundation (docs/design/01-fundacao-visual.md) ─────────────────
// Approved-spec tokens beside the legacy `Tokens`. Nothing here is consumed
// yet: no rendered pixel changes in this phase — `view.rs` migrates onto
// these tables in phase 2.

// Typography (spec §2) ──────────────────────────────────────────────────────

/// One named text style: size and line-height in px, numeric weight
/// (400 regular, 600 emphasized, 700 headline) [kit]. SF tracking does not
/// apply to Inter — size and line-height are copied, not tracking.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextStyle {
    pub size: f32,
    pub line_height: f32,
    pub weight: u16,
}

/// Named styles of the scale (spec §2 table). Floor 10 pt, body 13/16 [HIG].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextKind {
    Title2,
    Title3,
    Headline,
    Body,
    Callout,
    Subheadline,
    Footnote,
}

/// Pure mapping style + emphasis -> metrics. Emphasized lifts 400 to 600;
/// Headline stays 700 in both variants [kit].
pub fn text_style(kind: TextKind, emphasized: bool) -> TextStyle {
    let (size, line_height, regular, strong) = match kind {
        TextKind::Title2 => (17.0, 22.0, 400, 600),
        TextKind::Title3 => (15.0, 20.0, 400, 600),
        TextKind::Headline => (13.0, 16.0, 700, 700),
        TextKind::Body => (13.0, 16.0, 400, 600),
        TextKind::Callout => (12.0, 15.0, 400, 600),
        TextKind::Subheadline => (11.0, 14.0, 400, 600),
        TextKind::Footnote => (10.0, 13.0, 400, 600),
    };
    TextStyle {
        size,
        line_height,
        weight: if emphasized { strong } else { regular },
    }
}

// Semantic colors (spec §3) ─────────────────────────────────────────────────

/// Semantic color roles (spec §3 table) for both themes, kit values. The kit
/// specifies translucent roles (`#000 85%`, `#FFF 55%`); the app's declared
/// substitute is flat opaque surfaces (spec §5 menu note, principle 4), so
/// each alpha is composed over the theme's `window_bg`, rounded half up.
/// Not consumed yet — phase 2 migrates `view.rs` onto these roles.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Roles {
    pub label: Color,
    pub label_2: Color,
    pub label_3: Color,
    pub fill_1: Color,
    pub fill_3: Color,
    pub separator: Color,
    pub window_bg: Color,
    pub under_page: Color,
    /// Brand accent, Kiri teal (decision D-4 — not the kit's system blue).
    pub accent: Color,
    pub red: Color,
    pub green: Color,
    pub orange: Color,
    /// PDF sheet — always white in both themes (declared exception, spec §3).
    pub page: Color,
}

impl Roles {
    pub fn for_theme(theme: Theme) -> Self {
        match theme {
            Theme::Dark => Self {
                label: Color::WHITE,              // #FFF 100%
                label_2: hex(0x9a, 0x9a, 0x9a),   // #FFF 55% over #1E1E1E
                label_3: hex(0x56, 0x56, 0x56),   // #FFF 25%
                fill_1: hex(0x35, 0x35, 0x35),    // #FFF 10%
                fill_3: hex(0x29, 0x29, 0x29),    // #FFF 5%
                separator: hex(0x35, 0x35, 0x35), // #FFF 10%
                window_bg: hex(0x1e, 0x1e, 0x1e),
                under_page: hex(0x18, 0x18, 0x18),
                accent: hex(0x4d, 0xb8, 0xb0), // D-4 teal (kit blue: #0A99FF)
                red: hex(0xff, 0x47, 0x47),
                green: hex(0x3b, 0xdb, 0x63),
                orange: hex(0xff, 0x9e, 0x33),
                page: Color::WHITE,
            },
            Theme::Light => Self {
                label: hex(0x26, 0x26, 0x26),     // #000 85% over #FFF
                label_2: hex(0x80, 0x80, 0x80),   // #000 50%
                label_3: hex(0xbf, 0xbf, 0xbf),   // #000 25%
                fill_1: hex(0xe6, 0xe6, 0xe6),    // #000 10%
                fill_3: hex(0xf2, 0xf2, 0xf2),    // #000 5%
                separator: hex(0xc6, 0xc6, 0xc8), // #3C3C43 29%
                window_bg: Color::WHITE,
                under_page: hex(0xef, 0xef, 0xed),
                accent: hex(0x1f, 0x7d, 0x76), // D-4 teal (kit blue: #0078F0)
                red: hex(0xff, 0x38, 0x3c),
                green: hex(0x34, 0xc7, 0x59),
                orange: hex(0xff, 0x8d, 0x28),
                page: Color::WHITE,
            },
        }
    }
}

// Spacing, control heights, radius, elevation (spec §4) ─────────────────────

/// Step of the 4 px spacing grid (spec §4) [HIG].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    S4,
    S8,
    S12,
    S16,
    S20,
    S24,
}

/// Spacing step -> px.
pub fn spacing(step: Spacing) -> f32 {
    match step {
        Spacing::S4 => 4.0,
        Spacing::S8 => 8.0,
        Spacing::S12 => 12.0,
        Spacing::S16 => 16.0,
        Spacing::S20 => 20.0,
        Spacing::S24 => 24.0,
    }
}

/// Control heights (spec §4) [kit]: 20 mini, 24 regular, 28 alert-button
/// default, 36 large. Minimum hit area 20×20, default 28×28 [HIG].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlHeight {
    Mini,
    Regular,
    Alert,
    Large,
}

/// Control height -> px.
pub fn control_height(height: ControlHeight) -> f32 {
    match height {
        ControlHeight::Mini => 20.0,
        ControlHeight::Regular => 24.0,
        ControlHeight::Alert => 28.0,
        ControlHeight::Large => 36.0,
    }
}

/// Corner-radius scale (spec §4) [kit]. `Pill` is the kit's ∞, rendered as
/// 99 — the large-value convention the current styles already use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Radius {
    Mini,
    Button,
    Item,
    Tab,
    Menu,
    Window,
    Popover,
    Sheet,
    Pill,
}

/// Radius role -> px.
pub fn radius(role: Radius) -> f32 {
    match role {
        Radius::Mini => 4.0,     // mini
        Radius::Button => 6.0,   // button / field / content segment
        Radius::Item => 8.0,     // sidebar item, menu item, selection
        Radius::Tab => 9.0,      // tab
        Radius::Menu => 12.0,    // menu
        Radius::Window => 16.0,  // window
        Radius::Popover => 20.0, // popover
        Radius::Sheet => 26.0,   // dialog sheet
        Radius::Pill => 99.0,    // ∞: bar button, alert, search field, rail
    }
}

/// Elevation preset of a raised surface (spec §4) [kit]. The kit draws the
/// window/menu ring as a zero-offset shadow (`0 0 0 Npx rgba(0,0,0,.2)`);
/// iced has no shadow spread, so the ring is stored as a border.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElevationSpec {
    /// Ring around the surface: (width, color).
    pub ring: Option<(f32, Color)>,
    /// Shadow layers in kit order, up to two.
    pub shadows: [Option<Shadow>; 2],
    /// Modal scrim color; only the `Scrim` preset sets it.
    pub scrim: Option<Color>,
}

/// Elevation presets (spec §4) [kit]. Scrim pins modal black at 25% — the
/// app repeats 0.25/0.55 in seven places today (spec §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elevation {
    Window,
    Menu,
    Popover,
    Tooltip,
    Scrim,
}

/// Elevation preset -> ring, shadows, scrim.
pub fn elevation(preset: Elevation) -> ElevationSpec {
    match preset {
        Elevation::Window => ElevationSpec {
            ring: Some((1.0, Color::from_rgba(0.0, 0.0, 0.0, 0.2))),
            shadows: [
                Some(Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                    offset: Vector::new(0.0, 16.0),
                    blur_radius: 33.0,
                }),
                None,
            ],
            scrim: None,
        },
        Elevation::Menu => ElevationSpec {
            ring: Some((0.5, Color::from_rgba(0.0, 0.0, 0.0, 0.2))),
            shadows: [
                Some(Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 24.0,
                }),
                None,
            ],
            scrim: None,
        },
        Elevation::Popover => ElevationSpec {
            // Kit says "+ hairline" without width/color; kit ring convention.
            ring: Some((0.5, Color::from_rgba(0.0, 0.0, 0.0, 0.2))),
            shadows: [
                Some(Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                    offset: Vector::new(0.0, 18.0),
                    blur_radius: 46.0,
                }),
                None,
            ],
            scrim: None,
        },
        Elevation::Tooltip => ElevationSpec {
            ring: None,
            shadows: [
                Some(Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 2.0,
                }),
                Some(Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                }),
            ],
            scrim: None,
        },
        Elevation::Scrim => ElevationSpec {
            ring: None,
            shadows: [None, None],
            scrim: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.25)),
        },
    }
}

/// Ícone Ori (origami tsuru) — cor fixa no SVG, sem `.style()`.
/// Uso: `ori!("folder-open")` → 16×16. Pílula de busca usa `ori_small!`.
/// V2 (28 ícones): folder, file-text, x, copy, highlighter, underline,
/// strike, note, fit-page, folder-open, search, chevron-left, chevron-right,
/// minus, plus, fit-width, pages, shield, home, more, print, rotate, undo,
/// redo, save, page-single, continuous, settings.
#[allow(unused_macros)]
macro_rules! ori {
    ($file:literal) => {
        $crate::kiri::ori_icon($file, 16.0)
    };
}

#[allow(unused_macros)]
macro_rules! ori_small {
    ($file:literal) => {
        $crate::kiri::ori_icon($file, 15.0)
    };
}

#[allow(unused_imports)]
pub(crate) use ori;
#[allow(unused_imports)]
pub(crate) use ori_small;

pub fn ori_icon<Message: 'static>(file: &str, size: f32) -> iced::Element<'static, Message> {
    let bytes: &'static [u8] = match file {
        "folder-open" => include_bytes!("../assets/icons/ori/folder-open.svg"),
        "search" => include_bytes!("../assets/icons/ori/search.svg"),
        "chevron-left" => include_bytes!("../assets/icons/ori/chevron-left.svg"),
        "chevron-right" => include_bytes!("../assets/icons/ori/chevron-right.svg"),
        "minus" => include_bytes!("../assets/icons/ori/minus.svg"),
        "plus" => include_bytes!("../assets/icons/ori/plus.svg"),
        "fit-width" => include_bytes!("../assets/icons/ori/fit-width.svg"),
        "pages" => include_bytes!("../assets/icons/ori/pages.svg"),
        "shield" => include_bytes!("../assets/icons/ori/shield.svg"),
        "home" => include_bytes!("../assets/icons/ori/home.svg"),
        "more" => include_bytes!("../assets/icons/ori/more.svg"),
        "print" => include_bytes!("../assets/icons/ori/print.svg"),
        "settings" => include_bytes!("../assets/icons/ori/settings.svg"),
        // v2
        "folder" => include_bytes!("../assets/icons/ori/folder.svg"),
        "file-text" => include_bytes!("../assets/icons/ori/file-text.svg"),
        "x" => include_bytes!("../assets/icons/ori/x.svg"),
        "copy" => include_bytes!("../assets/icons/ori/copy.svg"),
        "highlighter" => include_bytes!("../assets/icons/ori/highlighter.svg"),
        "underline" => include_bytes!("../assets/icons/ori/underline.svg"),
        "strike" => include_bytes!("../assets/icons/ori/strike.svg"),
        "note" => include_bytes!("../assets/icons/ori/note.svg"),
        "fit-page" => include_bytes!("../assets/icons/ori/fit-page.svg"),
        "rotate" => include_bytes!("../assets/icons/ori/rotate.svg"),
        "undo" => include_bytes!("../assets/icons/ori/undo.svg"),
        "redo" => include_bytes!("../assets/icons/ori/redo.svg"),
        "save" => include_bytes!("../assets/icons/ori/save.svg"),
        "page-single" => include_bytes!("../assets/icons/ori/page-single.svg"),
        "continuous" => include_bytes!("../assets/icons/ori/continuous.svg"),
        _ => include_bytes!("../assets/icons/ori/more.svg"),
    };
    svg(svg::Handle::from_memory(bytes))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .into()
}

/// Separador vertical 1×18px da toolbar.
pub fn vsep(tokens: Tokens) -> iced::Element<'static, crate::session::Message> {
    container(Space::with_width(Length::Fixed(1.0)))
        .width(Length::Fixed(1.0))
        .height(Length::Fixed(18.0))
        .padding([2, 0])
        .style(move |_| container::Style {
            background: Some(Background::Color(tokens.line)),
            ..container::Style::default()
        })
        .into()
}

/// Botão-ícone Kiri: fundo transparente, `hover`, `accent_bg` quando ativo.
pub fn ibtn_style(
    tokens: Tokens,
    active: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let background = if active {
            tokens.accent_bg
        } else {
            match status {
                button::Status::Hovered | button::Status::Pressed => tokens.hover,
                _ => Color::TRANSPARENT,
            }
        };
        button::Style {
            background: Some(Background::Color(background)),
            text_color: tokens.ink,
            border: Border {
                radius: 8.0.into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
        }
    }
}

/// Pílula central (busca + página): fundo `elevated`, raio 99px.
pub fn pill_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.elevated)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 99.0.into(),
        },
        text_color: Some(tokens.ink),
        ..container::Style::default()
    }
}

/// Segmento Kiri (−/+ · ◀▶): fundo `elevated` no dark, `surface` no light, raio 99px.
pub fn seg_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(if tokens.is_dark {
            tokens.elevated
        } else {
            tokens.surface
        })),
        border: Border {
            radius: 99.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

/// Aba como pílula (nome e ×): hairline `line`, raio 8 igual ao do `ibtn` que
/// fica dentro, para o hover não passar da borda. `accent_bg` na ativa;
/// `elevated` (dark) ou `surface` (light) nas demais.
pub fn tab_pill_style(tokens: Tokens, active: bool) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(if active {
            tokens.accent_bg
        } else if tokens.is_dark {
            tokens.elevated
        } else {
            tokens.surface
        })),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(tokens.ink),
        ..container::Style::default()
    }
}

/// Gravidade de uma verificação exibida na ficha de assinatura. Não existe
/// "válida" global: cada dimensão tem a sua cor, e o dot do escudo mostra a pior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckSeverity {
    Passed,
    Attention,
    Failure,
}

pub fn check_color(tokens: Tokens, severity: CheckSeverity) -> Color {
    match severity {
        CheckSeverity::Passed => tokens.ok,
        CheckSeverity::Attention => tokens.warn,
        CheckSeverity::Failure => tokens.danger,
    }
}

/// Cor do dot do escudo: pior verificação vence; `None` sem assinaturas.
pub fn worst_check_color(
    tokens: Tokens,
    severities: impl IntoIterator<Item = CheckSeverity>,
) -> Option<Color> {
    severities.into_iter().max().map(|s| check_color(tokens, s))
}

/// Cartão do menu ⋯: fundo `elevated`, borda `line`, raio 8, sombra leve.
pub fn menu_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.elevated)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, if tokens.is_dark { 0.35 } else { 0.12 }),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        text_color: Some(tokens.ink),
        ..container::Style::default()
    }
}

/// Dica (tooltip) de ícone/botão: fundo `elevated`, borda `line`, raio 6,
/// sombra leve — mesmo vocabulário do `menu_style`, para o rótulo não boiar
/// solto sobre o conteúdo em nenhum tema. `text_color` sempre `ink` (não o
/// padrão do iced), senão o contraste falha no tema claro.
pub fn tooltip_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.elevated)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, if tokens.is_dark { 0.35 } else { 0.12 }),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
        text_color: Some(tokens.ink),
    }
}

/// Item do menu ⋯: texto à esquerda, fundo só no hover.
pub fn menu_item_style(tokens: Tokens) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let background = match status {
            button::Status::Hovered | button::Status::Pressed => tokens.hover,
            _ => Color::TRANSPARENT,
        };
        button::Style {
            background: Some(Background::Color(background)),
            text_color: tokens.ink,
            border: Border {
                radius: 6.0.into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
        }
    }
}

/// Cartão de recente (empty state): fundo `elevated`, raio 8, padding no uso.
pub fn recent_card_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.elevated)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(tokens.ink),
        ..container::Style::default()
    }
}

/// Cartão do Início (pasta ou PDF): fundo `elevated`, borda `line` que vira
/// `accent` no hover — o cartão inteiro é o botão.
pub fn home_card_style(tokens: Tokens) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_, status| {
        let hot = matches!(status, button::Status::Hovered | button::Status::Pressed);
        button::Style {
            background: Some(Background::Color(tokens.elevated)),
            text_color: tokens.ink,
            border: Border {
                color: if hot { tokens.accent } else { tokens.line },
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow::default(),
        }
    }
}

/// Área da prévia no cartão do Início: fundo `surface`, cantos 8px.
pub fn home_preview_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.surface)),
        border: Border {
            radius: 8.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

/// Folha de papel no lugar da prévia que ainda não chegou (ou não abriu).
pub fn home_page_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.page)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 2.0.into(),
        },
        ..container::Style::default()
    }
}

/// Moldura da folha PDF: sempre branca, uma sombra leve.
pub fn page_frame(tokens: &Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    let t = *tokens;
    move |_| container::Style {
        background: Some(Background::Color(t.page)),
        border: Border {
            color: t.line,
            width: 0.0,
            radius: 4.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, if t.is_dark { 0.45 } else { 0.08 }),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        ..container::Style::default()
    }
}

// ── Fatia HUD / overlays (Stitch) ──────────────────────────────────────────

/// Pílula do HUD flutuante (Stitch): fundo `elevated`, borda hairline `line`,
/// raio 99 e sombra difusa — 3º nível de elevação, sobre o canvas.
pub fn hud_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.elevated)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 99.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, if tokens.is_dark { 0.45 } else { 0.12 }),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 32.0,
        },
        text_color: Some(tokens.ink),
        ..container::Style::default()
    }
}

/// Ação primária teal do HUD/overlays (Stitch): fundo `accent`, texto escuro
/// (`bg`, que inverte no tema claro), raio 99.
pub fn hud_primary_style(tokens: Tokens) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let background = match status {
            button::Status::Hovered | button::Status::Pressed => lift(tokens.accent, 0.08),
            button::Status::Disabled => Color {
                a: 0.4,
                ..tokens.accent
            },
            _ => tokens.accent,
        };
        button::Style {
            background: Some(Background::Color(background)),
            text_color: tokens.bg,
            border: Border {
                radius: 99.0.into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
        }
    }
}

/// Ação secundária ghost do HUD/overlays (Stitch): só texto `muted`, que
/// sobe para `ink` com fundo `hover`.
pub fn hud_ghost_style(tokens: Tokens) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let interactive = matches!(status, button::Status::Hovered | button::Status::Pressed);
        button::Style {
            background: Some(Background::Color(if interactive {
                tokens.hover
            } else {
                Color::TRANSPARENT
            })),
            text_color: if interactive {
                tokens.ink
            } else {
                tokens.muted
            },
            border: Border {
                radius: 6.0.into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
        }
    }
}

/// Clareia `color` rumo ao branco — hover do primário (Stitch `#5DC7BF`).
fn lift(color: Color, amount: f32) -> Color {
    Color {
        r: color.r + (1.0 - color.r) * amount,
        g: color.g + (1.0 - color.g) * amount,
        b: color.b + (1.0 - color.b) * amount,
        a: color.a,
    }
}

/// Pílula do documento (toolbar): fundo `surface`, hairline `line`, raio 4.
pub fn bar_doc_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.surface)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: Some(tokens.ink),
        ..container::Style::default()
    }
}

/// Dot da pílula do documento (marcas ainda não salvas): `accent`, redondo.
/// O tamanho vem do `container` no uso.
pub fn bar_dot_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.accent)),
        border: Border {
            radius: 99.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

/// Barra de anotar (Destacar · Sublinhar · Riscar · Nota): fundo `elevated`,
/// borda `accent` a 30%, raio 99.
pub fn bar_mark_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.elevated)),
        border: Border {
            color: Color {
                a: 0.3,
                ..tokens.accent
            },
            width: 1.0,
            radius: 99.0.into(),
        },
        text_color: Some(tokens.ink),
        ..container::Style::default()
    }
}

/// Campo de busca/página dentro da pílula: transparente, sem borda própria
/// (a moldura é do `pill_style`), placeholder `muted`.
pub fn bar_input_style(
    tokens: Tokens,
) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    move |_theme, _status| text_input::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border::default(),
        icon: tokens.muted,
        placeholder: tokens.muted,
        value: tokens.ink,
        selection: Color {
            a: 0.4,
            ..tokens.accent
        },
    }
}

// ── Fatia estado vazio (Stitch) ────────────────────────────────────────────

/// Dropzone do estado vazio (Stitch): fundo `surface`, raio 12. O tracejado
/// é canvas (`EmptyDash` em `view.rs`) — `Border` do iced não tem dash.
pub fn empty_drop_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.surface)),
        border: Border {
            radius: 12.0.into(),
            ..Border::default()
        },
        text_color: Some(tokens.ink),
        ..container::Style::default()
    }
}

/// Pílula de contagem da seção "Documentos recentes": fundo `elevated`,
/// raio 99, texto `muted` (o tamanho vem do `container` no uso).
pub fn empty_badge_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.elevated)),
        border: Border {
            radius: 99.0.into(),
            ..Border::default()
        },
        text_color: Some(tokens.muted),
        ..container::Style::default()
    }
}

/// Palco do lockup Tsuro: fundo escuro fixo, não segue o tema. O lockup
/// foi desenhado para fundo escuro — sem o palco ele some no tema claro.
pub fn logo_stage_style() -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(Color::from_rgb8(0x18, 0x18, 0x18))),
        border: Border {
            radius: 10.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}
// ── Fatia painéis laterais (Stitch §5 Miniaturas) ─────────────────────────

/// Painel lateral (navegação / assinaturas): fundo chrome, hairline `line`,
/// raio 8 — camada 1 do DESIGN (sidebar docked).
pub fn panel_bg_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.chrome)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..container::Style::default()
    }
}

/// Trilho do segmento Miniaturas | Sumário: fundo `surface` (chrome no light),
/// hairline `line`, raio 6.
pub fn panel_seg_track_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(if tokens.is_dark {
            tokens.surface
        } else {
            tokens.chrome
        })),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..container::Style::default()
    }
}

/// Segmento do controle Miniaturas|Sumário: `accent_bg`+`accent` quando ativo,
/// `hover` só no hover; raio 4.
pub fn panel_seg_style(
    tokens: Tokens,
    active: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let background = if active {
            tokens.accent_bg
        } else {
            match status {
                button::Status::Hovered | button::Status::Pressed => tokens.hover,
                _ => Color::TRANSPARENT,
            }
        };
        button::Style {
            background: Some(Background::Color(background)),
            text_color: if active { tokens.accent } else { tokens.ink },
            border: Border {
                radius: 4.0.into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
        }
    }
}

/// Halo da miniatura ativa: 2px `accent` (transparente fora da ativa) — a
/// borda ocupa sempre o mesmo espaço, sem shift de layout.
pub fn panel_thumb_style(
    tokens: Tokens,
    active: bool,
) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: None,
        border: Border {
            color: if active {
                tokens.accent
            } else {
                Color::TRANSPARENT
            },
            width: 2.0,
            radius: 5.0.into(),
        },
        ..container::Style::default()
    }
}

/// Moldura da folha na miniatura: branca, 1px `line`, raio 3 (corte de papel).
pub fn panel_thumb_frame_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.page)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 3.0.into(),
        },
        ..container::Style::default()
    }
}

/// Item do sumário: `accent_bg`+`accent` quando ativo, `hover` no hover;
/// raio 6, texto body-md definido no uso.
pub fn panel_toc_item_style(
    tokens: Tokens,
    active: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let background = if active {
            tokens.accent_bg
        } else {
            match status {
                button::Status::Hovered | button::Status::Pressed => tokens.hover,
                _ => Color::TRANSPARENT,
            }
        };
        button::Style {
            background: Some(Background::Color(background)),
            text_color: if active { tokens.accent } else { tokens.ink },
            border: Border {
                radius: 6.0.into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
        }
    }
}

/// Botão ▸/▾ de expandir/colapsar do sumário: quadrado compacto, `hover` suave.
pub fn panel_fold_style(tokens: Tokens) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let background = match status {
            button::Status::Hovered | button::Status::Pressed => tokens.hover,
            _ => Color::TRANSPARENT,
        };
        button::Style {
            background: Some(Background::Color(background)),
            text_color: tokens.muted,
            border: Border {
                radius: 6.0.into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
        }
    }
}

/// Cartão de assinatura (painel lateral): fundo `elevated`, hairline `line`,
/// raio 8 — mesmo vocabulário do cartão de menu/recente.
pub fn panel_card_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(tokens.elevated)),
        border: Border {
            color: tokens.line,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(tokens.ink),
        ..container::Style::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_is_default_and_page_is_white() {
        let t = Tokens::for_theme(Theme::default(), Lang::Pt);
        assert!(t.is_dark);
        assert_eq!(t.page, Color::WHITE);
        assert_eq!(Tokens::for_theme(Theme::Light, Lang::Pt).page, Color::WHITE);
    }

    #[test]
    fn chrome_tokens_match_issue_12() {
        let t = Tokens::for_theme(Theme::Dark, Lang::Pt);
        assert_eq!(t.bg, hex(0x18, 0x18, 0x18));
        assert_eq!(t.accent, hex(0x4d, 0xb8, 0xb0));
        let l = Tokens::for_theme(Theme::Light, Lang::Pt);
        assert_eq!(l.bg, hex(0xef, 0xef, 0xed));
        assert_eq!(l.elevated, Color::WHITE);
    }

    #[test]
    fn seg_uses_elevated_dark_and_surface_light() {
        let dark = Tokens::for_theme(Theme::Dark, Lang::Pt);
        let style = seg_style(dark)(&iced::Theme::Dark);
        assert!(matches!(style.background, Some(Background::Color(c)) if c == dark.elevated));
        let light = Tokens::for_theme(Theme::Light, Lang::Pt);
        let style = seg_style(light)(&iced::Theme::Light);
        assert!(matches!(style.background, Some(Background::Color(c)) if c == light.surface));
    }

    #[test]
    fn tab_pill_fills_active_accent_and_inactive_per_theme() {
        let dark = Tokens::for_theme(Theme::Dark, Lang::Pt);
        let style = tab_pill_style(dark, false)(&iced::Theme::Dark);
        assert!(matches!(style.background, Some(Background::Color(c)) if c == dark.elevated));
        assert_eq!(style.border.width, 1.0);
        assert_eq!(style.border.color, dark.line);
        let style = tab_pill_style(dark, true)(&iced::Theme::Dark);
        assert!(matches!(style.background, Some(Background::Color(c)) if c == dark.accent_bg));
        let light = Tokens::for_theme(Theme::Light, Lang::Pt);
        let style = tab_pill_style(light, false)(&iced::Theme::Light);
        assert!(matches!(style.background, Some(Background::Color(c)) if c == light.surface));
    }

    #[test]
    fn dot_worst_check_wins_and_empty_has_no_dot() {
        use CheckSeverity as C;
        let t = Tokens::for_theme(Theme::Dark, Lang::Pt);
        assert_eq!(worst_check_color(t, []), None);
        assert_eq!(worst_check_color(t, [C::Passed]), Some(t.ok));
        assert_eq!(
            worst_check_color(t, [C::Passed, C::Attention]),
            Some(t.warn)
        );
        assert_eq!(
            worst_check_color(t, [C::Attention, C::Failure, C::Passed]),
            Some(t.danger)
        );
    }

    #[test]
    fn search_current_differs_from_mark_in_both_themes() {
        for theme in [Theme::Dark, Theme::Light] {
            let t = Tokens::for_theme(theme, Lang::Pt);
            assert_ne!(t.mark_current, t.mark);
            assert_ne!(t.mark_current, t.page);
        }
    }

    #[test]
    fn text_scale_pins_spec_section_2() {
        assert_eq!(
            text_style(TextKind::Body, false),
            TextStyle {
                size: 13.0,
                line_height: 16.0,
                weight: 400
            }
        );
        assert_eq!(text_style(TextKind::Body, true).weight, 600);
        assert_eq!(
            text_style(TextKind::Headline, false),
            TextStyle {
                size: 13.0,
                line_height: 16.0,
                weight: 700
            }
        );
        assert_eq!(text_style(TextKind::Title2, false).size, 17.0);
        assert_eq!(text_style(TextKind::Title2, false).line_height, 22.0);
        assert_eq!(text_style(TextKind::Title2, true).weight, 600);
        assert_eq!(text_style(TextKind::Title3, false).size, 15.0);
        assert_eq!(text_style(TextKind::Callout, false).size, 12.0);
        assert_eq!(text_style(TextKind::Subheadline, false).size, 11.0);
        // Scale floor [HIG].
        assert_eq!(text_style(TextKind::Footnote, false).size, 10.0);
        assert_eq!(text_style(TextKind::Footnote, false).line_height, 13.0);
    }

    #[test]
    fn semantic_roles_pin_spec_section_3() {
        let d = Roles::for_theme(Theme::Dark);
        assert_eq!(d.label, Color::WHITE);
        assert_eq!(d.label_2, hex(0x9a, 0x9a, 0x9a));
        assert_eq!(d.label_3, hex(0x56, 0x56, 0x56));
        assert_eq!(d.fill_1, hex(0x35, 0x35, 0x35));
        assert_eq!(d.fill_3, hex(0x29, 0x29, 0x29));
        assert_eq!(d.separator, hex(0x35, 0x35, 0x35));
        assert_eq!(d.window_bg, hex(0x1e, 0x1e, 0x1e));
        assert_eq!(d.under_page, hex(0x18, 0x18, 0x18));
        assert_eq!(d.accent, hex(0x4d, 0xb8, 0xb0));
        assert_eq!(d.red, hex(0xff, 0x47, 0x47));
        assert_eq!(d.green, hex(0x3b, 0xdb, 0x63));
        assert_eq!(d.orange, hex(0xff, 0x9e, 0x33));
        assert_eq!(d.page, Color::WHITE);

        let l = Roles::for_theme(Theme::Light);
        assert_eq!(l.label, hex(0x26, 0x26, 0x26));
        assert_eq!(l.label_2, hex(0x80, 0x80, 0x80));
        assert_eq!(l.label_3, hex(0xbf, 0xbf, 0xbf));
        assert_eq!(l.fill_1, hex(0xe6, 0xe6, 0xe6));
        assert_eq!(l.fill_3, hex(0xf2, 0xf2, 0xf2));
        assert_eq!(l.separator, hex(0xc6, 0xc6, 0xc8));
        assert_eq!(l.window_bg, Color::WHITE);
        assert_eq!(l.under_page, hex(0xef, 0xef, 0xed));
        assert_eq!(l.accent, hex(0x1f, 0x7d, 0x76));
        assert_eq!(l.red, hex(0xff, 0x38, 0x3c));
        assert_eq!(l.green, hex(0x34, 0xc7, 0x59));
        assert_eq!(l.orange, hex(0xff, 0x8d, 0x28));
        assert_eq!(l.page, Color::WHITE);

        // Brand teal matches the legacy tokens, so the phase-2 cutover is a
        // no-op for the accent.
        assert_eq!(d.accent, Tokens::for_theme(Theme::Dark, Lang::Pt).accent);
        assert_eq!(l.accent, Tokens::for_theme(Theme::Light, Lang::Pt).accent);
    }

    #[test]
    fn spacing_grid_pins_spec_section_4() {
        assert_eq!(
            [
                spacing(Spacing::S4),
                spacing(Spacing::S8),
                spacing(Spacing::S12),
                spacing(Spacing::S16),
                spacing(Spacing::S20),
                spacing(Spacing::S24)
            ],
            [4.0, 8.0, 12.0, 16.0, 20.0, 24.0]
        );
    }

    #[test]
    fn control_heights_pin_spec_section_4() {
        assert_eq!(control_height(ControlHeight::Mini), 20.0);
        // Menu item height = regular [kit].
        assert_eq!(control_height(ControlHeight::Regular), 24.0);
        assert_eq!(control_height(ControlHeight::Alert), 28.0);
        assert_eq!(control_height(ControlHeight::Large), 36.0);
    }

    #[test]
    fn radius_scale_pins_spec_section_4() {
        assert_eq!(radius(Radius::Mini), 4.0);
        assert_eq!(radius(Radius::Button), 6.0);
        assert_eq!(radius(Radius::Item), 8.0);
        assert_eq!(radius(Radius::Tab), 9.0);
        assert_eq!(radius(Radius::Menu), 12.0);
        assert_eq!(radius(Radius::Window), 16.0);
        assert_eq!(radius(Radius::Popover), 20.0);
        assert_eq!(radius(Radius::Sheet), 26.0);
        // Kit ∞, rendered with the 99 convention.
        assert_eq!(radius(Radius::Pill), 99.0);
    }

    #[test]
    fn elevation_presets_pin_spec_section_4() {
        let ring = |w: f32| Some((w, Color::from_rgba(0.0, 0.0, 0.0, 0.2)));
        let shadow = |dy: f32, blur: f32, alpha: f32| {
            Some(Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, alpha),
                offset: Vector::new(0.0, dy),
                blur_radius: blur,
            })
        };
        assert_eq!(
            elevation(Elevation::Window),
            ElevationSpec {
                ring: ring(1.0),
                shadows: [shadow(16.0, 33.0, 0.4), None],
                scrim: None
            }
        );
        assert_eq!(
            elevation(Elevation::Menu),
            ElevationSpec {
                ring: ring(0.5),
                shadows: [shadow(4.0, 24.0, 0.3), None],
                scrim: None
            }
        );
        assert_eq!(
            elevation(Elevation::Popover),
            ElevationSpec {
                ring: ring(0.5),
                shadows: [shadow(18.0, 46.0, 0.25), None],
                scrim: None
            }
        );
        assert_eq!(
            elevation(Elevation::Tooltip),
            ElevationSpec {
                ring: None,
                shadows: [shadow(0.0, 2.0, 0.4), shadow(2.0, 8.0, 0.2)],
                scrim: None
            }
        );
        assert_eq!(
            elevation(Elevation::Scrim),
            ElevationSpec {
                ring: None,
                shadows: [None, None],
                scrim: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.25))
            }
        );
    }
}
