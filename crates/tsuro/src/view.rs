use iced::advanced::widget::operation::{Focusable, Operation};
use iced::advanced::widget::{operate, Id as WidgetId};
use iced::widget::canvas::event::{Event as CanvasEvent, Status as CanvasStatus};
use iced::widget::canvas::{Frame, Geometry, LineDash, Path, Program, Stroke, Style};
use iced::widget::{
    button, column, container, image, mouse_area, pick_list, row, scrollable, stack, text,
    text_editor, text_input, tooltip, Canvas, Space,
};
use iced::{font, Font};
use iced::{mouse, Alignment, Background, Border, Color, Element, Length, Padding, Point, Task};
use iced::{Rectangle, Size};
use tsuro_sign::{SignatureInfo, SignatureStatus};

use crate::browse::{EmptyState, FsEntry, Thumb};
use crate::i18n::Lang;
use crate::kiri::{self, CheckSeverity, Theme, Tokens};
use crate::page::{MediaBox, PageNo};
use crate::prefs::ThemePref;
use crate::print::{PrintOrientation, MAX_COPIES};
use crate::search::Search;
#[cfg(target_os = "macos")]
use crate::session::PermissionsState;
use crate::session::{
    display_pt, display_rect, marker_side, page_pt_at, AnnotKind, AnnotationRow, Message, NavCmd,
    NoteDraft, OpenSource, PaletteItem, PaletteState, PanelKey, PanelTab, PrintDialog, RangeMode,
    Ready, RecoveryState, Session, SettingsPanel, Tabs, UpdateState, ViewMode, Zoom, ZoomFactor,
    ANNOT_ROW, DOC_GAP, DOC_PAD_BOTTOM, DOC_PAD_TOP, DOC_PAD_X, PAGES_PANEL_W, SIG_PANEL_W,
    THUMB_ROW,
};

/// Altura do chrome Kiri: toolbar 36px + progresso 2px + respiro.
pub const CHROME_HEIGHT: f32 = 46.0;

/// Altura da faixa de abas (issue #40), reservada só com 2+ documentos.
pub const TAB_STRIP_HEIGHT: f32 = 30.0;

/// Papel do post-it (issue #30): amarelo fixo, como o marcador nota do
/// canvas — papel não segue o tema e a tinta escura lê nos dois.
const STICKY: Color = Color::from_rgb(0.99, 0.80, 0.20);
/// Tinta sobre o papel (marcador, rótulo, texto e botões do post-it).
const STICKY_INK: Color = Color::from_rgb(0.42, 0.30, 0.02);
/// Borda do papel (um tom abaixo do fundo).
const STICKY_LINE: Color = Color::from_rgb(0.80, 0.62, 0.10);
/// Post-it: largura, altura do cartão (posição presa por ela) e altura do
/// campo de texto. A altura do cartão é a do conteúdo montado abaixo
/// (rótulo + editor + rodapé + paddings), só para prender a posição.
const POSTIT_W: f32 = 320.0;
const POSTIT_H: f32 = 210.0;
const POSTIT_EDITOR_H: f32 = 112.0;

pub fn pages_scroll_id() -> scrollable::Id {
    scrollable::Id::new("tsuro-pages")
}

pub fn annotations_scroll_id() -> scrollable::Id {
    scrollable::Id::new("tsuro-annotations")
}

pub fn doc_scroll_id() -> scrollable::Id {
    scrollable::Id::new("tsuro-doc")
}

/// Campo de busca da toolbar (Ctrl+F foca aqui).
pub fn search_input_id() -> text_input::Id {
    text_input::Id::new("tsuro-search")
}

pub fn page_input_id() -> text_input::Id {
    text_input::Id::new("tsuro-page")
}

pub fn palette_input_id() -> text_input::Id {
    text_input::Id::new("tsuro-palette")
}

pub fn palette_scroll_id() -> scrollable::Id {
    scrollable::Id::new("tsuro-palette-list")
}

/// Altura máxima da lista da paleta (~10 linhas); além disso, scroll.
const PALETTE_LIST_MAX_H: f32 = 420.0;

pub fn chrome(session: &Session, theme: Theme, lang: Lang) -> Element<'_, Message> {
    let t = Tokens::for_theme(theme, lang);
    let body: Element<'_, Message> = match session {
        Session::Empty(empty) => empty_browser(empty, false, t),
        Session::Loading { source, phase, .. } => opening(source, *phase, t),
        Session::Failed {
            source, message, ..
        } => failed(source, message, t),
        Session::Ready(ready) => ready_body(ready, t),
    };

    // Toolbar 36px + progresso 2px + respiro 8px = `CHROME_HEIGHT` (46px).
    let mut col = column![topbar(session, t)];
    if let Session::Ready(tabs) = session {
        // Progresso de leitura é do documento: some com o Início na tela.
        if tabs.home().is_none() {
            col = col.push(progress(tabs, t));
        }
        // Faixa de abas (issue #40): com um documento só a janela é a de
        // sempre — sem faixa nenhuma. Com o Início por cima, a faixa é o
        // caminho de volta (mesmo com uma aba).
        if tabs.len() > 1 || tabs.home().is_some() {
            col = col.push(tab_strip(tabs, t));
        }
    }
    col = col.push(body);
    let main: Element<'_, Message> = container(
        col.spacing(0)
            .padding(4)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_| container::Style {
        background: Some(Background::Color(t.bg)),
        text_color: Some(t.ink),
        ..container::Style::default()
    })
    .into();
    let empty = || -> Element<'_, Message> { Space::new(Length::Shrink, Length::Shrink).into() };
    // Tela vazia: `stack![janela, camada]` sempre, pelo mesmo motivo abaixo
    // (abrir a paleta não pode zerar a rolagem da grade).
    if let Session::Empty(home) = session {
        let overlay = match home.palette.as_ref() {
            Some(palette) => palette_layer(palette, t),
            None if home.about_open => about_layer(t),
            None if session.settings().open => settings_layer(session.settings(), t),
            None => empty(),
        };
        return stack![main, overlay].into();
    }
    let Session::Ready(tabs) = session else {
        return main;
    };
    // Com documento aberto a raiz é sempre `stack![janela, HUD, camada]`: o
    // que não está na tela vira um espaço vazio, sem sair da árvore. Trocar
    // a forma da raiz recriava o estado de todos os widgets — a rolagem do
    // contínuo voltava ao topo ao abrir um menu ou selecionar texto, e o
    // campo de busca perdia o foco a cada tique do HUD.
    // HUD flutuante (Stitch): pílula bottom-center sobre o canvas, sob os
    // modais — que escurecem por cima e capturam tudo. Some com o mouse
    // parado (ver `HudState`).
    let hud_layer = if tabs.home().is_none() && tabs.hud.visible() {
        hud(tabs, t)
    } else {
        empty()
    };
    // Com o Início por cima, a paleta na tela é a dele.
    let palette = match tabs.home() {
        Some(home) => home.palette.as_ref(),
        None => tabs.palette(),
    };
    let overlay = if tabs.close_prompt() {
        // Fechar com marcações sujas fica por cima dos outros modais.
        close_prompt_layer(t)
    } else if tabs.home().is_none() && tabs.recovery_prompt() {
        // Cartão de recuperação (#128): aparece na abertura, antes da paleta.
        recovery_layer(tabs.active(), t)
    } else if let Some(palette) = palette {
        palette_layer(palette, t)
    } else if let Some(dialog) = tabs.print_dialog.as_ref() {
        // Modal de impressão captura tudo; menu ⋯ nunca abre junto (fecha ao abrir).
        print_layer(tabs, dialog, t)
    } else if tabs.note_draft.is_some() {
        // Post-it (issue #30): editor ancorado no trecho da nota. Sem modal
        // centrado — só o fundo escurece e cancela no clique (como o de
        // impressão), que também captura os cliques de fora.
        note_layer(tabs, t)
    } else if tabs.save_warning {
        // Aviso de documento assinado (⋯ → Salvar cópia): captura tudo.
        save_warning_layer(t)
    } else if tabs.about_open {
        // Sobre (Ajuda): cartão de versão; fundo e Esc fecham.
        about_layer(t)
    } else if tabs.settings.open {
        // Configurações (Ajuda): abre por cima do Sobre; fundo e Esc fecham.
        settings_layer(&tabs.settings, t)
    } else if tabs.overflow_open {
        // Overlay visual: só os botões capturam clique, o resto atravessa.
        overflow_layer(tabs, t)
    } else if tabs.home().is_none() && tabs.selection_bar_pos().is_some() {
        // Mini-toolbar da seleção (issue #51): sem dim nem captura — só os
        // quatro botões recebem clique, o resto atravessa para a folha. Os
        // diálogos acima têm precedência; `Annotate` limpa a seleção e a
        // barra some sozinha.
        selection_bar_layer(tabs, t)
    } else {
        empty()
    };
    stack![main, hud_layer, overlay].into()
}

/// Botão-ícone da pílula do HUD: Ori + tooltip, no tamanho dos segmentos.
fn hud_icon(
    file: &str,
    label: &'static str,
    t: Tokens,
    message: Message,
) -> Element<'static, Message> {
    tip(
        t,
        control_seg(t, button(seg_icon(file))).on_press(message),
        label,
    )
}

/// HUD flutuante (Stitch §Floating Action Pill): pílula 36px centrada a 24px do
/// rodapé, sobre o canvas. Só os botões capturam clique — o resto atravessa.
fn hud(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    let n = ready.page_count().max(1);
    let page = ready.visible.index() + 1;
    let current = ready.zoom_step_factor();
    let out = Zoom::Manual(ZoomFactor::new(current / 1.1));
    let into = Zoom::Manual(ZoomFactor::new(current * 1.1));
    let mono = |s: String, color: Color| text(s).size(12).font(iced::Font::MONOSPACE).color(color);
    let mut pill = row![
        hud_icon(
            "chevron-left",
            t.lang.pick("Página anterior", "Previous page"),
            t,
            Message::Nav(NavCmd::Previous)
        ),
        mono(page.to_string(), t.ink),
        mono("/".to_string(), t.muted),
        mono(n.to_string(), t.muted),
        hud_icon(
            "chevron-right",
            t.lang.pick("Próxima página", "Next page"),
            t,
            Message::Nav(NavCmd::Next)
        ),
        kiri::vsep(t),
        hud_icon(
            "fit-width",
            t.lang.pick("Ajustar à largura", "Fit width"),
            t,
            Message::SetZoom(Zoom::Width)
        ),
        hud_icon(
            "minus",
            t.lang.pick("Diminuir zoom (-)", "Zoom out (-)"),
            t,
            Message::SetZoom(out)
        ),
        mono(format!("{}%", (current * 100.0).round() as i32), t.ink),
        hud_icon(
            "plus",
            t.lang.pick("Aumentar zoom (+)", "Zoom in (+)"),
            t,
            Message::SetZoom(into)
        ),
    ]
    .spacing(2)
    .align_y(Alignment::Center);
    // Modo Anotação: só com seleção viva ou marcações na sessão.
    if ready.selection_plain_text().is_some() || !ready.annotations.is_empty() {
        pill = pill.push(kiri::vsep(t)).push(
            row![
                container(Space::with_width(Length::Fixed(7.0)))
                    .width(Length::Fixed(7.0))
                    .height(Length::Fixed(7.0))
                    .style(move |_| container::Style {
                        background: Some(Background::Color(t.accent)),
                        border: Border {
                            radius: 99.0.into(),
                            ..Border::default()
                        },
                        ..container::Style::default()
                    }),
                text(t.lang.pick("Modo Anotação", "Annotation mode"))
                    .size(11)
                    .color(t.accent),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        );
    }
    // Cursor sobre a pílula a segura na tela (não some embaixo da mão).
    container(
        mouse_area(
            container(pill)
                .height(Length::Fixed(28.0))
                .align_y(Alignment::Center)
                .padding(Padding::from([4, 6]))
                .style(kiri::hud_style(t)),
        )
        .on_enter(Message::HudHover(true))
        .on_exit(Message::HudHover(false)),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::End)
    .padding(Padding {
        top: 0.0,
        right: 0.0,
        bottom: 24.0,
        left: 0.0,
    })
    .into()
}

fn control_style(
    t: Tokens,
    active: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    kiri::ibtn_style(t, active)
}

fn control_active(
    t: Tokens,
    btn: iced::widget::button::Button<'_, Message>,
    active: bool,
) -> iced::widget::button::Button<'_, Message> {
    btn.padding(Padding::from([9, 10]))
        .style(control_style(t, active))
}

/// Ícone dos botões da barra: 20px + padding 8 = os 36px da barra.
const BAR_ICON: f32 = 20.0;
/// Ícone dos segmentos (◀▶, −/+, marcar, HUD): 18px + padding 4 = 26px, a
/// mesma altura de antes (16 + 5).
const SEG_ICON: f32 = 18.0;

/// Botão-ícone 36×36 da barra (casa, abrir, painéis, ⋯).
fn bar_button(
    t: Tokens,
    file: &str,
    active: bool,
) -> iced::widget::button::Button<'static, Message> {
    button(kiri::ori_icon(file, BAR_ICON))
        .padding(8)
        .style(control_style(t, active))
}

fn seg_icon(file: &str) -> Element<'static, Message> {
    kiri::ori_icon(file, SEG_ICON)
}

/// Botão-ícone 26px de altura para dentro de segmentos (`kiri::seg_style` +
/// padding 2 = 30px).
fn control_seg(
    t: Tokens,
    btn: iced::widget::button::Button<'_, Message>,
) -> iced::widget::button::Button<'_, Message> {
    btn.padding(Padding::from([4, 5]))
        .style(control_style(t, false))
}

/// Dica (tooltip) de ícone/botão: fundo `kiri::tooltip_style`, para o rótulo
/// não boiar solto sobre o conteúdo (issue reportada nos dois temas).
/// `snap_within_viewport` já é `true` por padrão no `Tooltip` do iced 0.13 —
/// a pílula do HUD, perto do rodapé, não é cortada pela borda da janela.
fn tip<'a>(
    t: Tokens,
    content: impl Into<Element<'a, Message>>,
    label: &'static str,
) -> Element<'a, Message> {
    tooltip::Tooltip::new(
        content,
        container(text(label).size(13))
            .padding(Padding::from([4, 8]))
            .style(kiri::tooltip_style(t)),
        tooltip::Position::Bottom,
    )
    .gap(6)
    .into()
}

/// Dica com texto dinâmico (nome/caminho do documento): mesmo estilo da `tip`.
fn tip_owned<'a>(
    t: Tokens,
    content: impl Into<Element<'a, Message>>,
    label: String,
) -> Element<'a, Message> {
    tooltip::Tooltip::new(
        content,
        container(text(label).size(13))
            .padding(Padding::from([4, 8]))
            .style(kiri::tooltip_style(t)),
        tooltip::Position::Bottom,
    )
    .gap(6)
    .into()
}

/// Texto da dica de identidade: nome inteiro + caminho (abas e pílula).
fn identity_tip(path: &std::path::Path) -> String {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    format!("{}\n{}", name, path.display())
}

fn open_button(t: Tokens) -> Element<'static, Message> {
    tip(
        t,
        bar_button(t, "folder", false).on_press(Message::PickFile),
        t.lang.pick("Abrir PDF", "Open PDF"),
    )
}

/// Casinha: com abas abertas mostra o Início por cima delas (`GoHome`);
/// carregando/erro não têm aba a preservar e fecham (`Close`).
fn home_button(t: Tokens, message: Message) -> Element<'static, Message> {
    tip(
        t,
        bar_button(t, "home", false).on_press(message),
        t.lang.pick("Início", "Home"),
    )
}

/// Aviso de versão nova na barra, fora do menu: abre Configurações (onde
/// está Atualizar agora) e, com a versão já instalada, reinicia.
fn update_badge(t: Tokens, update: &UpdateState) -> Option<Element<'static, Message>> {
    let (label, message) = match update {
        UpdateState::Available(tag) => (
            format!("{} {tag}", t.lang.pick("Nova versão", "New version")),
            Message::ToggleSettings,
        ),
        UpdateState::Installed(_) => (
            t.lang
                .pick("Reiniciar para atualizar", "Restart to update")
                .to_string(),
            Message::RestartToUpdate,
        ),
        _ => return None,
    };
    Some(
        container(
            button(text(label).size(12).color(t.accent))
                .padding(Padding::from([2, 10]))
                .style(kiri::menu_item_style(t))
                .on_press(message),
        )
        .style(kiri::bar_mark_style(t))
        .into(),
    )
}

fn settings_button(t: Tokens, open: bool) -> Element<'static, Message> {
    tip(
        t,
        bar_button(t, "settings", open).on_press(Message::ToggleSettings),
        t.lang.pick("Configurações", "Settings"),
    )
}

/// Moldura da toolbar Kiri: 36px, fundo `chrome`, respiro horizontal 8px.
fn toolbar_frame(t: Tokens, content: Element<'_, Message>) -> Element<'_, Message> {
    container(content)
        .width(Length::Fill)
        .height(Length::Fixed(36.0))
        .padding(Padding::from([0, 8]))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.chrome)),
            ..container::Style::default()
        })
        .into()
}

/// Nome do arquivo com elipse no meio (`Contrato_Locacao_..._2024.pdf`).
fn middle_truncate(name: &str, max: usize) -> String {
    let count = name.chars().count();
    if count <= max {
        return name.to_string();
    }
    // A elipse ocupa 1; o resto divide cabeça e cauda.
    let head = (max - 1) / 2;
    let tail = max - 1 - head;
    let head_end = name.char_indices().nth(head).map_or(name.len(), |(i, _)| i);
    let tail_start = name
        .char_indices()
        .nth(count - tail)
        .map_or(name.len(), |(i, _)| i);
    format!("{}…{}", &name[..head_end], &name[tail_start..])
}

/// Contador da busca (#43): `None` sem consulta, `"0"` sem hits, total puro
/// digitando (`"12"`) e `"3 de 12"` navegando.
/// `scanning`: ainda há páginas sem texto na fila da busca — o "…" avisa
/// que o total pode crescer.
fn search_count_text(search: &Search, scanning: bool, lang: Lang) -> Option<String> {
    if search.query().is_empty() {
        return None;
    }
    let total = search.hits().len();
    let count = match search.current() {
        Some(i) if total > 0 => format!("{} {} {}", i + 1, lang.pick("de", "of"), total),
        _ => total.to_string(),
    };
    Some(if scanning {
        format!("{count}…")
    } else {
        count
    })
}

/// Navegação da busca na pílula: contador + ‹ › (só com consulta; botões só
/// com hits). Vazio encolhe para não empurrar a página.
fn search_nav<'a>(ready: &'a Ready, t: Tokens) -> Element<'a, Message> {
    if ready.search.query().is_empty() {
        return Space::with_width(Length::Shrink).into();
    }
    let mut nav = row![].spacing(2).align_y(Alignment::Center);
    if let Some(count) = search_count_text(&ready.search, ready.search_scanning(), t.lang) {
        nav = nav.push(text(count).size(11).color(t.muted));
    }
    if !ready.search.hits().is_empty() {
        nav = nav.push(tip(
            t,
            control_seg(
                t,
                button(seg_icon("chevron-left")).on_press(Message::SearchPrev),
            ),
            t.lang
                .pick("Anterior (Shift+Enter)", "Previous (Shift+Enter)"),
        ));
        nav = nav.push(tip(
            t,
            control_seg(
                t,
                button(seg_icon("chevron-right")).on_press(Message::SearchNext),
            ),
            t.lang.pick("Próximo (Enter)", "Next (Enter)"),
        ));
    }
    nav.into()
}

/// Barra única Kiri (`Session::Ready`): abrir │ pílula do documento │ busca e
/// página │ marcar │ zoom │ painéis │ ⋯. Uma linha só (`CHROME_HEIGHT`).
fn topbar(session: &Session, t: Tokens) -> Element<'_, Message> {
    // Com o Início por cima das abas, a barra é a da tela inicial.
    let doc = match session {
        Session::Ready(ready) if ready.home().is_none() => Some(ready),
        _ => None,
    };
    if let Some(ready) = doc {
        let n = ready.page_count().max(1);
        // Pílula do documento: dot (marcas da sessão ainda não salvas) + nome do
        // arquivo com elipse no meio. Nada de tamanho/data: só o que o estado tem.
        let name = ready
            .source
            .path()
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let mut doc_row = row![].spacing(6).align_y(Alignment::Center);
        if ready.marks_dirty() {
            doc_row = doc_row.push(
                container(Space::with_width(Length::Fixed(6.0)))
                    .width(Length::Fixed(6.0))
                    .height(Length::Fixed(6.0))
                    .style(kiri::bar_dot_style(t)),
            );
        }
        let doc_pill = tip_owned(
            t,
            container(doc_row.push(text(middle_truncate(&name, 24)).size(13).color(t.ink)))
                .padding(Padding {
                    top: 4.0,
                    right: 8.0,
                    bottom: 4.0,
                    left: 8.0,
                })
                .style(kiri::bar_doc_style(t)),
            identity_tip(ready.source.path()),
        );
        let pill = container(
            row![
                tip(
                    t,
                    kiri::ori_small!("search"),
                    if cfg!(target_os = "macos") {
                        t.lang
                            .pick("Buscar no documento (Cmd+F)", "Find in document (Cmd+F)")
                    } else {
                        t.lang
                            .pick("Buscar no documento (Ctrl+F)", "Find in document (Ctrl+F)")
                    },
                ),
                text_input(
                    t.lang.pick("Buscar no documento...", "Find in document..."),
                    ready.search.query()
                )
                .id(search_input_id())
                .on_input(Message::SearchChanged)
                .on_submit(Message::SearchSubmit)
                .style(kiri::bar_input_style(t))
                .padding([2, 4])
                .size(12)
                .width(Length::Fixed(120.0)),
                search_nav(ready, t),
                kiri::vsep(t),
                row![
                    tip(
                        t,
                        text_input(t.lang.pick("Página", "Page"), ready.page_input())
                            .id(page_input_id())
                            .on_input(Message::PageInput)
                            .on_submit(Message::PageSubmit)
                            .style(kiri::bar_input_style(t))
                            .font(iced::Font::MONOSPACE)
                            .size(12)
                            .padding([2, 4])
                            .width(Length::Fixed(30.0)),
                        t.lang.pick(
                            "Ir para página (Enter confirma)",
                            "Go to page (Enter confirms)"
                        ),
                    ),
                    text(format!("/{n}"))
                        .size(12)
                        .font(iced::Font::MONOSPACE)
                        .color(t.muted),
                ]
                .spacing(2)
                .align_y(Alignment::Center),
                container(
                    row![
                        tip(
                            t,
                            control_seg(
                                t,
                                button(seg_icon("chevron-left"))
                                    .on_press(Message::Nav(NavCmd::Previous))
                            ),
                            t.lang.pick("Página anterior", "Previous page")
                        ),
                        tip(
                            t,
                            control_seg(
                                t,
                                button(seg_icon("chevron-right"))
                                    .on_press(Message::Nav(NavCmd::Next))
                            ),
                            t.lang.pick("Próxima página", "Next page")
                        ),
                    ]
                    .spacing(0)
                    .align_y(Alignment::Center)
                )
                .padding(2)
                .style(kiri::seg_style(t)),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .padding(Padding {
            top: 2.0,
            right: 6.0,
            bottom: 2.0,
            left: 10.0,
        })
        .style(kiri::pill_style(t))
        .max_width(520.0);
        let current = ready.zoom_step_factor();
        let out = Zoom::Manual(ZoomFactor::new(current / 1.1));
        let into = Zoom::Manual(ZoomFactor::new(current * 1.1));

        let zoom_seg = container(
            row![
                tip(
                    t,
                    control_seg(t, button(seg_icon("minus")).on_press(Message::SetZoom(out))),
                    t.lang.pick("Diminuir zoom (-)", "Zoom out (-)")
                ),
                tip(
                    t,
                    container(
                        text(format!("{}%", (current * 100.0).round() as i32))
                            .size(12)
                            .font(iced::Font::MONOSPACE)
                            .color(t.muted),
                    )
                    .padding(Padding::from([0, 4])),
                    t.lang.pick("Zoom atual", "Current zoom"),
                ),
                tip(
                    t,
                    control_seg(t, button(seg_icon("plus")).on_press(Message::SetZoom(into))),
                    t.lang.pick("Aumentar zoom (+)", "Zoom in (+)")
                ),
                tip(
                    t,
                    control_seg(
                        t,
                        button(seg_icon("fit-width")).on_press(Message::SetZoom(Zoom::Width))
                    ),
                    t.lang.pick("Ajustar à largura", "Fit width")
                ),
            ]
            .spacing(0)
            .align_y(Alignment::Center),
        )
        .padding(2)
        .style(kiri::seg_style(t));

        // Copiar/fechar/fit-page vivem no menu ⋯ (`overflow_menu`).
        let shield =
            bar_button(t, "shield", ready.signatures_open).on_press(Message::ToggleSignatures);
        let dot = kiri::worst_check_color(
            t,
            ready
                .signatures
                .signatures
                .iter()
                .flat_map(|s| sig_rows(s, t.lang).map(|r| r.severity)),
        );
        let shield_el: Element<'_, Message> = match dot {
            Some(dot) => tip(
                t,
                stack![
                    shield,
                    container(
                        container(Space::with_width(Length::Fixed(7.0)))
                            .width(Length::Fixed(7.0))
                            .height(Length::Fixed(7.0))
                            .style(move |_| container::Style {
                                background: Some(Background::Color(dot)),
                                border: Border {
                                    color: t.chrome,
                                    width: 2.0,
                                    radius: 99.0.into(),
                                },
                                ..container::Style::default()
                            })
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Alignment::End)
                    .align_y(Alignment::Start)
                    .padding(Padding {
                        top: 6.0,
                        right: 6.0,
                        bottom: 0.0,
                        left: 0.0,
                    })
                ],
                t.lang.pick("Assinaturas", "Signatures"),
            ),
            None => tip(t, shield, t.lang.pick("Assinaturas", "Signatures")),
        };
        let mut right = row![
            tip(
                t,
                bar_button(t, "pages", ready.pages_open).on_press(Message::TogglePages),
                t.lang.pick("Páginas", "Pages")
            ),
            shield_el,
        ]
        .spacing(4)
        .align_y(Alignment::Center);
        if let Some(badge) = update_badge(t, &session.settings().update) {
            right = right.push(badge);
        }
        right = right.push(tip(
            t,
            bar_button(t, "more", ready.overflow_open).on_press(Message::ToggleOverflow),
            t.lang.pick("Mais opções", "More options"),
        ));

        // Barra de anotar na cara (seleção viva): ⋯ é fuga, não casa. Os atalhos
        // H/U/S/N já vivem em `keyboard_message`; aqui só o rótulo da tecla.
        let mut bar = row![
            home_button(t, Message::GoHome),
            open_button(t),
            kiri::vsep(t),
            doc_pill,
            Space::with_width(Length::Fill),
            pill,
            Space::with_width(Length::Fill),
        ]
        .spacing(6)
        .align_y(Alignment::Center);
        if ready.selection_plain_text().is_some() {
            let hint = |key: &'static str| {
                text(key)
                    .size(10)
                    .font(iced::Font::MONOSPACE)
                    .color(t.muted)
            };
            // Rótulo só com janela larga (Stitch: `hidden xl:inline`, 1280px);
            // estreita fica ícone + tecla, que é o que cabe na mesma linha.
            let wide = ready.viewport().width >= 1280.0;
            let tool = |icon: Element<'static, Message>, label: &'static str, key: &'static str| {
                let mut content = row![icon].spacing(4).align_y(Alignment::Center);
                if wide {
                    content = content.push(text(label).size(12));
                }
                content.push(hint(key))
            };
            let mark_seg = container(
                row![
                    tip(
                        t,
                        control_seg(
                            t,
                            button(tool(
                                seg_icon("highlighter"),
                                t.lang.pick("Destacar", "Highlight"),
                                "H"
                            ))
                            .on_press(Message::Annotate(AnnotKind::Highlight))
                        ),
                        t.lang.pick("Destacar (H)", "Highlight (H)")
                    ),
                    tip(
                        t,
                        control_seg(
                            t,
                            button(tool(
                                seg_icon("underline"),
                                t.lang.pick("Sublinhar", "Underline"),
                                "U"
                            ))
                            .on_press(Message::Annotate(AnnotKind::Underline))
                        ),
                        t.lang.pick("Sublinhar (U)", "Underline (U)")
                    ),
                    tip(
                        t,
                        control_seg(
                            t,
                            button(tool(
                                seg_icon("strike"),
                                t.lang.pick("Riscar", "Strikeout"),
                                "S"
                            ))
                            .on_press(Message::Annotate(AnnotKind::Strikeout))
                        ),
                        t.lang.pick("Riscar (S)", "Strikeout (S)")
                    ),
                    tip(
                        t,
                        control_seg(
                            t,
                            button(tool(seg_icon("note"), t.lang.pick("Nota", "Note"), "N"))
                                .on_press(Message::Annotate(AnnotKind::Note))
                        ),
                        t.lang.pick("Nota (N)", "Note (N)")
                    ),
                ]
                .spacing(0)
                .align_y(Alignment::Center),
            )
            .padding(2)
            .style(kiri::bar_mark_style(t));
            bar = bar.push(mark_seg).push(kiri::vsep(t));
        }
        bar = bar.push(zoom_seg).push(kiri::vsep(t)).push(right);

        return toolbar_frame(t, bar.into());
    }

    // Tela inicial (vazia ou por cima das abas) não precisa de home;
    // erro/carregando usam para voltar.
    let mut items: Vec<Element<'_, Message>> = Vec::new();
    if matches!(session, Session::Loading { .. } | Session::Failed { .. }) {
        items.push(home_button(t, Message::Close));
    }
    items.push(open_button(t));
    if matches!(session, Session::Empty(_) | Session::Ready(_)) {
        items.push(Space::with_width(Length::Fill).into());
        if let Some(badge) = update_badge(t, &session.settings().update) {
            items.push(badge);
        }
        items.push(settings_button(t, session.settings().open));
    }
    toolbar_frame(t, row(items).spacing(4).align_y(Alignment::Center).into())
}

/// Camada do menu ⋯: fundo fecha ao clicar, menu no canto.
fn overflow_layer(tabs: &Tabs, t: Tokens) -> Element<'_, Message> {
    let dim = container(Space::with_width(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
            ..container::Style::default()
        });
    // Com 2+ abas a faixa soma TAB_STRIP_HEIGHT à topbar (44px).
    let top = if tabs.len() > 1 {
        44.0 + TAB_STRIP_HEIGHT
    } else {
        44.0
    };
    let card = container(overflow_menu(tabs, tabs.settings.theme_pref, t))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::End)
        .align_y(Alignment::Start)
        .padding(Padding {
            top,
            right: 8.0,
            bottom: 0.0,
            left: 0.0,
        });
    stack![mouse_area(dim).on_press(Message::ToggleOverflow), card].into()
}

/// Menu ⋯ (#42): grupos Arquivo/Editar/Ver/Ir/Ajuda com dicas de atalho.
/// Recentes na mesma lista da tela vazia; Sobre abre o cartão de versão.
fn overflow_menu(ready: &Ready, pref: ThemePref, t: Tokens) -> Element<'_, Message> {
    let mut items = column![].spacing(2).width(Length::Fill);
    items = items.push(section_title(t.lang.pick("Arquivo", "File"), t));
    items = items.push(menu_item(
        t,
        "folder-open",
        t.lang.pick("Abrir…", "Open…"),
        Message::PickFile,
        false,
    ));
    items = items.push(menu_item(
        t,
        "folder",
        if ready.recents_expanded {
            t.lang.pick("Recentes ▾", "Recent ▾")
        } else {
            t.lang.pick("Recentes ▸", "Recent ▸")
        },
        Message::ToggleRecents,
        false,
    ));
    if ready.recents_expanded {
        let recents: Vec<_> = ready.recents().iter().take(5).collect();
        if recents.is_empty() {
            items = items.push(menu_disabled(
                t,
                t.lang.pick("Nenhum arquivo recente", "No recent files"),
            ));
        } else {
            for path in recents {
                items = items.push(menu_recent(
                    t,
                    recent_label(path),
                    Message::OpenRecent(path.clone()),
                ));
            }
        }
    }
    items = items.push(menu_item(
        t,
        "print",
        t.lang.pick("Imprimir", "Print"),
        Message::OpenPrintDialog,
        false,
    ));
    if !ready.annotations.is_empty() {
        items = items.push(menu_item(
            t,
            "save",
            t.lang
                .pick("Salvar cópia com marcações…", "Save copy with marks…"),
            Message::SaveCopyRequested,
            false,
        ));
    }
    items = items.push(menu_item(
        t,
        "x",
        t.lang.pick("Fechar documento", "Close document"),
        Message::CloseTabActive,
        false,
    ));
    let can_edit = ready.selection_plain_text().is_some()
        || !ready.annotations.is_empty()
        || ready.can_annot_undo()
        || ready.can_annot_redo()
        || ready.selected_annot().is_some();
    if can_edit {
        items = items.push(section_title(t.lang.pick("Editar", "Edit"), t));
        if ready.selection_plain_text().is_some() {
            items = items.push(menu_item(
                t,
                "copy",
                t.lang.pick("Copiar seleção", "Copy selection"),
                Message::CopySelection,
                false,
            ));
        }
        if !ready.annotations.is_empty() {
            items = items.push(menu_item(
                t,
                "copy",
                t.lang.pick(
                    "Copiar destaques como Markdown",
                    "Copy highlights as Markdown",
                ),
                Message::CopyAnnotations,
                false,
            ));
            items = items.push(menu_item(
                t,
                "file-text",
                t.lang.pick("Exportar marcações…", "Export annotations…"),
                Message::ExportAnnotations,
                false,
            ));
        }
        if ready.selected_annot().is_some() {
            items = items.push(menu_item(
                t,
                "x",
                t.lang.pick("Apagar marcação", "Delete mark"),
                Message::DeleteSelectedAnnot,
                false,
            ));
        }
        if ready.can_annot_undo() {
            items = items.push(menu_item(
                t,
                "undo",
                t.lang.pick("Desfazer marcação", "Undo mark"),
                Message::AnnotUndo,
                false,
            ));
        }
        if ready.can_annot_redo() {
            items = items.push(menu_item(
                t,
                "redo",
                t.lang.pick("Refazer marcação", "Redo mark"),
                Message::AnnotRedo,
                false,
            ));
        }
    }
    items = items.push(section_title(t.lang.pick("Ver", "View"), t));
    items = items.push(menu_item(
        t,
        "fit-page",
        t.lang.pick("Ajustar página inteira", "Fit page"),
        Message::SetZoom(Zoom::Page),
        false,
    ));
    items = items.push(menu_item(
        t,
        "rotate",
        t.lang.pick("Girar vista (90°)", "Rotate view (90°)"),
        Message::RotateView,
        false,
    ));
    let single = ready.view_mode == ViewMode::Single;
    items = items.push(menu_item(
        t,
        "page-single",
        t.lang.pick("Página única", "Single page"),
        Message::SetViewMode(ViewMode::Single),
        single,
    ));
    items = items.push(menu_item(
        t,
        "continuous",
        t.lang.pick("Rolagem contínua", "Continuous scroll"),
        Message::SetViewMode(ViewMode::Continuous),
        !single,
    ));
    items = items.push(
        row![
            menu_theme_button(
                t,
                t.lang.pick("Sistema", "System"),
                ThemePref::System,
                pref == ThemePref::System
            ),
            menu_theme_button(
                t,
                t.lang.pick("Escuro", "Dark"),
                ThemePref::Dark,
                pref == ThemePref::Dark
            ),
            menu_theme_button(
                t,
                t.lang.pick("Claro", "Light"),
                ThemePref::Light,
                pref == ThemePref::Light
            ),
        ]
        .spacing(4),
    );
    if ready.can_history_back() || ready.can_history_forward() {
        items = items.push(section_title(t.lang.pick("Ir", "Go"), t));
        if ready.can_history_back() {
            items = items.push(menu_item(
                t,
                "chevron-left",
                t.lang.pick("Voltar", "Back"),
                Message::HistoryBack,
                false,
            ));
        }
        if ready.can_history_forward() {
            items = items.push(menu_item(
                t,
                "chevron-right",
                t.lang.pick("Avançar", "Forward"),
                Message::HistoryForward,
                false,
            ));
        }
    }
    items = items.push(section_title(t.lang.pick("Ajuda", "Help"), t));
    items = items.push(menu_item(
        t,
        "more",
        t.lang.pick("Configurações", "Settings"),
        Message::ToggleSettings,
        false,
    ));
    items = items.push(menu_item(
        t,
        "home",
        t.lang
            .pick("Definir como leitor padrão…", "Set as default reader…"),
        Message::SetDefaultReader,
        false,
    ));
    items = items.push(menu_item(
        t,
        "file-text",
        t.lang.pick("Sobre o Tsuro PDF", "About Tsuro PDF"),
        Message::ToggleAbout,
        false,
    ));
    container(items)
        .width(Length::Fixed(264.0))
        .padding(6)
        .style(kiri::menu_style(t))
        .into()
}

/// Rótulo de recente no menu: `pasta/arquivo.pdf` (como o cartão da tela
/// vazia, numa linha só), com elipse no meio se passar de 30.
fn recent_label(path: &std::path::Path) -> String {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let full = match path.parent().and_then(|p| p.file_name()) {
        Some(parent) => format!("{}/{}", parent.to_string_lossy(), name),
        None => name,
    };
    middle_truncate(&full, 30)
}

/// Linha do menu ⋯: ícone Ori + rótulo + dica do atalho à direita;
/// `active` pinta o modo vigente (`accent_bg` + texto `accent`).
fn menu_item(
    t: Tokens,
    icon: &str,
    label: impl Into<String>,
    message: Message,
    active: bool,
) -> Element<'static, Message> {
    let hint = crate::session::shortcut_hint(&message);
    let mut content = row![kiri::ori_icon(icon, 16.0), text(label.into()).size(13)]
        .spacing(8)
        .align_y(Alignment::Center);
    if let Some(hint) = hint {
        content = content
            .push(Space::with_width(Length::Fill))
            .push(text(hint).size(12).color(t.muted));
    }
    button(content)
        .width(Length::Fill)
        .padding(Padding::from([8, 10]))
        .style(kiri::panel_seg_style(t, active))
        .on_press(message)
        .into()
}

/// Linha de recente no grupo Recentes: sem ícone, recuada para marcar que é
/// documento (filho do grupo), não comando do Arquivo.
fn menu_recent(t: Tokens, label: String, message: Message) -> Element<'static, Message> {
    button(text(label).size(13))
        .width(Length::Fill)
        .padding(Padding {
            top: 8.0,
            right: 10.0,
            bottom: 8.0,
            left: 34.0,
        })
        .style(kiri::panel_seg_style(t, false))
        .on_press(message)
        .into()
}

/// Linha desabilitada do menu (sem ação): rótulo `muted`, alinhado aos demais.
fn menu_disabled(t: Tokens, label: &'static str) -> Element<'static, Message> {
    container(
        row![
            Space::with_width(Length::Fixed(16.0)),
            text(label).size(13).color(t.muted),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding(Padding::from([8, 10]))
    .into()
}

/// Modal de impressão: fundo fecha ao clicar, cartão captura sem efeito —
/// nada atravessa para o documento atrás.
fn print_layer<'a>(ready: &'a Ready, dialog: &'a PrintDialog, t: Tokens) -> Element<'a, Message> {
    let dim = container(Space::with_width(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.55))),
            ..container::Style::default()
        });
    let card = container(mouse_area(print_card(ready, dialog, t)).on_press(Message::PrintNop))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);
    stack![mouse_area(dim).on_press(Message::ClosePrintDialog), card,].into()
}

fn print_card<'a>(ready: &'a Ready, dialog: &'a PrintDialog, t: Tokens) -> Element<'a, Message> {
    let pages = dialog.preview_pages(ready.page_count(), ready.visible);
    let at = dialog.preview.min(pages.len().saturating_sub(1));
    let mut col = column![
        row![
            text(t.lang.pick("Imprimir", "Print")).size(16),
            Space::with_width(Length::Fill),
            button(text(t.lang.pick("Fechar", "Close")).size(13))
                .style(kiri::menu_item_style(t))
                .on_press_maybe((!dialog.busy).then_some(Message::ClosePrintDialog)),
        ]
        .align_y(Alignment::Center),
        row![
            print_preview(ready, dialog, &pages, at, t),
            print_controls(dialog, t),
        ]
        .spacing(16),
    ]
    .spacing(12);
    if let Some(err) = &dialog.error {
        col = col.push(
            text(format!(
                "{}: {err}",
                t.lang.pick("Não foi possível imprimir", "Could not print")
            ))
            .size(13)
            .color(t.danger),
        );
    }
    col = col.push(print_footer(dialog, t));
    container(col)
        .width(Length::Fixed(620.0))
        .padding(16)
        .style(kiri::menu_style(t))
        .into()
}

/// Preview reaproveita o thumb do cache (escala de tela, sem render novo).
fn print_preview<'a>(
    ready: &'a Ready,
    dialog: &'a PrintDialog,
    pages: &[PageNo],
    at: usize,
    t: Tokens,
) -> Element<'a, Message> {
    let thumb: Element<'a, Message> =
        match pages.get(at).and_then(|page| ready.thumb_surface(*page)) {
            Some(surface) => image(surface.image.clone())
                .width(Length::Fixed(220.0))
                .into(),
            None => container(
                text(t.lang.pick("carregando…", "loading…"))
                    .size(13)
                    .color(t.muted),
            )
            .width(Length::Fixed(220.0))
            .height(Length::Fixed(280.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into(),
        };
    let pager = row![
        button(kiri::ori!("chevron-left"))
            .style(kiri::ibtn_style(t, false))
            .on_press_maybe((!dialog.busy && at > 0).then_some(Message::PrintPreviewPrev)),
        text(format!(
            "{} {} {}",
            at + 1,
            t.lang.pick("de", "of"),
            pages.len().max(1)
        ))
        .size(13),
        button(kiri::ori!("chevron-right"))
            .style(kiri::ibtn_style(t, false))
            .on_press_maybe(
                (!dialog.busy && at + 1 < pages.len()).then_some(Message::PrintPreviewNext)
            ),
    ]
    .spacing(8)
    .align_y(Alignment::Center);
    column![thumb, pager]
        .spacing(8)
        .align_x(Alignment::Center)
        .into()
}

fn print_controls(dialog: &PrintDialog, t: Tokens) -> Element<'_, Message> {
    let busy = dialog.busy;
    let printer: Element<'_, Message> = if dialog.printers_loading {
        text(t.lang.pick("Carregando impressoras…", "Loading printers…"))
            .size(13)
            .color(t.muted)
            .into()
    } else if dialog.printers.is_empty() {
        text(
            t.lang
                .pick("Nenhuma impressora encontrada", "No printer found"),
        )
        .size(13)
        .color(t.warn)
        .into()
    } else {
        let names: Vec<String> = dialog.printers.iter().map(|p| p.name.clone()).collect();
        let selected = dialog.selected_printer().map(|p| p.name.clone());
        pick_list(names.clone(), selected, move |name: String| {
            Message::PrintSelectPrinter(names.iter().position(|n| *n == name).unwrap_or(0))
        })
        .placeholder(t.lang.pick("Impressora", "Printer"))
        .width(Length::Fill)
        .into()
    };
    let mut col = column![
        section_title(t.lang.pick("Impressora", "Printer"), t),
        printer,
        section_title(t.lang.pick("Páginas", "Pages"), t),
        row![
            seg_button(t, t.lang.pick("Todas", "All"), RangeMode::All, dialog, busy),
            seg_button(
                t,
                t.lang.pick("Atual", "Current"),
                RangeMode::Current,
                dialog,
                busy
            ),
            seg_button(
                t,
                t.lang.pick("De–Até", "From–To"),
                RangeMode::Custom,
                dialog,
                busy
            ),
        ]
        .spacing(4),
    ]
    .spacing(6);
    if dialog.range_mode == RangeMode::Custom {
        col = col.push(
            row![
                text_input(t.lang.pick("De", "From"), &dialog.from_input)
                    .on_input(Message::PrintSetFromInput)
                    .width(Length::Fixed(64.0)),
                text(t.lang.pick("até", "to")).size(13).color(t.muted),
                text_input(t.lang.pick("Até", "To"), &dialog.to_input)
                    .on_input(Message::PrintSetToInput)
                    .width(Length::Fixed(64.0)),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        );
    }
    col = col.push(section_title(t.lang.pick("Cópias", "Copies"), t));
    col = col.push(
        row![
            button(kiri::ori!("minus"))
                .style(kiri::ibtn_style(t, false))
                .on_press_maybe((!busy && dialog.copies > 1).then_some(Message::PrintCopiesMinus)),
            container(text(dialog.copies.to_string()).size(14))
                .width(Length::Fixed(32.0))
                .align_x(Alignment::Center),
            button(kiri::ori!("plus"))
                .style(kiri::ibtn_style(t, false))
                .on_press_maybe(
                    (!busy && dialog.copies < MAX_COPIES).then_some(Message::PrintCopiesPlus)
                ),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    );
    col = col.push(section_title(t.lang.pick("Orientação", "Orientation"), t));
    col = col.push(
        row![
            ori_button(
                t,
                t.lang.pick("Automática", "Automatic"),
                PrintOrientation::Auto,
                dialog,
                busy
            ),
            ori_button(
                t,
                t.lang.pick("Retrato", "Portrait"),
                PrintOrientation::Portrait,
                dialog,
                busy
            ),
            ori_button(
                t,
                t.lang.pick("Paisagem", "Landscape"),
                PrintOrientation::Landscape,
                dialog,
                busy
            ),
        ]
        .spacing(4),
    );
    col.width(Length::Fill).into()
}

fn seg_button(
    t: Tokens,
    label: &'static str,
    mode: RangeMode,
    dialog: &PrintDialog,
    busy: bool,
) -> Element<'static, Message> {
    button(text(label).size(13))
        .style(kiri::ibtn_style(t, dialog.range_mode == mode))
        .on_press_maybe((!busy).then_some(Message::PrintSetRangeMode(mode)))
        .into()
}

fn ori_button(
    t: Tokens,
    label: &'static str,
    orientation: PrintOrientation,
    dialog: &PrintDialog,
    busy: bool,
) -> Element<'static, Message> {
    button(text(label).size(13))
        .style(kiri::ibtn_style(t, dialog.orientation == orientation))
        .on_press_maybe((!busy).then_some(Message::PrintSetOrientation(orientation)))
        .into()
}

fn print_footer(dialog: &PrintDialog, t: Tokens) -> Element<'static, Message> {
    let busy = dialog.busy;
    let label = if busy {
        t.lang.pick("Enviando…", "Sending…")
    } else {
        t.lang.pick("Imprimir", "Print")
    };
    let secondary = |label: &'static str, message: Message| {
        button(text(label).size(13))
            .padding(Padding::from([8, 12]))
            .style(kiri::hud_ghost_style(t))
            .on_press_maybe((!busy).then_some(message))
    };
    row![
        Space::with_width(Length::Fill),
        secondary(t.lang.pick("Abrir PDF", "Open PDF"), Message::PrintOpenPdf),
        secondary(t.lang.pick("Cancelar", "Cancel"), Message::ClosePrintDialog),
        button(text(label).size(13))
            .padding(Padding::from([8, 12]))
            .style(kiri::hud_primary_style(t))
            .on_press_maybe((!busy).then_some(Message::PrintSubmit)),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

/// Post-it (issue #30): o editor abre ancorado no trecho da nota, não no
/// centro da janela — o rascunho guarda o canto (origem da folha no clique +
/// rolagem) e a vista o desenha ali, preso à janela. Mesmo padrão do modal de
/// impressão: o fundo escurece e cancela no clique, o cartão engole o clique
/// (`PrintNop` é no-op) e o editor dentro dele recebe os eventos primeiro.
fn note_layer(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    let Some(draft) = ready.note_draft.as_ref() else {
        return Space::with_width(Length::Fill).into();
    };
    let [x, y] = ready.postit_pos([POSTIT_W, POSTIT_H]);
    let dim = container(Space::with_width(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
            ..container::Style::default()
        });
    let card = container(mouse_area(note_card(draft, t)).on_press(Message::PrintNop))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Start)
        .align_y(Alignment::Start)
        .padding(Padding {
            top: y,
            right: 0.0,
            bottom: 0.0,
            left: x,
        });
    stack![mouse_area(dim).on_press(Message::NoteCancel), card,].into()
}
/// Mini-toolbar da seleção (issue #51): os mesmos quatro `Annotate` da topbar
/// num segmento flutuante ancorado no trecho. Sem dim: o cartão posicionado
/// por padding não captura clique fora dos botões. Folha sempre branca, então
/// o segmento usa o estilo da barra em qualquer tema.
fn selection_bar_layer(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    let Some([x, y]) = ready.selection_bar_pos() else {
        return Space::with_width(Length::Fill).into();
    };
    let seg = |icon: Element<'static, Message>, label: &'static str, kind| {
        tip(
            t,
            control_seg(t, button(icon).on_press(Message::Annotate(kind))),
            label,
        )
    };
    let bar = container(
        row![
            seg(
                seg_icon("highlighter"),
                t.lang.pick("Destacar (H)", "Highlight (H)"),
                AnnotKind::Highlight
            ),
            seg(
                seg_icon("underline"),
                t.lang.pick("Sublinhar (U)", "Underline (U)"),
                AnnotKind::Underline
            ),
            seg(
                seg_icon("strike"),
                t.lang.pick("Riscar (S)", "Strikeout (S)"),
                AnnotKind::Strikeout
            ),
            seg(
                seg_icon("note"),
                t.lang.pick("Nota (N)", "Note (N)"),
                AnnotKind::Note
            ),
        ]
        .spacing(0)
        .align_y(Alignment::Center),
    )
    .padding(2)
    .style(kiri::bar_mark_style(t));
    container(bar)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Start)
        .align_y(Alignment::Start)
        .padding(Padding {
            top: y,
            right: 0.0,
            bottom: 0.0,
            left: x,
        })
        .into()
}

/// Cartão do post-it: rótulo caps + editor multilinha + rodapé com o vermelho
/// `remover nota` (só em edição), `Cancelar` ghost e `Salvar` primária. Texto
/// escuro fixo: o papel é amarelo em qualquer tema.
fn note_card(draft: &NoteDraft, t: Tokens) -> Element<'_, Message> {
    let editing = draft.editing.is_some();
    let can_save = !draft.content.text().trim().is_empty();
    let mut footer = row![].spacing(8).align_y(Alignment::Center);
    if editing {
        footer = footer.push(
            button(text(t.lang.pick("remover nota", "delete note")).size(13))
                .padding(Padding::from([8, 12]))
                .style(sticky_button_style(t.danger))
                .on_press(Message::NoteDelete),
        );
    }
    container(
        column![
            text(if editing {
                t.lang.pick("EDITAR NOTA", "EDIT NOTE")
            } else {
                t.lang.pick("NOVA NOTA", "NEW NOTE")
            })
            .size(10)
            .color(STICKY_INK),
            container(Space::with_height(Length::Fixed(1.0)))
                .width(Length::Fill)
                .style(|_| container::Style {
                    background: Some(Background::Color(STICKY_LINE)),
                    ..container::Style::default()
                }),
            text_editor(&draft.content)
                .placeholder(t.lang.pick("Escreva a nota…", "Write the note…"))
                .on_action(Message::NoteEdit)
                .padding(6)
                .size(13)
                .height(Length::Fixed(POSTIT_EDITOR_H))
                .style(sticky_editor_style),
            footer
                .push(Space::with_width(Length::Fill))
                .push(
                    button(text(t.lang.pick("Cancelar", "Cancel")).size(13))
                        .padding(Padding::from([8, 12]))
                        .style(sticky_button_style(STICKY_INK))
                        .on_press(Message::NoteCancel),
                )
                .push(
                    button(text(t.lang.pick("Salvar", "Save")).size(13))
                        .padding(Padding::from([8, 12]))
                        .style(kiri::hud_primary_style(t))
                        .on_press_maybe(can_save.then_some(Message::NoteSave)),
                ),
        ]
        .spacing(8),
    )
    .width(Length::Fixed(POSTIT_W))
    .padding(12)
    .style(sticky_style)
    .into()
}

/// Papel do post-it: fundo amarelo, borda discreta e sombra de papel solto
/// (o cartão flutua sobre a folha, então precisa descolar dela).
fn sticky_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(STICKY)),
        border: Border {
            color: STICKY_LINE,
            width: 1.0,
            radius: 3.0.into(),
        },
        shadow: iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.35),
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
        text_color: Some(STICKY_INK),
        ..container::Style::default()
    }
}

/// Botão de texto sobre o papel (ghost): sem fundo, tinta da cor pedida —
/// `t.danger` no `remover nota`, a tinta do papel no `Cancelar`.
fn sticky_button_style(color: Color) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let interactive = matches!(status, button::Status::Hovered | button::Status::Pressed);
        button::Style {
            background: Some(Background::Color(if interactive {
                Color::from_rgba(0.0, 0.0, 0.0, 0.10)
            } else {
                Color::TRANSPARENT
            })),
            text_color: color,
            border: Border {
                radius: 6.0.into(),
                ..Border::default()
            },
            shadow: iced::Shadow::default(),
        }
    }
}

/// Editor sobre o papel: fundo do próprio papel (sem caixa branca), tinta
/// escura e cursor/seleção legíveis no amarelo.
fn sticky_editor_style(_theme: &iced::Theme, _status: text_editor::Status) -> text_editor::Style {
    text_editor::Style {
        background: Background::Color(STICKY),
        border: Border::default(),
        icon: STICKY_INK,
        placeholder: Color::from_rgba(STICKY_INK.r, STICKY_INK.g, STICKY_INK.b, 0.55),
        value: STICKY_INK,
        selection: Color::from_rgba(0.42, 0.30, 0.02, 0.25),
    }
}

/// Cartão de recuperação (#128, `funcoes/01-marcacoes.md`): aviso modal como o
/// de assinatura. O fundo equivale a "Decidir depois": não apaga nada.
fn recovery_layer(ready: &Ready, t: Tokens) -> Element<'static, Message> {
    let dim = container(Space::with_width(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
            ..container::Style::default()
        });
    let card = container(mouse_area(recovery_card(ready, t)).on_press(Message::PrintNop))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);
    stack![mouse_area(dim).on_press(Message::RecoveryLater), card,].into()
}

fn recovery_card(ready: &Ready, t: Tokens) -> Element<'static, Message> {
    let lang = t.lang;
    let (title, detail, restorable) = match &ready.recovery {
        RecoveryState::Disponivel(offers) => {
            let count = offers
                .first()
                .map_or(0, |offer| offer.checkpoint.annotations.len());
            let noun = if count == 1 {
                lang.pick(
                    "marcação de uma sessão anterior.",
                    "mark from an earlier session.",
                )
            } else {
                lang.pick(
                    "marcações de uma sessão anterior.",
                    "marks from an earlier session.",
                )
            };
            (
                lang.pick(
                    "Marcações recuperáveis encontradas.",
                    "Recoverable annotations found.",
                ),
                format!("{count} {noun}"),
                true,
            )
        }
        RecoveryState::Incompativel(_) => (
            lang.pick(
                "Marcações de uma versão anterior.",
                "Marks from an earlier version.",
            ),
            lang.pick(
                "O arquivo mudou desde essas marcações. Nada foi aplicado.",
                "The file changed since those marks. Nothing was applied.",
            )
            .to_string(),
            false,
        ),
        _ => return Space::new(Length::Shrink, Length::Shrink).into(),
    };
    // Diário de outro caminho com o mesmo conteúdo: mostra de onde veio.
    let origin = match &ready.recovery {
        RecoveryState::Disponivel(offers) | RecoveryState::Incompativel(offers) => offers
            .first()
            .filter(|offer| offer.checkpoint.path.as_path() != ready.source.path())
            .map(|offer| {
                format!(
                    "{} {}",
                    lang.pick("Origem:", "From:"),
                    offer.checkpoint.path.display()
                )
            }),
        _ => None,
    };
    let mut body = column![text(title).size(16), text(detail).size(13)].spacing(12);
    if let Some(origin) = origin {
        body = body.push(text(origin).size(12).color(t.muted));
    }
    if ready.discard_ask {
        body = body.push(
            text(lang.pick(
                "Descartar essas marcações? Não dá para desfazer.",
                "Discard these marks? This cannot be undone.",
            ))
            .size(13),
        );
    }
    let mut buttons = row![Space::with_width(Length::Fill)]
        .spacing(8)
        .align_y(Alignment::Center);
    if ready.discard_ask {
        buttons = buttons
            .push(
                button(text(lang.pick("Cancelar", "Cancel")).size(13))
                    .padding(Padding::from([8, 12]))
                    .style(kiri::hud_ghost_style(t))
                    .on_press(Message::RecoveryDiscardAsk(false)),
            )
            .push(
                button(text(lang.pick("Descartar", "Discard")).size(13))
                    .padding(Padding::from([8, 12]))
                    .style(kiri::hud_primary_style(t))
                    .on_press(Message::RecoveryDiscardConfirm),
            );
    } else {
        buttons = buttons.push(
            button(text(lang.pick("Decidir depois", "Decide later")).size(13))
                .padding(Padding::from([8, 12]))
                .style(kiri::hud_ghost_style(t))
                .on_press(Message::RecoveryLater),
        );
        buttons = buttons.push(
            button(text(lang.pick("Descartar", "Discard")).size(13))
                .padding(Padding::from([8, 12]))
                .style(kiri::hud_ghost_style(t))
                .on_press(Message::RecoveryDiscardAsk(true)),
        );
        if restorable {
            buttons = buttons.push(
                button(text(lang.pick("Restaurar", "Restore")).size(13))
                    .padding(Padding::from([8, 12]))
                    .style(kiri::hud_primary_style(t))
                    .on_press(Message::RecoveryRestore),
            );
        }
    }
    container(body.push(buttons))
        .width(Length::Fixed(440.0))
        .padding(16)
        .style(kiri::menu_style(t))
        .into()
}

/// Falha de gravação do diário: aviso na faixa do documento, memória intacta.
fn recovery_failed_row(t: Tokens) -> Element<'static, Message> {
    text(t.lang.pick(
        "Não foi possível guardar a recuperação.",
        "Recovery could not be saved.",
    ))
    .size(13)
    .color(t.danger)
    .into()
}

/// Configurações → Recuperação: esquecer diários sem dono. Pede confirmação,
/// porque apaga marcações não salvas que ainda não voltaram.
fn forget_recovery_block(settings: &SettingsPanel, t: Tokens) -> Element<'static, Message> {
    let mut block = column![].spacing(8);
    if settings.forget_ask {
        block = block
            .push(
                text(t.lang.pick(
                    "Apaga os diários sem sessão aberta. As marcações não salvas neles somem.",
                    "Deletes journals with no open session. Their unsaved marks are lost.",
                ))
                .size(12)
                .color(t.muted),
            )
            .push(
                row![
                    button(text(t.lang.pick("Cancelar", "Cancel")).size(13))
                        .padding(Padding::from([8, 12]))
                        .style(kiri::hud_ghost_style(t))
                        .on_press(Message::ForgetRecoveryAsk(false)),
                    button(text(t.lang.pick("Esquecer", "Forget")).size(13))
                        .padding(Padding::from([8, 12]))
                        .style(kiri::hud_primary_style(t))
                        .on_press(Message::ForgetRecoveryConfirm),
                ]
                .spacing(8),
            );
    } else {
        block = block.push(
            button(
                text(t.lang.pick(
                    "Esquecer diários de recuperação…",
                    "Forget recovery journals…",
                ))
                .size(13),
            )
            .padding(Padding::from([8, 12]))
            .style(kiri::hud_ghost_style(t))
            .on_press(Message::ForgetRecoveryAsk(true)),
        );
    }
    if let Some(status) = &settings.forget_status {
        block = block.push(text(status.clone()).size(12).color(t.muted));
    }
    block.into()
}

/// Aviso de documento assinado antes de salvar a cópia (⋯ → Salvar cópia):
/// mesmo padrão do popover de nota — fundo escurece e cancela no clique,
/// cartão engole o clique (`PrintNop` é no-op).
fn save_warning_layer(t: Tokens) -> Element<'static, Message> {
    let dim = container(Space::with_width(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
            ..container::Style::default()
        });
    let card = container(mouse_area(save_warning_card(t)).on_press(Message::PrintNop))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);
    stack![mouse_area(dim).on_press(Message::SaveCopyCancelled), card,].into()
}

/// Sobre (Ajuda → Sobre o Tsuro PDF): nome, versão e missão — mesmo padrão
/// do aviso de assinado (fundo fecha, cartão engole o clique).
fn about_layer(t: Tokens) -> Element<'static, Message> {
    let dim = container(Space::with_width(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
            ..container::Style::default()
        });
    let card = container(mouse_area(about_card(t)).on_press(Message::PrintNop))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);
    stack![mouse_area(dim).on_press(Message::ToggleAbout), card,].into()
}

fn about_card(t: Tokens) -> Element<'static, Message> {
    let mut body = column![
        text("Tsuro PDF").size(16).color(t.ink),
        text(format!(
            "{} {}",
            t.lang.pick("Versão", "Version"),
            env!("CARGO_PKG_VERSION")
        ))
        .size(13)
        .color(t.muted),
        text(t.lang.pick(
            "Ler, anotar, marcar, imprimir. Nada além disso.",
            "Read, annotate, mark, print. Nothing more."
        ))
        .size(13)
        .color(t.ink),
        button(text(t.lang.pick("Fechar", "Close")).size(13))
            .padding(Padding::from([8, 12]))
            .style(kiri::menu_item_style(t))
            .on_press(Message::ToggleAbout),
    ];
    // macOS: trocar o leitor padrão é escolha do usuário no Finder.
    if cfg!(target_os = "macos") {
        body = body.push(
            text(t.lang.pick(
                "Leitor padrão: Finder › Obter Informações › Abrir com › TsuroPDF › Alterar tudo.",
                "Default reader: Finder › Get Info › Open With › TsuroPDF › Change All.",
            ))
            .size(12)
            .color(t.muted),
        );
    }
    let body = body.spacing(8).align_x(Alignment::Center);
    container(body)
        .width(Length::Fixed(320.0))
        .padding(16)
        .style(kiri::menu_style(t))
        .into()
}

/// Configurações (Ajuda → Configurações): tema, leitor padrão, permissões,
/// idioma e versão — mesmo padrão do Sobre (fundo fecha, cartão engole).
fn settings_layer(settings: &SettingsPanel, t: Tokens) -> Element<'static, Message> {
    let dim = container(Space::with_width(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
            ..container::Style::default()
        });
    let card = container(mouse_area(settings_card(settings, t)).on_press(Message::PrintNop))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);
    stack![mouse_area(dim).on_press(Message::ToggleSettings), card,].into()
}

#[allow(clippy::too_many_lines)]
fn settings_card(settings: &SettingsPanel, t: Tokens) -> Element<'static, Message> {
    let mut body = column![
        text(t.lang.pick("Configurações", "Settings"))
            .size(16)
            .color(t.ink),
        section_title(t.lang.pick("Tema", "Theme"), t),
        row![
            menu_theme_button(
                t,
                t.lang.pick("Sistema", "System"),
                ThemePref::System,
                settings.theme_pref == ThemePref::System
            ),
            menu_theme_button(
                t,
                t.lang.pick("Escuro", "Dark"),
                ThemePref::Dark,
                settings.theme_pref == ThemePref::Dark
            ),
            menu_theme_button(
                t,
                t.lang.pick("Claro", "Light"),
                ThemePref::Light,
                settings.theme_pref == ThemePref::Light
            ),
        ]
        .spacing(4),
        section_title(t.lang.pick("Leitor padrão", "Default reader"), t),
        menu_item(
            t,
            "home",
            t.lang
                .pick("Definir como leitor padrão…", "Set as default reader…"),
            Message::SetDefaultReader,
            false,
        ),
    ]
    .spacing(8)
    .align_x(Alignment::Start)
    .width(300);
    #[cfg(target_os = "macos")]
    {
        let label = match settings.permissions {
            PermissionsState::Checking => t.lang.pick(
                "Downloads: verificando acesso…",
                "Downloads: checking access…",
            ),
            PermissionsState::Accessible => t
                .lang
                .pick("Downloads: acesso liberado.", "Downloads: access granted."),
            PermissionsState::Denied => t
                .lang
                .pick("Downloads: acesso negado.", "Downloads: access denied."),
            PermissionsState::Unavailable => t.lang.pick(
                "Downloads: acesso não disponível.",
                "Downloads: access unavailable.",
            ),
        };
        body = body
            .push(section_title(t.lang.pick("Permissões", "Permissions"), t))
            .push(text(label).size(13).color(
                if settings.permissions == PermissionsState::Denied {
                    t.danger
                } else {
                    t.muted
                },
            ));
        if settings.permissions == PermissionsState::Denied {
            body = body.push(menu_item(
                t,
                "shield",
                t.lang.pick("Abrir Ajustes…", "Open Settings…"),
                Message::OpenPrivacySettings,
                false,
            ));
        }
    }
    body = body
        .push(section_title(t.lang.pick("Recuperação", "Recovery"), t))
        .push(forget_recovery_block(settings, t))
        .push(section_title(t.lang.pick("Idioma", "Language"), t))
        .push(
            row![
                menu_lang_button(t, "Português", Lang::Pt, t.lang == Lang::Pt),
                menu_lang_button(t, "English", Lang::En, t.lang == Lang::En),
            ]
            .spacing(4),
        )
        .push(section_title(t.lang.pick("Sobre", "About"), t))
        .push(
            text(format!(
                "{} {}",
                t.lang.pick("Versão", "Version"),
                env!("CARGO_PKG_VERSION")
            ))
            .size(13)
            .color(t.muted),
        );
    body = match &settings.update {
        UpdateState::Checking => body.push(
            text(
                t.lang
                    .pick("Verificando atualização…", "Checking for updates…"),
            )
            .size(12)
            .color(t.muted),
        ),
        UpdateState::Current => body.push(
            text(t.lang.pick("Você está atualizado.", "You're up to date."))
                .size(12)
                .color(t.muted),
        ),
        UpdateState::Available(tag) => body
            .push(
                text(format!(
                    "{}: {tag}",
                    t.lang
                        .pick("Nova versão disponível", "New version available")
                ))
                .size(12)
                .color(t.accent),
            )
            .push(update_action(t, tag)),
        UpdateState::Installing(tag) => body.push(
            text(format!(
                "{} {tag}…",
                t.lang
                    .pick("Baixando e instalando", "Downloading and installing")
            ))
            .size(12)
            .color(t.muted),
        ),
        UpdateState::Installed(tag) => body
            .push(
                text(format!(
                    "{tag} {}",
                    t.lang.pick(
                        "instalada. Reinicie para usar.",
                        "installed. Restart to use it."
                    )
                ))
                .size(12)
                .color(t.accent),
            )
            .push(menu_item(
                t,
                "rotate",
                t.lang.pick("Reiniciar agora", "Restart now"),
                Message::RestartToUpdate,
                false,
            )),
        UpdateState::InstallFailed(tag, error) => body
            .push(text(error.message(t.lang)).size(12).color(t.muted))
            .push(menu_item(
                t,
                "save",
                t.lang.pick("Tentar de novo", "Try again"),
                Message::InstallUpdate(tag.clone()),
                false,
            ))
            .push(menu_item(
                t,
                "file-text",
                t.lang.pick("Abrir página da release", "Open release page"),
                Message::OpenReleasePage(tag.clone()),
                false,
            )),
        UpdateState::Idle | UpdateState::Failed => body,
    };
    body = body.push(
        button(text(t.lang.pick("Fechar", "Close")).size(13))
            .padding(Padding::from([8, 12]))
            .style(kiri::menu_item_style(t))
            .on_press(Message::ToggleSettings),
    );
    container(body)
        .padding(12)
        .width(324)
        .style(kiri::menu_style(t))
        .into()
}

/// Fechar aba, Home ou a janela com marcações não salvas.
fn close_prompt_layer(t: Tokens) -> Element<'static, Message> {
    let dim = container(Space::with_width(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
            ..container::Style::default()
        });
    let card = container(mouse_area(close_prompt_card(t)).on_press(Message::PrintNop))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);
    stack![mouse_area(dim).on_press(Message::CloseCancel), card,].into()
}

fn close_prompt_card(t: Tokens) -> Element<'static, Message> {
    container(
        column![
            text(t.lang.pick("Marcações não salvas", "Unsaved marks")).size(16),
            text(t.lang.pick(
                "Fechar descarta as marcações desta sessão.",
                "Closing discards this session's marks."
            ))
            .size(13),
            row![
                Space::with_width(Length::Fill),
                button(text(t.lang.pick("Cancelar", "Cancel")).size(13))
                    .padding(Padding::from([8, 12]))
                    .style(kiri::hud_ghost_style(t))
                    .on_press(Message::CloseCancel),
                button(text(t.lang.pick("Descartar", "Discard")).size(13))
                    .padding(Padding::from([8, 12]))
                    .style(kiri::hud_ghost_style(t))
                    .on_press(Message::CloseDiscard),
                button(text(t.lang.pick("Salvar cópia", "Save copy")).size(13))
                    .padding(Padding::from([8, 12]))
                    .style(kiri::hud_primary_style(t))
                    .on_press(Message::CloseSave),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(12),
    )
    .width(Length::Fixed(440.0))
    .padding(16)
    .style(kiri::menu_style(t))
    .into()
}

/// Só o estado da paleta — o overlay não pede Tabs/Ready.
fn palette_layer(palette: &PaletteState, t: Tokens) -> Element<'_, Message> {
    let dim = container(Space::with_width(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.25))),
            ..container::Style::default()
        });
    // Âncora no topo (Spotlight/VSCode), não no centro como os outros modais.
    let card = container(mouse_area(palette_card(palette, t)).on_press(Message::PrintNop))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Start)
        .padding(Padding {
            top: 64.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        });
    stack![mouse_area(dim).on_press(Message::PaletteClose), card].into()
}

/// Título de 1 linha para a linha da paleta: outline trunca como no
/// painel; o resto só colapsa whitespace (trechos já vêm colapsados).
fn palette_title(item: &PaletteItem) -> String {
    match item {
        PaletteItem::OutlineRow { title, .. } => outline_title(title),
        _ => item
            .title()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
    }
}

fn palette_card(palette: &PaletteState, t: Tokens) -> Element<'_, Message> {
    let query = text_input(
        t.lang.pick("Digite um comando…", "Type a command…"),
        palette.query(),
    )
    .id(palette_input_id())
    .on_input(Message::PaletteQuery)
    .on_submit(Message::PaletteConfirm)
    .style(kiri::bar_input_style(t))
    .padding([8, 10])
    .size(14)
    .width(Length::Fill);
    let mut rows = column![].spacing(2);
    if palette.items().is_empty() {
        rows = rows.push(
            text(t.lang.pick("Nenhum resultado", "No results"))
                .size(13)
                .color(t.muted),
        );
    } else {
        for (i, item) in palette.items().iter().enumerate() {
            let selected = Some(i) == palette.selected();
            let mut titlecol = column![text(palette_title(item)).size(13)].spacing(1);
            // Segunda linha só com hint; sem isso a linha fica compacta.
            if let Some(sub) = item.subtitle() {
                let one_line = sub.split_whitespace().collect::<Vec<_>>().join(" ");
                if !one_line.is_empty() {
                    titlecol = titlecol.push(text(one_line).size(11).color(t.muted));
                }
            }
            rows = rows.push(
                button(
                    row![
                        kiri::ori_icon(item.icon(), 16.0),
                        titlecol,
                        Space::with_width(Length::Fill),
                        text((i + 1).to_string()).size(11).color(t.muted),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                    .width(Length::Fill),
                )
                .width(Length::Fill)
                .padding(Padding::from([8, 10]))
                .style(kiri::panel_seg_style(t, selected))
                .on_press(Message::PaletteSelect(i)),
            );
        }
    }
    let list = container(
        scrollable(rows)
            .id(palette_scroll_id())
            .width(Length::Fill)
            .height(Length::Shrink),
    )
    .max_height(PALETTE_LIST_MAX_H)
    .width(Length::Fill);
    container(column![query, list].spacing(8))
        .width(Length::Fixed(560.0))
        .padding(12)
        .style(kiri::menu_style(t))
        .into()
}

/// Pergunta sim/não do aviso de assinatura; "Salvar mesmo assim" segue para o
/// diálogo de destino.
fn save_warning_card(t: Tokens) -> Element<'static, Message> {
    container(
        column![
            text(t.lang.pick("Documento assinado", "Signed document")).size(16),
            text(t.lang.pick(
                "Salvar marcações invalida a assinatura digital. Continuar?",
                "Saving marks invalidates the digital signature. Continue?"
            ))
            .size(13),
            row![
                Space::with_width(Length::Fill),
                button(text(t.lang.pick("Voltar", "Back")).size(13))
                    .padding(Padding::from([8, 12]))
                    .style(kiri::hud_ghost_style(t))
                    .on_press(Message::SaveCopyCancelled),
                button(text(t.lang.pick("Salvar mesmo assim", "Save anyway")).size(13))
                    .padding(Padding::from([8, 12]))
                    .style(kiri::hud_primary_style(t))
                    .on_press(Message::SaveCopyConfirmed),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(12),
    )
    .width(Length::Fixed(440.0))
    .padding(16)
    .style(kiri::menu_style(t))
    .into()
}

fn menu_theme_button(
    t: Tokens,
    label: &'static str,
    pref: ThemePref,
    active: bool,
) -> Element<'static, Message> {
    control_active(
        t,
        button(text(label).size(13)).on_press(Message::SetTheme(pref)),
        active,
    )
    .width(Length::Fill)
    .into()
}

fn menu_lang_button(
    t: Tokens,
    label: &'static str,
    lang: Lang,
    active: bool,
) -> Element<'static, Message> {
    control_active(
        t,
        button(text(label).size(13)).on_press(Message::SetLang(lang)),
        active,
    )
    .width(Length::Fill)
    .into()
}

/// Progresso Kiri: 2px, fill `accent` proporcional à página visível.
fn progress(ready: &Ready, t: Tokens) -> Element<'static, Message> {
    let n = ready.page_count().max(1);
    let done = (ready.visible.index() + 1).min(n);
    // Escala fixa em 1000 partes — sem overflow de `FillPortion` em docs grandes.
    let filled = ((done * 1000) / n).clamp(1, 1000) as u16;
    let mut bar = row![container(Space::with_width(Length::Fill))
        .width(Length::FillPortion(filled))
        .height(Length::Fixed(2.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.accent)),
            ..container::Style::default()
        }),]
    .spacing(0);
    if filled < 1000 {
        bar = bar.push(Space::new(
            Length::FillPortion(1000 - filled),
            Length::Shrink,
        ));
    }
    container(bar)
        .width(Length::Fill)
        .height(Length::Fixed(2.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.surface)),
            ..container::Style::default()
        })
        .into()
}

/// Moldura tracejada da dropzone (Stitch). `Border` do iced não tem dash,
/// então o retângulo vem do canvas — 1px `line`, traço 6/4.
struct EmptyDash {
    color: Color,
}

impl Program<Message> for EmptyDash {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        // Meio pixel para dentro: o traço de 1px cai na linha física.
        let rect = Path::rectangle(
            Point::new(0.5, 0.5),
            Size::new(bounds.width - 1.0, bounds.height - 1.0),
        );
        frame.stroke(
            &rect,
            Stroke {
                style: Style::Solid(self.color),
                width: 1.0,
                line_dash: LineDash {
                    segments: &[6.0, 4.0],
                    offset: 0,
                },
                ..Stroke::default()
            },
        );
        vec![frame.into_geometry()]
    }
}

/// Início em grade: barra de abrir (arrastar ou botão) e, abaixo, cartões —
/// recentes com a prévia do topo da 1ª página, pastas da pasta aberta e os
/// PDFs dela. A grade quebra conforme a largura; o conteúdo fica numa coluna
/// central com largura máxima.
/// `tabs_open`: Início por cima das abas — o título não diz "nenhum
/// documento" com documentos abertos na faixa acima.
fn empty_browser(empty: &EmptyState, tabs_open: bool, t: Tokens) -> Element<'_, Message> {
    let mut content = column![open_bar(tabs_open, t)].spacing(14);

    content = content.push(home_section(
        t,
        kiri::ori_icon("pages", 22.0),
        t.lang.pick("Recentes", "Recent"),
        Some(empty.recents.len()),
    ));
    if empty.recents.is_empty() {
        content = content.push(
            text(t.lang.pick("Nenhum arquivo recente.", "No recent files."))
                .size(13)
                .color(t.muted),
        );
    } else {
        content = content.push(home_grid(
            empty
                .recents
                .iter()
                .map(|path| file_card(path, empty.thumbs.get(path), t)),
        ));
    }

    // Pastas: título, caminho atual e voltar na mesma linha.
    let mut folders_head = row![home_section(
        t,
        kiri::ori_icon("folder", 22.0),
        t.lang.pick("Pastas", "Folders"),
        None,
    )]
    .spacing(10)
    .align_y(Alignment::Center);
    if let Some(parent) = empty.parent() {
        folders_head = folders_head.push(tip(
            t,
            control_seg(
                t,
                button(seg_icon("chevron-left")).on_press(Message::BrowseTo(parent)),
            ),
            t.lang.pick("Voltar", "Back"),
        ));
    }
    // Na raiz o título já diz tudo; dentro de uma pasta, o caminho (longo
    // perde o meio, o fim — a pasta atual — fica legível).
    if empty.cwd.is_some() {
        folders_head = folders_head.push(
            text(middle_truncate(&empty.path_label(), FOLDER_PATH_CHARS))
                .size(12)
                .color(t.muted)
                .wrapping(text::Wrapping::None),
        );
    }
    content = content.push(folders_head);
    let folders: Vec<&FsEntry> = empty.listing.iter().filter(|e| e.is_dir).collect();
    let pdfs: Vec<&FsEntry> = empty.listing.iter().filter(|e| !e.is_dir).collect();
    if let Some(err) = &empty.listing_error {
        content = content.push(text(err).size(13).color(t.muted));
    } else if empty.listing.is_empty() {
        content = content.push(
            text(
                t.lang
                    .pick("Nenhuma pasta ou PDF aqui.", "No folders or PDFs here."),
            )
            .size(13)
            .color(t.muted),
        );
    } else if !folders.is_empty() {
        content = content.push(home_grid(folders.iter().map(|entry| folder_card(entry, t))));
    }
    if !pdfs.is_empty() {
        content = content.push(home_section(
            t,
            kiri::ori_icon("file-text", 22.0),
            t.lang.pick("PDFs nesta pasta", "PDFs in this folder"),
            Some(pdfs.len()),
        ));
        content = content
            .push(home_grid(pdfs.iter().map(|entry| {
                file_card(&entry.path, empty.thumbs.get(&entry.path), t)
            })));
    }

    scrollable(
        container(
            content
                .max_width(HOME_MAX_W)
                .padding(Padding::from([12, 16])),
        )
        .width(Length::Fill)
        .center_x(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Largura máxima do conteúdo do Início (cabem 5 cartões por linha).
const HOME_MAX_W: f32 = 1000.0;
/// Cartão do Início e a área da página dentro da prévia (a prévia do motor
/// vem recortada nessa proporção — ver `Session::schedule_thumb`).
const HOME_CARD_W: f32 = 184.0;
pub const HOME_PREVIEW_W: f32 = 144.0;
pub const HOME_PREVIEW_H: f32 = 108.0;
/// Respiro da página dentro da área de prévia (topo e laterais).
const HOME_PAGE_INSET_TOP: f32 = 12.0;
const HOME_PAGE_INSET_X: f32 = (HOME_CARD_W - 12.0 - HOME_PREVIEW_W) / 2.0;

/// Barra de abrir: soltar o PDF em qualquer lugar da janela abre; aqui fica o
/// convite e o botão, numa faixa baixa tracejada (a tela é dos cartões).
fn open_bar(tabs_open: bool, t: Tokens) -> Element<'static, Message> {
    let bar_h = 88.0;
    container(stack![
        Canvas::new(EmptyDash { color: t.line })
            .width(Length::Fill)
            .height(Length::Fixed(bar_h)),
        container(
            row![
                container(
                    image(image::Handle::from_bytes(
                        &include_bytes!("../../../public/tsuro-horizontal-pdf.png")[..],
                    ))
                    .width(Length::Fixed(120.0)),
                )
                .padding(6)
                .style(kiri::logo_stage_style()),
                column![
                    text(if tabs_open {
                        t.lang
                            .pick("Abrir outro documento", "Open another document")
                    } else {
                        t.lang.pick("Nenhum documento aberto", "No document open")
                    })
                    .size(16),
                    text(t.lang.pick(
                        "Arraste e solte um arquivo PDF aqui",
                        "Drag and drop a PDF file here"
                    ))
                    .size(12)
                    .color(t.muted),
                ]
                .spacing(4),
                Space::with_width(Length::Fill),
                button(text(t.lang.pick("Abrir PDF", "Open PDF")).size(13))
                    .padding(Padding::from([6, 14]))
                    .style(kiri::hud_primary_style(t))
                    .on_press(Message::PickFile),
            ]
            .spacing(16)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fixed(bar_h))
        .center_y(Length::Fixed(bar_h))
        .padding(Padding::from([0, 18])),
    ])
    .width(Length::Fill)
    .style(kiri::empty_drop_style(t))
    .into()
}

const FOLDER_PATH_CHARS: usize = 40;

fn home_section(
    t: Tokens,
    icon: Element<'static, Message>,
    label: &'static str,
    count: Option<usize>,
) -> Element<'static, Message> {
    let mut head = row![
        icon,
        text(label)
            .size(15)
            .font(Font {
                weight: font::Weight::Semibold,
                ..Font::default()
            })
            .color(t.ink)
    ]
    .spacing(10)
    .align_y(Alignment::Center);
    if let Some(count) = count {
        head = head.push(
            container(text(count.to_string()).size(12).color(t.muted))
                .padding(Padding::from([1, 6]))
                .style(kiri::empty_badge_style(t)),
        );
    }
    head.push(
        container(Space::with_height(Length::Fixed(1.0)))
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(t.line)),
                ..container::Style::default()
            }),
    )
    .into()
}

fn home_grid<'a>(cards: impl Iterator<Item = Element<'a, Message>>) -> Element<'a, Message> {
    container(row(cards).spacing(14).wrap())
        .center_x(Length::Fill)
        .into()
}

/// Cartão de PDF: topo da 1ª página (papel em branco enquanto a prévia não
/// chega ou se o arquivo não abrir), nome e pasta.
fn file_card<'a>(path: &std::path::Path, thumb: Option<&Thumb>, t: Tokens) -> Element<'a, Message> {
    let page: Element<'a, Message> = match thumb {
        Some(Thumb::Ready(handle)) => image(handle.clone())
            .width(Length::Fixed(HOME_PREVIEW_W))
            .height(Length::Fixed(HOME_PREVIEW_H))
            .content_fit(iced::ContentFit::Cover)
            .into(),
        _ => container(Space::new(
            Length::Fixed(HOME_PREVIEW_W),
            Length::Fixed(HOME_PREVIEW_H),
        ))
        .style(kiri::home_page_style(t))
        .into(),
    };
    let preview = container(page).padding(Padding {
        top: HOME_PAGE_INSET_TOP,
        right: HOME_PAGE_INSET_X,
        bottom: 0.0,
        left: HOME_PAGE_INSET_X,
    });
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let parent = path
        .parent()
        .and_then(|p| p.file_name())
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    home_card(
        t,
        preview.into(),
        kiri::ori!("file-text"),
        name,
        parent,
        Message::OpenRecent(path.to_path_buf()),
    )
}

/// Cartão de pasta: ícone grande no lugar da prévia.
fn folder_card<'a>(entry: &FsEntry, t: Tokens) -> Element<'a, Message> {
    let preview = container(kiri::ori_icon("folder", 44.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill);
    home_card(
        t,
        preview.into(),
        kiri::ori!("folder"),
        entry.name.clone(),
        t.lang.pick("Pasta", "Folder").into(),
        Message::BrowseTo(Some(entry.path.clone())),
    )
}

/// Moldura comum dos cartões: área de prévia em cima, ícone + nome + detalhe
/// embaixo; o cartão inteiro é o botão.
fn home_card<'a>(
    t: Tokens,
    preview: Element<'a, Message>,
    glyph: Element<'static, Message>,
    title: String,
    detail: String,
    message: Message,
) -> Element<'a, Message> {
    let preview = container(preview)
        .width(Length::Fill)
        .height(Length::Fixed(HOME_PAGE_INSET_TOP + HOME_PREVIEW_H))
        .clip(true)
        .style(kiri::home_preview_style(t));
    let footer = row![
        glyph,
        column![
            text(middle_truncate(&title, 22))
                .size(13)
                .wrapping(text::Wrapping::None),
            text(middle_truncate(&detail, 26))
                .size(11)
                .color(t.muted)
                .wrapping(text::Wrapping::None),
        ]
        .spacing(2),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .padding(Padding::from([8, 4]));
    tip_owned(
        t,
        button(column![preview, footer])
            .width(Length::Fixed(HOME_CARD_W))
            .padding(6)
            .style(kiri::home_card_style(t))
            .on_press(message),
        title,
    )
}

fn ready_body(tabs: &Tabs, t: Tokens) -> Element<'_, Message> {
    if let Some(home) = tabs.home() {
        let browser = empty_browser(home, true, t);
        return match tabs.open_error() {
            Some(error) => column![text(error).size(13).color(t.muted), browser]
                .spacing(8)
                .height(Length::Fill)
                .into(),
            None => browser,
        };
    }
    let ready = tabs.active();
    let mut panes = row![].spacing(12).height(Length::Fill);
    if ready.pages_open {
        panes = panes.push(pages_panel(ready, t));
    }
    panes = panes.push(page_pane(ready, t));
    if ready.signatures_open {
        panes = panes.push(signatures_panel(ready, t));
    }
    // Status pós-ação ("Enviado para …", "Cópia salva em …", falha ao abrir
    // outra aba): 1 linha no topo. O aviso de disco é outra linha, com ação.
    let restored = matches!(ready.recovery, RecoveryState::Restaurando).then(|| {
        t.lang
            .pick("Marcações restauradas.", "Annotations restored.")
    });
    let status = ready
        .save_status
        .as_deref()
        .or(ready.print_status.as_deref())
        .or(tabs.open_error())
        .or(restored);
    let mut rows: Vec<Element<'_, Message>> = doc_warnings(ready)
        .into_iter()
        .map(|warning| match warning {
            DocWarning::DiskStale => disk_stale_row(ready, t),
            DocWarning::RecoveryFailed => recovery_failed_row(t),
        })
        .collect();
    if let Some(status) = status {
        rows.push(text(status).size(13).color(t.muted).into());
    }
    if rows.is_empty() {
        return panes.into();
    }
    rows.push(panes.into());
    column(rows).spacing(8).height(Length::Fill).into()
}

/// Avisos da faixa do documento, na ordem em que aparecem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DocWarning {
    DiskStale,
    RecoveryFailed,
}

/// Avisos do documento. Disco e recuperação são independentes: cada um aparece
/// sozinho ou junto.
pub(crate) fn doc_warnings(ready: &Ready) -> Vec<DocWarning> {
    let mut warnings = Vec::new();
    if ready.disk_stale() {
        warnings.push(DocWarning::DiskStale);
    }
    if ready.journal_failed {
        warnings.push(DocWarning::RecoveryFailed);
    }
    warnings
}

fn disk_stale_row(ready: &Ready, t: Tokens) -> Element<'static, Message> {
    let mut row = row![text(
        t.lang
            .pick("O arquivo mudou no disco.", "The file changed on disk.")
    )
    .size(13)
    .color(t.muted)]
    .spacing(8)
    .align_y(Alignment::Center);
    if ready.marks_dirty() || ready.note_draft.is_some() {
        row = row.push(
            button(text(t.lang.pick("Salvar cópia", "Save copy")).size(13))
                .padding(Padding::from([4, 10]))
                .style(kiri::hud_ghost_style(t))
                .on_press(Message::SaveCopyRequested),
        );
    }
    row.push(
        button(text(t.lang.pick("Recarregar", "Reload")).size(13))
            .padding(Padding::from([4, 10]))
            .style(kiri::hud_primary_style(t))
            .on_press(Message::ReloadDisk),
    )
    .into()
}

/// Faixa de abas sob a toolbar (issue #40): uma pílula por documento, com o
/// nome e o × dentro. A ativa tem fundo `accent_bg`. Só existe com 2+ abas ou
/// com o Início por cima delas (aí nenhuma fica destacada: clicar volta para ela).
fn tab_strip(tabs: &Tabs, t: Tokens) -> Element<'_, Message> {
    // Padding vertical de 4px: os 9px dos botões-ícone somavam ~35px numa faixa
    // de 30px e o texto sumia.
    let mut strip = row![].spacing(4).align_y(Alignment::Center);
    for (index, doc) in tabs.docs().iter().enumerate() {
        let label = short_name(doc.source.path());
        let active = tabs.home().is_none() && index == tabs.active_index();
        let name = tip_owned(
            t,
            button(text(label).size(13))
                .padding(Padding::from([4, 10]))
                .style(kiri::ibtn_style(t, false))
                .on_press(Message::SelectTab(index)),
            identity_tip(doc.source.path()),
        );
        let close = tip(
            t,
            button(text("×").size(14).color(t.muted))
                .padding(Padding::from([4, 8]))
                .style(kiri::ibtn_style(t, false))
                .on_press(Message::CloseTab(index)),
            if cfg!(target_os = "macos") {
                t.lang.pick("Fechar aba (Cmd+W)", "Close tab (Cmd+W)")
            } else {
                t.lang.pick("Fechar aba (Ctrl+W)", "Close tab (Ctrl+W)")
            },
        );
        // Padding de 1px: o hover dos botões não cobre a moldura.
        strip = strip.push(
            container(row![name, close].align_y(Alignment::Center))
                .padding(1)
                .style(kiri::tab_pill_style(t, active)),
        );
    }
    container(strip)
        .width(Length::Fill)
        .height(Length::Fixed(TAB_STRIP_HEIGHT))
        .padding(Padding::from([0, 8]))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.chrome)),
            ..container::Style::default()
        })
        .into()
}

/// Nome na aba: elipse no meio como a pílula do documento (o fim — onde
/// mora o `.pdf` e a versão — continua legível).
fn short_name(path: &std::path::Path) -> String {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    middle_truncate(&name, 24)
}

/// Painel de navegação (Stitch sidebar): cabeçalho `Navegação`, controle
/// segmentado Páginas | Sumário | Marcações e a lista da aba escolhida.
fn pages_panel(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    let mut col = column![text(t.lang.pick("Navegação", "Navigation"))
        .size(13)
        .color(t.ink)]
    .spacing(8);
    col = col.push(panel_tabs(ready, t));
    col = match ready.panel {
        Some(PanelTab::Outline) if ready.outline.is_some() => col.push(outline_tab(ready, t)),
        Some(PanelTab::Annotations) => col.push(annotations_tab(ready, t)),
        _ => col.push(thumbs_tab(ready, t)),
    };
    container(col.width(Length::Fill).height(Length::Fill))
        .width(Length::Fixed(PAGES_PANEL_W))
        .height(Length::Fill)
        .padding(6)
        .style(kiri::panel_bg_style(t))
        .into()
}

/// Controle segmentado do painel. Sumário só aparece com outline no PDF;
/// Marcações existe sempre, inclusive sem sumário.
fn panel_tabs(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    // Três abas em partes iguais cortavam "Marcações"/"Annotations" (~45px de
    // texto por aba). Cada aba recebe partes pelo tamanho do rótulo: no painel
    // de 216px sobram 200px úteis, ~8,7px por caractere (23 caracteres por
    // conjunto de rótulos, PT e EN).
    let seg = |label: &'static str, active: bool, tab: Option<PanelTab>| {
        button(
            text(label)
                .size(12)
                .wrapping(text::Wrapping::None)
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .width(Length::FillPortion(label.chars().count() as u16))
        .padding(Padding::from([5, 4]))
        .style(kiri::panel_seg_style(t, active))
        .on_press(Message::SetPanelTab(tab))
    };
    let mut tabs = row![seg(
        t.lang.pick("Páginas", "Pages"),
        ready.panel.is_none(),
        None
    )];
    if ready.outline.is_some() {
        tabs = tabs.push(seg(
            t.lang.pick("Sumário", "Outline"),
            ready.panel == Some(PanelTab::Outline),
            Some(PanelTab::Outline),
        ));
    }
    tabs = tabs.push(seg(
        t.lang.pick("Marcações", "Annotations"),
        ready.panel == Some(PanelTab::Annotations),
        Some(PanelTab::Annotations),
    ));
    container(tabs)
        .padding(Padding::from(2))
        .style(kiri::panel_seg_track_style(t))
        .into()
}

/// Aba Sumário: árvore clicável com expandir/colapsar (body-md + indent) e
/// destaque da ativa.
fn outline_tab(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    let active = ready.outline_active();
    let focus = ready.outline_focus();
    let mut col = column![].spacing(8);
    for (path, depth, title, page, has_children) in ready.outline_rows() {
        let is_active = active.as_ref() == Some(&path);
        // O cursor do teclado é a pílula; a página ativa, o texto em accent.
        let is_focus = focus.as_ref() == Some(&path);
        let fold: Element<'_, Message> = if has_children {
            let collapsed = ready.outline_collapsed.contains(&path);
            button(text(if collapsed { "▸" } else { "▾" }).size(11))
                .padding(Padding::from([3, 5]))
                .style(kiri::panel_fold_style(t))
                .on_press(Message::OutlineFold(path.clone()))
                .into()
        } else {
            Space::with_width(Length::Fixed(24.0)).into()
        };
        let label = format!("{} · {}", outline_title(title), page.index() + 1);
        let entry = control_active(
            t,
            button(
                text(label)
                    .size(12)
                    .color(if is_active { t.accent } else { t.ink }),
            )
            .width(Length::Fill)
            .padding(Padding::from([4, 6]))
            .style(kiri::panel_toc_item_style(t, is_active))
            .on_press(Message::OutlineJump(page)),
            is_focus,
        );
        col = col.push(
            row![
                Space::with_width(Length::Fixed(depth as f32 * 12.0)),
                fold,
                entry,
            ]
            .spacing(2)
            .align_y(Alignment::Center),
        );
    }
    scrollable(col)
        .id(pages_scroll_id())
        .on_scroll(|viewport| Message::PagesScrolled(viewport.absolute_offset().y))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Título colapsado em 1 linha e truncado em 24 caracteres: título de
/// PDF com quebra de linha não quebra a linha do painel nem da paleta.
fn outline_title(title: &str) -> String {
    const MAX: usize = 24;
    let one_line = title.split_whitespace().collect::<Vec<_>>().join(" ");
    let end = one_line
        .char_indices()
        .nth(MAX)
        .map(|(i, _)| i)
        .unwrap_or(one_line.len());
    if end < one_line.len() {
        format!("{}…", &one_line[..end])
    } else {
        one_line
    }
}

/// Largura útil de uma linha da lista de marcações, em caracteres (painel de
/// 200px, texto 12). Linha de altura fixa: texto que quebra estouraria o
/// cálculo de rolagem do cursor.
const ANNOT_ROW_CHARS: usize = 30;

/// Aba Marcações: as marcações desta sessão como linhas (página, tipo, trecho,
/// nota), com busca por trecho e nota. Sem marcações, diz como criar uma; não
/// promete anotações externas, que a v1 não importa.
fn annotations_tab(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    let search = text_input(
        t.lang
            .pick("Buscar em trechos e notas", "Search excerpts and notes"),
        ready.annot_query(),
    )
    .on_input(Message::AnnotQuery)
    // iced captura o Enter num text_input focado: sem isto, ↓ + Enter não salta.
    .on_submit(Message::PanelKey(PanelKey::Activate))
    .style(kiri::bar_input_style(t))
    .padding([6, 8])
    .size(12)
    .width(Length::Fill);
    let mut col = column![
        text(
            t.lang
                .pick("Marcações desta sessão", "Marks from this session")
        )
        .size(11)
        .color(t.muted),
        search,
    ]
    .spacing(8);
    if ready.annotations.is_empty() {
        col = col.push(
            text(t.lang.pick(
                "Nenhuma marcação nesta sessão. Selecione um texto e use H, U, S ou N.",
                "No marks in this session. Select text and press H, U, S or N.",
            ))
            .size(12)
            .color(t.muted),
        );
    } else {
        let rows = ready.annotation_rows();
        if rows.is_empty() {
            col = col.push(
                text(t.lang.pick(
                    "Nenhum resultado para a busca.",
                    "No results for this search.",
                ))
                .size(12)
                .color(t.muted),
            );
        } else {
            let focus = ready.annot_focus();
            let mut list = column![].spacing(2);
            for row in &rows {
                list = list.push(annotation_row(t, row, focus == Some(row.id)));
            }
            col = col.push(
                scrollable(list)
                    .id(annotations_scroll_id())
                    .width(Length::Fill)
                    .height(Length::Fill),
            );
        }
    }
    col.width(Length::Fill).height(Length::Fill).into()
}

/// Uma linha da lista: página e tipo, trecho e nota, cada um numa linha de
/// texto. Altura fixa (`ANNOT_ROW`), que a rolagem do cursor conta.
fn annotation_row(t: Tokens, row: &AnnotationRow, focus: bool) -> Element<'static, Message> {
    let excerpt: String = match &row.excerpt {
        Some(excerpt) => clip_one_line(excerpt, ANNOT_ROW_CHARS),
        None => t
            .lang
            .pick("trecho indisponível", "excerpt unavailable")
            .into(),
    };
    let mut lines = column![
        text(format!(
            "{} · {}",
            annot_kind_label(row.kind, t.lang),
            row.page.index() + 1
        ))
        .size(12)
        .color(if focus { t.accent } else { t.muted })
        .wrapping(text::Wrapping::None),
        text(excerpt)
            .size(12)
            .color(t.ink)
            .wrapping(text::Wrapping::None),
    ]
    .width(Length::Fill);
    if let Some(note) = &row.note {
        lines = lines.push(
            text(clip_one_line(note, ANNOT_ROW_CHARS))
                .size(12)
                .color(t.muted)
                .wrapping(text::Wrapping::None),
        );
    }
    button(lines)
        .width(Length::Fill)
        .height(Length::Fixed(ANNOT_ROW - 2.0))
        .padding(Padding::from([4, 6]))
        .style(kiri::panel_toc_item_style(t, focus))
        .on_press(Message::AnnotActivate(row.id))
        .into()
}

fn annot_kind_label(kind: AnnotKind, lang: Lang) -> &'static str {
    match kind {
        AnnotKind::Highlight => lang.pick("Destaque", "Highlight"),
        AnnotKind::Underline => lang.pick("Sublinhado", "Underline"),
        AnnotKind::Strikeout => lang.pick("Riscado", "Strikeout"),
        AnnotKind::Note => lang.pick("Nota", "Note"),
    }
}

/// Uma linha, no máximo `max` caracteres, com "…" quando corta.
fn clip_one_line(text: &str, max: usize) -> String {
    let one_line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    match one_line.char_indices().nth(max) {
        Some((end, _)) => format!("{}…", &one_line[..end]),
        None => one_line,
    }
}
fn thumbs_tab(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    // Janela virtualizada do remoto: só monta as miniaturas visíveis.
    let window = ready.thumb_page_window();
    let start = window.first().map(|page| page.index()).unwrap_or(0);
    let end = window.last().map(|page| page.index() + 1).unwrap_or(0);
    let mut col = column![].spacing(8);
    if start > 0 {
        col = col.push(Space::with_height(Length::Fixed(start as f32 * THUMB_ROW)));
    }
    for i in start..end {
        let page = PageNo::from_index(i);
        let preview: Element<'_, Message> = match ready.thumb_surface(page) {
            Some(surface) => image(surface.image.clone())
                .width(Length::Fixed(120.0))
                .into(),
            None => container(Space::new(Length::Fixed(120.0), Length::Fixed(150.0)))
                .style(kiri::panel_thumb_frame_style(t))
                .into(),
        };
        let active = ready.visible == page;
        col = col.push(
            control_active(
                t,
                button(
                    column![
                        // Halo 2px `accent` na ativa (transparente fora) + gap 2px —
                        // borda sempre ocupando o mesmo espaço, sem shift de layout.
                        container(container(preview).style(kiri::panel_thumb_frame_style(t)),)
                            .padding(2)
                            .style(kiri::panel_thumb_style(t, active)),
                        text(format!("{} {}", t.lang.pick("Pág.", "Page"), i + 1))
                            .size(11)
                            .font(iced::Font::MONOSPACE)
                            .color(if active { t.accent } else { t.muted }),
                    ]
                    .spacing(4)
                    .align_x(Alignment::Center),
                )
                .on_press(Message::Nav(NavCmd::GoTo(page))),
                active,
            )
            .width(Length::Fill),
        );
    }
    let remaining = ready.page_count().saturating_sub(end);
    if remaining > 0 {
        col = col.push(Space::with_height(Length::Fixed(
            remaining as f32 * THUMB_ROW,
        )));
    }
    scrollable(col)
        .id(pages_scroll_id())
        .on_scroll(|viewport| Message::PagesScrolled(viewport.absolute_offset().y))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn page_pane(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    match ready.view_mode {
        ViewMode::Single => single_pane(ready, t),
        ViewMode::Continuous => continuous_pane(ready, t),
    }
}

fn single_pane(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    let sw = ready.sheet_width(ready.visible);
    let stage_w = ready.doc_stage_width(ready.visible);
    let rotated = ready.rotated_media(ready.visible);
    let sh = sw * rotated.height.max(1.0) / rotated.width.max(1.0);
    let page_view: Element<'_, Message> = match ready.visible_surface() {
        Some(surface) => with_marks(
            ready,
            ready.visible,
            sw,
            image(surface.image.clone())
                .width(Length::Fixed(sw))
                .height(Length::Fixed(sh))
                .into(),
            t,
        ),
        // Sem bitmap: caixa do tamanho da folha (mesma geometria do
        // placeholder do contínuo) — sem salto de layout quando chega.
        None => {
            let msg = if ready.visible_render_failed() {
                t.lang.pick(
                    "Não foi possível renderizar esta página.",
                    "Could not render this page.",
                )
            } else {
                t.lang.pick("Renderizando página…", "Rendering page…")
            };
            container(text(msg).size(13).color(t.muted))
                .width(Length::Fixed(sw))
                .height(Length::Fixed(sh))
                .center_x(Length::Fixed(sw))
                .align_y(Alignment::Center)
                .into()
        }
    };

    scrollable(
        container(
            // A moldura abraça a folha: só ela escala com o zoom, o palco
            // fora segue estável.
            container(page_view)
                .center_x(Length::Shrink)
                .padding(0)
                .style(kiri::page_frame(&t)),
        )
        .center_x(Length::Fixed(stage_w))
        .padding(Padding {
            top: DOC_PAD_TOP,
            right: DOC_PAD_X,
            bottom: DOC_PAD_BOTTOM,
            left: DOC_PAD_X,
        })
        .style(move |_| container::Style {
            // Canvas do documento: mesmo fundo da janela (Stitch Layer 0).
            background: Some(Background::Color(t.bg)),
            ..container::Style::default()
        }),
    )
    .id(doc_scroll_id())
    .direction(scrollable::Direction::Both {
        vertical: scrollable::Scrollbar::default(),
        horizontal: scrollable::Scrollbar::default(),
    })
    // A rolagem do modo página também entra no estado: o post-it aberto segue
    // a folha (a âncora é janela, a rolagem é o delta).
    .on_scroll(|viewport| {
        let offset = viewport.absolute_offset();
        Message::DocScrolled {
            x: offset.x,
            y: offset.y,
        }
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Rolagem contínua: coluna de células com a mesma estrutura da página única;
/// fora da janela, placeholders de altura exata (sem montar bitmaps).
fn continuous_pane(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    let total = ready.page_count();
    let (start, end) = ready.doc_window();
    let mut col = column![]
        .spacing(DOC_GAP)
        .width(Length::Fixed(ready.doc_stage_max_width()));
    if start > 0 {
        // Offset acumulado menos um gap (o spacing da coluna já conta um).
        let h = (ready.page_offset(PageNo::from_index(start)) - DOC_GAP).max(0.0);
        col = col.push(Space::with_height(Length::Fixed(h)));
    }
    for i in start..end {
        col = col.push(doc_cell(ready, PageNo::from_index(i), t));
    }
    if end < total {
        let h = (ready.doc_total_height() - ready.page_offset(PageNo::from_index(end)) - DOC_GAP)
            .max(0.0);
        col = col.push(Space::with_height(Length::Fixed(h)));
    }
    scrollable(col)
        .id(doc_scroll_id())
        .direction(scrollable::Direction::Both {
            vertical: scrollable::Scrollbar::default(),
            horizontal: scrollable::Scrollbar::default(),
        })
        .on_scroll(|viewport| {
            let offset = viewport.absolute_offset();
            Message::DocScrolled {
                x: offset.x,
                y: offset.y,
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn doc_cell(ready: &Ready, page: PageNo, t: Tokens) -> Element<'_, Message> {
    let sw = ready.sheet_width(page);
    let rotated = ready.rotated_media(page);
    let sh = sw * rotated.height.max(1.0) / rotated.width.max(1.0);
    let inner: Element<'_, Message> = match ready.page_surface(page) {
        Some(surface) => with_marks(
            ready,
            page,
            sw,
            image(surface.image.clone())
                .width(Length::Fixed(sw))
                .height(Length::Fixed(sh))
                .into(),
            t,
        ),
        None => {
            let h = (ready.doc_cell_height(page, sw) - DOC_PAD_TOP - DOC_PAD_BOTTOM).max(1.0);
            container(
                text(t.lang.pick("Renderizando página…", "Rendering page…"))
                    .size(13)
                    .color(t.muted),
            )
            .width(Length::Fixed(sw))
            .height(Length::Fixed(h))
            .center_x(Length::Fixed(sw))
            .align_y(Alignment::Center)
            .into()
        }
    };
    container(
        // A moldura abraça a folha: só ela escala com o zoom, o palco fora
        // segue estável.
        container(inner)
            .center_x(Length::Shrink)
            .padding(0)
            .style(kiri::page_frame(&t)),
    )
    .width(Length::Fill)
    .center_x(Length::Fill)
    .padding(Padding {
        top: DOC_PAD_TOP,
        right: DOC_PAD_X,
        bottom: DOC_PAD_BOTTOM,
        left: DOC_PAD_X,
    })
    .style(move |_| container::Style {
        // Canvas do documento: mesmo fundo da janela (Stitch Layer 0).
        background: Some(Background::Color(t.bg)),
        ..container::Style::default()
    })
    .into()
}

/// Retângulo desenhável (px CSS, espaço exibido); `kind: None` = seleção ativa.
/// `marker: true` = quadrado compacto de nota (não segue o traço do kind).
/// `search: true` = hit da busca (amarelo; `current` = âmbar do hit atual).
struct DrawMark {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    kind: Option<AnnotKind>,
    marker: bool,
    /// Candidato do arrasto de nota: contorno claro pontilhado (o original
    /// segue sólido até o soltar).
    ghost: bool,
    selected: bool,
    search: bool,
    current: bool,
}

/// Camada transparente sobre a folha (issue #30): desenha marcações/seleção
/// e traduz o arrasto em PointerDown/Move/Up (a seleção não tinha emissor).
struct MarkLayer {
    page: PageNo,
    media: MediaBox,
    rotation: u8,
    size: Size,
    marks: Vec<DrawMark>,
    mark: Color,
    mark_current: Color,
}

#[derive(Default)]
struct MarkDrag {
    pressing: bool,
}

impl Program<Message> for MarkLayer {
    type State = MarkDrag;

    fn update(
        &self,
        state: &mut Self::State,
        event: CanvasEvent,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (CanvasStatus, Option<Message>) {
        let page_pt = |p: Point| {
            page_pt_at(
                [p.x, p.y],
                self.media,
                self.rotation,
                self.size.width,
                self.size.height,
            )
        };
        match event {
            CanvasEvent::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                match cursor.position_in(bounds) {
                    Some(at) => {
                        state.pressing = true;
                        (
                            CanvasStatus::Captured,
                            Some(Message::PointerDown {
                                page: self.page,
                                page_pt: page_pt(at),
                                // Canto da folha na janela: a sessão não vê o
                                // layout, e o post-it nasce ancorado nele.
                                sheet: [bounds.x, bounds.y],
                            }),
                        )
                    }
                    None => (CanvasStatus::Ignored, None),
                }
            }
            CanvasEvent::Mouse(mouse::Event::CursorMoved { .. }) => {
                if !state.pressing {
                    return (CanvasStatus::Ignored, None);
                }
                // Fora da folha, fixa na borda (o arrasto continua valendo).
                let at = cursor.position_in(bounds).unwrap_or_else(|| {
                    let p = cursor.position().unwrap_or(Point::new(0.0, 0.0));
                    Point::new(
                        (p.x - bounds.x).clamp(0.0, bounds.width),
                        (p.y - bounds.y).clamp(0.0, bounds.height),
                    )
                });
                (
                    CanvasStatus::Ignored,
                    Some(Message::PointerMove {
                        page: self.page,
                        page_pt: page_pt(at),
                    }),
                )
            }
            CanvasEvent::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if !state.pressing {
                    return (CanvasStatus::Ignored, None);
                }
                state.pressing = false;
                match cursor.position_in(bounds) {
                    Some(at) => (
                        CanvasStatus::Ignored,
                        Some(Message::PointerUp {
                            page: self.page,
                            page_pt: page_pt(at),
                        }),
                    ),
                    // Solta fora da folha: cancela (sem Up, a âncora do press
                    // e o arrasto de nota ficariam presos).
                    None => (CanvasStatus::Ignored, Some(Message::DragCancelled)),
                }
            }
            _ => (CanvasStatus::Ignored, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, self.size);
        // Traço do ghost: o mesmo para todos (fatia estática).
        const GHOST_DASH: [f32; 2] = [6.0, 4.0];
        for m in &self.marks {
            let rect = Path::rectangle(Point::new(m.x, m.y), Size::new(m.w, m.h));
            // Ghost do arrasto de nota: contorno claro pontilhado — a nota
            // fica onde está até o soltar.
            if m.ghost {
                frame.fill(&rect, Color::from_rgba(0.99, 0.80, 0.20, 0.25));
                frame.stroke(
                    &rect,
                    Stroke {
                        style: Style::Solid(STICKY),
                        width: 1.5,
                        line_dash: LineDash {
                            segments: &GHOST_DASH,
                            offset: 0,
                        },
                        ..Stroke::default()
                    },
                );
                continue;
            }
            // Nota: amarelo fixo — a folha é sempre branca, o tema não se aplica.
            if m.marker {
                frame.fill(&rect, STICKY);
                frame.stroke(
                    &rect,
                    Stroke {
                        style: Style::Solid(STICKY_INK),
                        width: 1.0,
                        ..Stroke::default()
                    },
                );
                if m.selected {
                    stroke_selected(&mut frame, &rect);
                }
                continue;
            }
            // Busca: amarelo claro nos hits, âmbar no atual (#43).
            if m.search {
                frame.fill(
                    &rect,
                    if m.current {
                        self.mark_current
                    } else {
                        self.mark
                    },
                );
                continue;
            }
            match m.kind {
                None => frame.fill(&rect, Color::from_rgba(0.25, 0.45, 1.0, 0.30)),
                Some(AnnotKind::Highlight) => {
                    frame.fill(&rect, Color::from_rgba(1.0, 0.85, 0.25, 0.45))
                }
                Some(AnnotKind::Underline) => frame.stroke(
                    &Path::line(
                        Point::new(m.x, m.y + m.h - 1.0),
                        Point::new(m.x + m.w, m.y + m.h - 1.0),
                    ),
                    Stroke {
                        style: Style::Solid(Color::from_rgb(0.1, 0.35, 0.9)),
                        width: 2.0,
                        ..Stroke::default()
                    },
                ),
                // Nota: sublinhado sutil só para ancorar o trecho na folha.
                Some(AnnotKind::Note) => frame.stroke(
                    &Path::line(
                        Point::new(m.x, m.y + m.h - 1.0),
                        Point::new(m.x + m.w, m.y + m.h - 1.0),
                    ),
                    Stroke {
                        style: Style::Solid(Color::from_rgba(0.85, 0.62, 0.05, 0.75)),
                        width: 1.5,
                        ..Stroke::default()
                    },
                ),
                Some(AnnotKind::Strikeout) => frame.stroke(
                    &Path::line(
                        Point::new(m.x, m.y + m.h * 0.5),
                        Point::new(m.x + m.w, m.y + m.h * 0.5),
                    ),
                    Stroke {
                        style: Style::Solid(Color::from_rgb(0.85, 0.15, 0.2)),
                        width: 2.0,
                        ..Stroke::default()
                    },
                ),
            }
            if m.selected {
                stroke_selected(&mut frame, &rect);
            }
        }
        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::Text
    }
}

/// Foca o editor do post-it recém-aberto (autofocus): o `text_editor` do iced
/// não expõe `Id`, então `operation::focus` não o alcança — aqui o alvo é o
/// único campo focável sem id (e os demais perdem o foco, como no `focus`).
/// A operação roda no quadro seguinte, sobre a árvore que já tem o post-it.
pub fn focus_postit() -> Task<Message> {
    struct FocusEditor;

    impl Operation<Message> for FocusEditor {
        fn focusable(&mut self, state: &mut dyn Focusable, id: Option<&WidgetId>) {
            if id.is_none() {
                state.focus();
            } else {
                state.unfocus();
            }
        }

        fn container(
            &mut self,
            _id: Option<&WidgetId>,
            _bounds: Rectangle,
            operate_on_children: &mut dyn FnMut(&mut dyn Operation<Message>),
        ) {
            operate_on_children(self);
        }
    }

    operate(FocusEditor)
}

/// Folha com overlay: imagem em tamanho fixo + canvas transparente da mesma
/// medida (view e canvas dividem `cw`/`ch`, então o mapeamento alinha).
fn with_marks<'a>(
    ready: &Ready,
    page: PageNo,
    cw: f32,
    sheet: Element<'a, Message>,
    t: Tokens,
) -> Element<'a, Message> {
    let media = ready.media(page);
    let rotated = ready.rotated_media(page);
    let ch = cw * rotated.height.max(1.0) / rotated.width.max(1.0);
    let size = Size::new(cw, ch);
    let mut marks = Vec::new();
    // Busca (#43): hits amarelos + atual âmbar, sob seleção e marcas.
    let current = ready.search.current();
    for (i, hit) in ready.search.hits().iter().enumerate() {
        if hit.page != page {
            continue;
        }
        let [x, y, w, h] = display_rect(hit.quad, media, ready.view_rotation, cw, ch);
        marks.push(DrawMark {
            x,
            y,
            w,
            h,
            kind: None,
            marker: false,
            ghost: false,
            selected: false,
            search: true,
            current: Some(i) == current,
        });
    }
    if let Some((sel_page, quads)) = ready.selection_quads() {
        if sel_page == page {
            for quad in &quads {
                let [x, y, w, h] = display_rect(*quad, media, ready.view_rotation, cw, ch);
                marks.push(DrawMark {
                    x,
                    y,
                    w,
                    h,
                    kind: None,
                    marker: false,
                    ghost: false,
                    selected: false,
                    search: false,
                    current: false,
                });
            }
        }
    }
    for a in ready.annotations.iter().filter(|a| a.page == page) {
        let selected = ready.selected_annot() == Some(a.id);
        for (i, quad) in a.quads.iter().enumerate() {
            let [x, y, w, h] = display_rect(*quad, media, ready.view_rotation, cw, ch);
            marks.push(DrawMark {
                x,
                y,
                w,
                h,
                kind: Some(a.kind),
                marker: false,
                ghost: false,
                selected,
                search: false,
                current: false,
            });
            // Nota: marcador compacto — na origem do primeiro quad ou onde o
            // arrasto o deixou (`Annotation::marker`); o trecho não se move.
            if a.kind == AnnotKind::Note && i == 0 {
                let side = marker_side(&a.quads, media, ready.view_rotation, cw, ch);
                let [mx, my] = display_pt(a.marker_pt(), media, ready.view_rotation, cw, ch);
                marks.push(DrawMark {
                    x: mx,
                    y: my,
                    w: side,
                    h: side,
                    kind: Some(AnnotKind::Note),
                    marker: true,
                    ghost: false,
                    selected,
                    search: false,
                    current: false,
                });
            }
        }
    }
    // Marcador arrastado: ghost = só o ícone, em contorno pontilhado claro, na
    // posição candidata (o marcador sólido fica onde está até o soltar).
    if let Some((id, marker_pt)) = ready.note_drag_ghost(page) {
        if let Some(note) = ready.annotations.iter().find(|a| a.id == id) {
            let side = marker_side(&note.quads, media, ready.view_rotation, cw, ch);
            let [x, y] = display_pt(marker_pt, media, ready.view_rotation, cw, ch);
            marks.push(DrawMark {
                x,
                y,
                w: side,
                h: side,
                kind: Some(AnnotKind::Note),
                marker: true,
                ghost: true,
                selected: false,
                search: false,
                current: false,
            });
        }
    }
    let layer = MarkLayer {
        page,
        media,
        rotation: ready.view_rotation,
        size,
        marks,
        mark: t.mark,
        mark_current: t.mark_current,
    };
    stack![
        sheet,
        Canvas::new(layer)
            .width(Length::Fixed(cw))
            .height(Length::Fixed(ch)),
    ]
    .into()
}

fn stroke_selected(frame: &mut Frame, rect: &Path) {
    frame.stroke(
        rect,
        Stroke {
            style: Style::Solid(Color::from_rgb(0.15, 0.45, 0.95)),
            width: 1.5,
            ..Stroke::default()
        },
    );
}

fn signatures_panel(ready: &Ready, t: Tokens) -> Element<'_, Message> {
    let mut col = column![section_title(t.lang.pick("Assinaturas", "Signatures"), t)].spacing(8);
    if ready.signatures.signatures.is_empty() {
        col = col.push(
            text(t.lang.pick(
                "Nenhuma assinatura neste arquivo.",
                "No signatures in this file.",
            ))
            .size(12)
            .color(t.muted),
        );
    } else {
        for sig in &ready.signatures.signatures {
            let name = signer_title(sig, t.lang);
            let mut card = column![text(name).size(13).color(t.ink)].spacing(8);
            for check in sig_rows(sig, t.lang) {
                card = card.push(
                    column![
                        text(check.title).size(10).color(t.muted),
                        row![
                            status_dot(kiri::check_color(t, check.severity)),
                            text(check.text).size(12).color(t.ink).width(Length::Fill),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Start),
                    ]
                    .spacing(2),
                );
            }
            for fact in sig_facts(sig, t.lang) {
                card = card.push(text(fact).size(11).color(t.muted));
            }
            col = col.push(
                container(card)
                    .width(Length::Fill)
                    .padding(12)
                    .style(kiri::panel_card_style(t)),
            );
        }
    }
    container(scrollable(col).height(Length::Fill))
        .width(Length::Fixed(SIG_PANEL_W))
        .height(Length::Fill)
        .padding(6)
        .style(kiri::panel_bg_style(t))
        .into()
}

/// Ação da versão nova: instalar pelo app no macOS e no Windows; nos demais,
/// a página da release.
fn update_action(t: Tokens, tag: &str) -> Element<'static, Message> {
    if cfg!(any(target_os = "macos", target_os = "windows")) {
        menu_item(
            t,
            "save",
            t.lang.pick("Atualizar agora", "Update now"),
            Message::InstallUpdate(tag.to_string()),
            false,
        )
    } else {
        menu_item(
            t,
            "file-text",
            t.lang
                .pick("Baixar na página da release", "Download from release page"),
            Message::OpenReleasePage(tag.to_string()),
            false,
        )
    }
}

/// Título de seção de painel: label-caps 10px, maiúsculas, `muted`.
fn section_title(label: &'static str, t: Tokens) -> Element<'static, Message> {
    text(label.to_uppercase()).size(10).color(t.muted).into()
}

/// Dot 8px de status (cartões de assinatura).
fn status_dot(color: Color) -> Element<'static, Message> {
    container(Space::with_width(Length::Fixed(8.0)))
        .width(Length::Fixed(8.0))
        .height(Length::Fixed(8.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(color)),
            border: Border {
                radius: 99.0.into(),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .into()
}

/// Dimensões independentes de uma assinatura. Cada uma sai de um check que o
/// `tsuro-sign` executou; sem resultado, fica "não verificada".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContentCheck {
    Matches,
    /// `DocumentModified` com cobertura total. O mesmo estado vem de um CMS sem
    /// messageDigest (tsuro-sign #161), então o rótulo não afirma adulteração.
    Altered,
    /// `DocumentModified` com cobertura parcial: o engine não separa alteração
    /// de bytes posteriores, então o rótulo não afirma nenhum dos dois.
    AlteredOrAppended,
    NotVerified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoverageCheck {
    WholeFile,
    PartialFile,
    NotVerified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CryptoCheck {
    Passed,
    UnsupportedDigest,
    /// tsuro-sign sobrescreve o status ao achar bytes fora do intervalo, então o
    /// resultado RSA original some: não dá para afirmar nem aprovação nem falha.
    UnavailablePartialCoverage,
    NotVerified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClockCheck {
    WithinValidity,
    Expired,
    NotYetValid,
    NotVerified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SigChecks {
    content: ContentCheck,
    coverage: CoverageCheck,
    crypto: CryptoCheck,
    clock: ClockCheck,
}

/// Dimensões a partir dos campos de `SignatureInfo`, na ordem de `verify_cms`.
/// `status` mistura etapas, então cada dimensão lê só o que a etapa garante.
fn sig_checks(sig: &SignatureInfo) -> SigChecks {
    use SignatureStatus as S;
    let digest = sig.digest_algorithm.as_deref();
    // Estes status só existem depois de o digest SHA-256 e a RSA baterem.
    let sha256_passed = digest == Some("SHA-256")
        && matches!(
            sig.status,
            S::Valid | S::IntactButUntrusted | S::CertificateExpired | S::CertificateNotYetValid
        );
    let sha1_matched =
        sig.status == S::Unsupported && digest == Some("SHA-1") && sig.covers_whole_document;
    let content = if sig.byte_range.is_none() {
        ContentCheck::NotVerified
    } else if sig.status == S::DocumentModified {
        if sig.covers_whole_document {
            ContentCheck::Altered
        } else {
            ContentCheck::AlteredOrAppended
        }
    } else if sha256_passed || sha1_matched {
        ContentCheck::Matches
    } else {
        ContentCheck::NotVerified
    };
    let coverage = match (sig.byte_range, sig.covers_whole_document) {
        (None, _) => CoverageCheck::NotVerified,
        (Some(_), true) => CoverageCheck::WholeFile,
        (Some(_), false) => CoverageCheck::PartialFile,
    };
    let crypto = if sha256_passed {
        CryptoCheck::Passed
    } else if sig.status == S::DocumentModified
        && sig.byte_range.is_some()
        && !sig.covers_whole_document
    {
        // Sem ByteRange a RSA nunca roda. Com ele, pode ter rodado e tsuro-sign
        // sobrescreveu o status com bytes fora do intervalo.
        CryptoCheck::UnavailablePartialCoverage
    } else if matches!(sig.status, S::Unsupported | S::IntactButUntrusted)
        && digest.is_some_and(|alg| alg != "SHA-256")
    {
        CryptoCheck::UnsupportedDigest
    } else {
        CryptoCheck::NotVerified
    };
    let clock = match sig.status {
        S::CertificateExpired => ClockCheck::Expired,
        S::CertificateNotYetValid => ClockCheck::NotYetValid,
        S::Valid | S::IntactButUntrusted if sha256_passed && sig.certificate.is_some() => {
            ClockCheck::WithinValidity
        }
        _ => ClockCheck::NotVerified,
    };
    SigChecks {
        content,
        coverage,
        crypto,
        clock,
    }
}

impl ContentCheck {
    fn label(self, lang: Lang) -> &'static str {
        match self {
            Self::Matches => {
                lang.pick("Confere com o resumo assinado", "Matches the signed digest")
            }
            Self::Altered => lang.pick(
                "Não confere com o resumo assinado, ou o resumo não pôde ser lido",
                "Does not match the signed digest, or the digest could not be read",
            ),
            Self::AlteredOrAppended => lang.pick(
                "Alterado, ou com bytes depois da assinatura: não dá para separar",
                "Changed, or bytes added after signing: cannot tell apart",
            ),
            Self::NotVerified => lang.pick("Não verificado", "Not verified"),
        }
    }

    fn severity(self) -> CheckSeverity {
        match self {
            Self::Matches => CheckSeverity::Passed,
            Self::Altered => CheckSeverity::Failure,
            Self::AlteredOrAppended | Self::NotVerified => CheckSeverity::Attention,
        }
    }
}

impl CoverageCheck {
    fn label(self, lang: Lang) -> &'static str {
        match self {
            Self::WholeFile => lang.pick(
                "O intervalo assinado cobre o arquivo inteiro",
                "The signed range covers the whole file",
            ),
            Self::PartialFile => lang.pick(
                "O intervalo assinado não cobre o arquivo inteiro",
                "The signed range does not cover the whole file",
            ),
            Self::NotVerified => lang.pick(
                "Não verificado: intervalo de bytes ausente",
                "Not verified: byte range missing",
            ),
        }
    }

    fn severity(self) -> CheckSeverity {
        match self {
            Self::WholeFile => CheckSeverity::Passed,
            Self::PartialFile | Self::NotVerified => CheckSeverity::Attention,
        }
    }
}

impl CryptoCheck {
    fn label(self, lang: Lang) -> &'static str {
        match self {
            Self::Passed => lang.pick(
                "Confere com a chave pública do certificado no PDF",
                "Matches the public key of the certificate in the PDF",
            ),
            Self::UnsupportedDigest => lang.pick(
                "Não verificada: algoritmo de resumo sem suporte",
                "Not verified: digest algorithm not supported",
            ),
            Self::UnavailablePartialCoverage => lang.pick(
                "Não verificada: resultado indisponível quando o intervalo não cobre o arquivo inteiro",
                "Not verified: result unavailable when the signed range does not cover the whole file",
            ),
            Self::NotVerified => lang.pick(
                "Não verificada: a checagem não passou ou não pôde ser feita",
                "Not verified: the check did not pass or could not run",
            ),
        }
    }

    fn severity(self) -> CheckSeverity {
        match self {
            Self::Passed => CheckSeverity::Passed,
            Self::UnsupportedDigest | Self::UnavailablePartialCoverage | Self::NotVerified => {
                CheckSeverity::Attention
            }
        }
    }
}

impl ClockCheck {
    fn label(self, lang: Lang) -> &'static str {
        match self {
            Self::WithinValidity => lang.pick(
                "Dentro do prazo; não prova a data da assinatura",
                "Within validity; does not prove the signing date",
            ),
            Self::Expired => lang.pick(
                "Vencido neste relógio; não mostra se valia na assinatura",
                "Expired on this clock; does not show whether it was valid when signed",
            ),
            Self::NotYetValid => lang.pick(
                "Ainda não válido neste relógio",
                "Not yet valid on this clock",
            ),
            Self::NotVerified => lang.pick("Não verificado", "Not verified"),
        }
    }

    fn severity(self) -> CheckSeverity {
        match self {
            Self::WithinValidity => CheckSeverity::Passed,
            Self::Expired | Self::NotYetValid | Self::NotVerified => CheckSeverity::Attention,
        }
    }
}

/// Linha da ficha: título da dimensão, rótulo do estado e gravidade (cor).
struct CheckRow {
    title: &'static str,
    text: &'static str,
    severity: CheckSeverity,
}

/// Cinco linhas fixas, sem Vec: o dot do escudo consome isto a cada frame.
fn sig_rows(sig: &SignatureInfo, lang: Lang) -> [CheckRow; 5] {
    let checks = sig_checks(sig);
    [
        CheckRow {
            title: lang.pick("Conteúdo assinado", "Signed content"),
            text: checks.content.label(lang),
            severity: checks.content.severity(),
        },
        CheckRow {
            title: lang.pick("Cobertura do arquivo", "File coverage"),
            text: checks.coverage.label(lang),
            severity: checks.coverage.severity(),
        },
        CheckRow {
            title: lang.pick("Assinatura criptográfica", "Cryptographic signature"),
            text: checks.crypto.label(lang),
            severity: checks.crypto.severity(),
        },
        CheckRow {
            title: lang.pick(
                "Certificado no relógio deste computador",
                "Certificate on this computer's clock",
            ),
            text: checks.clock.label(lang),
            severity: checks.clock.severity(),
        },
        // tsuro-sign não confere cadeia nem âncora: a linha é fixa até existir essa checagem.
        CheckRow {
            title: lang.pick("Cadeia de confiança", "Trust chain"),
            text: lang.pick(
                "Não verificada. Sem âncoras de confiança neste app; isso não é confiança pública.",
                "Not verified. No trust anchors in this app; this is not public trust.",
            ),
            severity: CheckSeverity::Attention,
        },
    ]
}

/// De onde vem o nome exibido. Só `Certificate` foi conferido com o certificado;
/// `Declared` é o /Name escrito no PDF, que qualquer um pode preencher.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SignerSource<'a> {
    Certificate(&'a str),
    Declared(&'a str),
    Unknown,
}

fn signer_source(sig: &SignatureInfo) -> SignerSource<'_> {
    if let Some(cn) = sig
        .certificate
        .as_ref()
        .and_then(|c| c.common_name.as_deref())
    {
        SignerSource::Certificate(cn)
    } else if let Some(name) = sig.signer_name.as_deref() {
        SignerSource::Declared(name)
    } else {
        SignerSource::Unknown
    }
}

/// Nome do cartão: o do certificado, senão o declarado no PDF, senão genérico.
fn signer_title(sig: &SignatureInfo, lang: Lang) -> &str {
    match signer_source(sig) {
        SignerSource::Certificate(name) | SignerSource::Declared(name) => name,
        SignerSource::Unknown => lang.pick("Assinante", "Signer"),
    }
}

/// Fatos do certificado e do PDF, mostrados sem o rótulo de verificação.
fn sig_facts(sig: &SignatureInfo, lang: Lang) -> Vec<String> {
    let mut facts = Vec::new();
    if matches!(signer_source(sig), SignerSource::Declared(_)) {
        facts.push(
            lang.pick(
                "Nome declarado no PDF, não conferido com o certificado",
                "Name declared in the PDF, not checked against the certificate",
            )
            .to_string(),
        );
    }
    if sig.certificate.as_ref().is_some_and(|c| c.is_self_signed) {
        facts.push(
            lang.pick(
                "Titular e emissor têm o mesmo nome",
                "Subject and issuer have the same name",
            )
            .to_string(),
        );
    }
    if let Some(time) = &sig.signing_time {
        facts.push(format!(
            "{} {time}",
            lang.pick(
                "Data declarada no PDF, não verificada:",
                "Date declared in the PDF, not verified:",
            )
        ));
    }
    facts
}

/// Trilho da barra indeterminada de abertura (issue #41): 2px, como o
/// progresso de página (`progress`), mas com um segmento que desliza.
const LOADING_BAR_W: f32 = 220.0;
const LOADING_SEG_W: f32 = 72.0;
/// Passos do ciclo do segmento. `Session::Loading.phase` só conta tiques
/// (não é porcentagem — o restante é fechado aqui no `%`).
pub(crate) const LOADING_STEPS: u16 = 12;

/// X do segmento dentro do trilho: 0 → direita, e volta ao início no ciclo
/// seguinte. Sem porcentagem falsa: a fase dá a posição, nunca a fração
/// carregada.
fn sweep_x(phase: u16) -> f32 {
    let step = f32::from(phase % LOADING_STEPS);
    (LOADING_BAR_W - LOADING_SEG_W) * step / f32::from(LOADING_STEPS - 1)
}

/// Barra indeterminada: trilho `surface` 2px com segmento `accent` deslizando.
fn loading_bar(phase: u16, t: Tokens) -> Element<'static, Message> {
    let segment = container(Space::with_width(Length::Fill))
        .width(Length::Fixed(LOADING_SEG_W))
        .height(Length::Fixed(2.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.accent)),
            ..container::Style::default()
        });
    container(row![Space::with_width(Length::Fixed(sweep_x(phase))), segment].spacing(0))
        .width(Length::Fixed(LOADING_BAR_W))
        .height(Length::Fixed(2.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(t.surface)),
            ..container::Style::default()
        })
        .into()
}

/// Nome do arquivo no chrome de abertura/falha (o caminho completo não cabe).
fn source_name(source: &OpenSource) -> String {
    let path = source.path();
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

/// Tela de abertura (issue #41): nome do arquivo + "Abrindo…" + barra
/// indeterminada, no lugar do documento.
fn opening(source: &OpenSource, phase: u16, t: Tokens) -> Element<'_, Message> {
    container(
        column![
            text(source_name(source)).size(16),
            text(t.lang.pick("Abrindo…", "Opening…"))
                .size(13)
                .color(t.muted),
            loading_bar(phase, t),
        ]
        .spacing(12)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

/// Falha ao abrir (issue #41): o erro toma o lugar da barra — nunca spinner
/// preso; a saída é o topo (Início / abrir outro).
fn failed<'a>(source: &'a OpenSource, message: &'a str, t: Tokens) -> Element<'a, Message> {
    container(
        column![
            text(t.lang.pick(
                "Não foi possível abrir o documento",
                "Could not open the document"
            ))
            .size(16),
            text(source_name(source)).size(13).color(t.muted),
            text(message).size(13).color(t.danger),
        ]
        .spacing(12)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

#[cfg(test)]
mod tests {
    use super::{outline_title, sweep_x, LOADING_BAR_W, LOADING_SEG_W, LOADING_STEPS};
    use crate::i18n::Lang;

    #[test]
    fn opening_sweep_stays_inside_the_track() {
        let travel = LOADING_BAR_W - LOADING_SEG_W;
        assert_eq!(sweep_x(0), 0.0);
        assert_eq!(sweep_x(LOADING_STEPS - 1), travel);
        let mut last = -1.0;
        for phase in 0..LOADING_STEPS {
            let x = sweep_x(phase);
            assert!(x >= last, "phase {phase} went backwards");
            assert!(x <= travel, "phase {phase} left the track");
            last = x;
        }
        // Vários ciclos: a fase só conta, o `%` fecha o ciclo.
        assert_eq!(sweep_x(LOADING_STEPS), sweep_x(0));
        assert_eq!(sweep_x(u16::MAX), sweep_x(u16::MAX % LOADING_STEPS));
    }

    #[test]
    fn outline_title_truncates_long_labels() {
        assert_eq!(outline_title("Curto"), "Curto");
        assert_eq!(outline_title(""), "");
        let long = "Cláusula de rescisão contratual e multa por descumprimento";
        let short = outline_title(long);
        assert!(short.ends_with('…'));
        assert_eq!(short.chars().count(), 25);
        // Exatos 24 caracteres passam intactos.
        let exact: String = "a".repeat(24);
        assert_eq!(outline_title(&exact), exact);
    }

    #[test]
    fn outline_title_collapses_whitespace_to_one_line() {
        assert_eq!(outline_title("Capítulo\nquebrado"), "Capítulo quebrado");
        assert_eq!(outline_title("  antes\tdepois  "), "antes depois");
        assert_eq!(outline_title("a\nb\nc"), "a b c");
    }

    #[test]
    fn palette_title_keeps_rows_single_line() {
        use super::{palette_title, PaletteItem};
        use crate::page::PageNo;
        use std::path::PathBuf;
        let outline = PaletteItem::OutlineRow {
            title: "Capítulo com\nquebra e cauda longa demais".into(),
            page: PageNo::first(),
        };
        let shown = palette_title(&outline);
        assert!(!shown.contains('\n'), "outline cru: {shown:?}");
        assert!(shown.ends_with('…'), "outline longo trunca: {shown:?}");
        let recent = PaletteItem::Recent {
            path: PathBuf::from("/tmp/nome\nquebrado.pdf"),
        };
        assert_eq!(palette_title(&recent), "nome quebrado.pdf");
        let hit = PaletteItem::DocHit {
            page: PageNo::first(),
            range: crate::session::TextRange { start: 0, end: 4 },
            excerpt: "trecho limpo".into(),
        };
        assert_eq!(palette_title(&hit), "trecho limpo");
    }

    #[test]
    fn recent_label_shows_parent_and_truncates() {
        use super::recent_label;
        use std::path::Path;
        assert_eq!(
            recent_label(Path::new("/docs/contratos/termo.pdf")),
            "contratos/termo.pdf"
        );
        assert_eq!(recent_label(Path::new("solto.pdf")), "solto.pdf");
        let deep = "/docs/um-nome-de-pasta-bem-longo/outro-nome-longo/arquivo-final.pdf";
        let short = recent_label(Path::new(deep));
        assert!(short.chars().count() <= 30);
        assert!(short.contains('…'));
        assert!(short.ends_with(".pdf"), "elipse no meio, fim intacto");
    }

    #[test]
    fn tab_label_keeps_ending_and_tip_has_path() {
        use super::{identity_tip, short_name};
        use std::path::Path;
        assert_eq!(short_name(Path::new("/a/curto.pdf")), "curto.pdf");
        let long = "Contrato_de_Locacao_Comercial_v12_final_assinado.pdf";
        let label = short_name(Path::new(&format!("/docs/{long}")));
        assert!(label.chars().count() <= 24);
        assert!(label.contains('…'), "elipse no meio, não no fim");
        assert!(label.ends_with(".pdf"), "terminação intacta: {label}");
        let tip = identity_tip(Path::new(&format!("/docs/sub/{long}")));
        assert!(tip.starts_with(long), "nome inteiro primeiro");
        assert!(tip.contains("/docs/sub/"), "caminho distingue homônimos");
    }

    #[test]
    fn search_count_shows_total_then_position() {
        use super::search_count_text;
        use crate::page::{Glyph, PageNo, Quad, TextLayer};
        use crate::search::Search;
        let layer = || TextLayer {
            page: PageNo::first(),
            plain: "a a".into(),
            glyphs: vec![Glyph::new("a a", Quad::from_rect(0.0, 0.0, 10.0, 10.0))],
        };
        let empty = Search::derive("", &[]);
        assert_eq!(search_count_text(&empty, false, Lang::Pt), None);
        assert_eq!(search_count_text(&empty, true, Lang::Pt), None);
        let none = Search::derive("zzz", &[Some(layer())]);
        assert_eq!(
            search_count_text(&none, false, Lang::Pt).as_deref(),
            Some("0")
        );
        assert_eq!(
            search_count_text(&none, true, Lang::Pt).as_deref(),
            Some("0…")
        );
        let mut two = Search::derive("a", &[Some(layer())]);
        assert_eq!(
            search_count_text(&two, false, Lang::Pt).as_deref(),
            Some("2")
        );
        two.step(1);
        assert_eq!(
            search_count_text(&two, false, Lang::Pt).as_deref(),
            Some("1 de 2")
        );
        assert_eq!(
            search_count_text(&two, true, Lang::Pt).as_deref(),
            Some("1 de 2…")
        );
        two.step(1);
        assert_eq!(
            search_count_text(&two, false, Lang::Pt).as_deref(),
            Some("2 de 2")
        );
    }
}

#[cfg(test)]
mod signature_checks_tests {
    use super::*;
    use crate::i18n::Lang;
    use std::path::PathBuf;
    use tsuro_sign::CertificateInfo;

    fn sig(
        status: SignatureStatus,
        digest: Option<&str>,
        covers: bool,
        byte_range: bool,
        certificate: bool,
    ) -> SignatureInfo {
        SignatureInfo {
            field_name: None,
            signer_name: None,
            reason: None,
            location: None,
            contact_info: None,
            signing_time: None,
            filter: None,
            sub_filter: None,
            byte_range: byte_range.then_some([0, 4, 8, 4]),
            covers_whole_document: covers,
            status,
            status_detail: String::new(),
            certificate: certificate.then(|| CertificateInfo {
                subject: "CN=Maria".into(),
                common_name: Some("Maria".into()),
                organization: None,
                issuer: "CN=Autoridade de teste".into(),
                serial: "1".into(),
                not_before: "2020-01-01".into(),
                not_after: "2030-01-01".into(),
                is_self_signed: false,
            }),
            digest_algorithm: digest.map(str::to_string),
            signature_algorithm: Some("RSA".into()),
        }
    }

    fn pt_en(label: impl Fn(Lang) -> &'static str) -> (&'static str, &'static str) {
        (label(Lang::Pt), label(Lang::En))
    }

    #[test]
    fn intact_sha256_signature_passes_each_check_the_engine_ran() {
        let s = sig(
            SignatureStatus::IntactButUntrusted,
            Some("SHA-256"),
            true,
            true,
            true,
        );
        assert_eq!(
            sig_checks(&s),
            SigChecks {
                content: ContentCheck::Matches,
                coverage: CoverageCheck::WholeFile,
                crypto: CryptoCheck::Passed,
                clock: ClockCheck::WithinValidity,
            }
        );
    }

    #[test]
    fn certificate_dates_only_change_the_clock_row() {
        for (status, clock) in [
            (SignatureStatus::CertificateExpired, ClockCheck::Expired),
            (
                SignatureStatus::CertificateNotYetValid,
                ClockCheck::NotYetValid,
            ),
        ] {
            let s = sig(status, Some("SHA-256"), true, true, true);
            assert_eq!(
                sig_checks(&s),
                SigChecks {
                    content: ContentCheck::Matches,
                    coverage: CoverageCheck::WholeFile,
                    crypto: CryptoCheck::Passed,
                    clock,
                }
            );
        }
    }

    #[test]
    fn whole_file_digest_failure_is_not_claimed_as_tampering() {
        let s = sig(
            SignatureStatus::DocumentModified,
            Some("SHA-256"),
            true,
            true,
            true,
        );
        assert_eq!(
            sig_checks(&s),
            SigChecks {
                content: ContentCheck::Altered,
                coverage: CoverageCheck::WholeFile,
                crypto: CryptoCheck::NotVerified,
                clock: ClockCheck::NotVerified,
            }
        );
    }

    #[test]
    fn bytes_after_the_signed_range_are_not_called_altered() {
        let s = sig(
            SignatureStatus::DocumentModified,
            Some("SHA-256"),
            false,
            true,
            true,
        );
        assert_eq!(
            sig_checks(&s),
            SigChecks {
                content: ContentCheck::AlteredOrAppended,
                coverage: CoverageCheck::PartialFile,
                crypto: CryptoCheck::UnavailablePartialCoverage,
                clock: ClockCheck::NotVerified,
            }
        );
    }

    #[test]
    fn partial_coverage_crypto_row_does_not_claim_a_failed_check() {
        let s = sig(
            SignatureStatus::DocumentModified,
            Some("SHA-256"),
            false,
            true,
            true,
        );
        let pt = sig_rows(&s, Lang::Pt);
        assert_eq!(
            pt[2].text,
            "Não verificada: resultado indisponível quando o intervalo não cobre o arquivo inteiro"
        );
        assert_eq!(pt[2].severity, CheckSeverity::Attention);
        assert_eq!(
            sig_rows(&s, Lang::En)[2].text,
            "Not verified: result unavailable when the signed range does not cover the whole file"
        );
    }

    #[test]
    fn missing_byte_range_is_not_reported_as_altered() {
        let s = sig(SignatureStatus::DocumentModified, None, false, false, true);
        assert_eq!(
            sig_checks(&s),
            SigChecks {
                content: ContentCheck::NotVerified,
                coverage: CoverageCheck::NotVerified,
                crypto: CryptoCheck::NotVerified,
                clock: ClockCheck::NotVerified,
            }
        );
    }

    #[test]
    fn invalid_status_is_not_verified_rather_than_failed() {
        // RSA que não confere e chave ausente (ECDSA, parse) chegam ambos como `Invalid`.
        let rsa_mismatch = sig(SignatureStatus::Invalid, Some("SHA-256"), true, true, true);
        let parse_error = sig(SignatureStatus::Invalid, None, true, true, false);
        for s in [rsa_mismatch, parse_error] {
            assert_eq!(
                sig_checks(&s),
                SigChecks {
                    content: ContentCheck::NotVerified,
                    coverage: CoverageCheck::WholeFile,
                    crypto: CryptoCheck::NotVerified,
                    clock: ClockCheck::NotVerified,
                }
            );
        }
    }

    #[test]
    fn unsupported_digest_is_not_verified_even_when_status_reads_intact() {
        let s = sig(
            SignatureStatus::IntactButUntrusted,
            Some("SHA-512"),
            true,
            true,
            true,
        );
        assert_eq!(
            sig_checks(&s),
            SigChecks {
                content: ContentCheck::NotVerified,
                coverage: CoverageCheck::WholeFile,
                crypto: CryptoCheck::UnsupportedDigest,
                clock: ClockCheck::NotVerified,
            }
        );
    }

    #[test]
    fn sha1_digest_matches_but_rsa_is_not_checked() {
        let s = sig(
            SignatureStatus::Unsupported,
            Some("SHA-1"),
            true,
            true,
            true,
        );
        assert_eq!(
            sig_checks(&s),
            SigChecks {
                content: ContentCheck::Matches,
                coverage: CoverageCheck::WholeFile,
                crypto: CryptoCheck::UnsupportedDigest,
                clock: ClockCheck::NotVerified,
            }
        );
    }

    #[test]
    fn unsupported_status_without_hash_payload_is_not_verified() {
        let s = sig(
            SignatureStatus::Unsupported,
            Some("SHA-256"),
            true,
            true,
            true,
        );
        assert_eq!(
            sig_checks(&s),
            SigChecks {
                content: ContentCheck::NotVerified,
                coverage: CoverageCheck::WholeFile,
                crypto: CryptoCheck::NotVerified,
                clock: ClockCheck::NotVerified,
            }
        );
    }

    #[test]
    fn every_state_has_its_pt_and_en_label() {
        assert_eq!(
            pt_en(|l| ContentCheck::Matches.label(l)),
            ("Confere com o resumo assinado", "Matches the signed digest")
        );
        assert_eq!(
            pt_en(|l| ContentCheck::Altered.label(l)),
            (
                "Não confere com o resumo assinado, ou o resumo não pôde ser lido",
                "Does not match the signed digest, or the digest could not be read"
            )
        );
        assert_eq!(
            pt_en(|l| ContentCheck::AlteredOrAppended.label(l)),
            (
                "Alterado, ou com bytes depois da assinatura: não dá para separar",
                "Changed, or bytes added after signing: cannot tell apart"
            )
        );
        assert_eq!(
            pt_en(|l| ContentCheck::NotVerified.label(l)),
            ("Não verificado", "Not verified")
        );
        assert_eq!(
            pt_en(|l| CoverageCheck::WholeFile.label(l)),
            (
                "O intervalo assinado cobre o arquivo inteiro",
                "The signed range covers the whole file"
            )
        );
        assert_eq!(
            pt_en(|l| CoverageCheck::PartialFile.label(l)),
            (
                "O intervalo assinado não cobre o arquivo inteiro",
                "The signed range does not cover the whole file"
            )
        );
        assert_eq!(
            pt_en(|l| CoverageCheck::NotVerified.label(l)),
            (
                "Não verificado: intervalo de bytes ausente",
                "Not verified: byte range missing"
            )
        );
        assert_eq!(
            pt_en(|l| CryptoCheck::Passed.label(l)),
            (
                "Confere com a chave pública do certificado no PDF",
                "Matches the public key of the certificate in the PDF"
            )
        );
        assert_eq!(
            pt_en(|l| CryptoCheck::UnsupportedDigest.label(l)),
            (
                "Não verificada: algoritmo de resumo sem suporte",
                "Not verified: digest algorithm not supported"
            )
        );
        assert_eq!(
            pt_en(|l| CryptoCheck::NotVerified.label(l)),
            (
                "Não verificada: a checagem não passou ou não pôde ser feita",
                "Not verified: the check did not pass or could not run"
            )
        );
        assert_eq!(
            pt_en(|l| CryptoCheck::UnavailablePartialCoverage.label(l)),
            (
                "Não verificada: resultado indisponível quando o intervalo não cobre o arquivo inteiro",
                "Not verified: result unavailable when the signed range does not cover the whole file"
            )
        );
        assert_eq!(
            pt_en(|l| ClockCheck::WithinValidity.label(l)),
            (
                "Dentro do prazo; não prova a data da assinatura",
                "Within validity; does not prove the signing date"
            )
        );
        assert_eq!(
            pt_en(|l| ClockCheck::Expired.label(l)),
            (
                "Vencido neste relógio; não mostra se valia na assinatura",
                "Expired on this clock; does not show whether it was valid when signed"
            )
        );
        assert_eq!(
            pt_en(|l| ClockCheck::NotYetValid.label(l)),
            (
                "Ainda não válido neste relógio",
                "Not yet valid on this clock"
            )
        );
        assert_eq!(
            pt_en(|l| ClockCheck::NotVerified.label(l)),
            ("Não verificado", "Not verified")
        );
    }

    #[test]
    fn only_a_digest_mismatch_over_the_whole_file_reads_as_failure() {
        let altered = sig(
            SignatureStatus::DocumentModified,
            Some("SHA-256"),
            true,
            true,
            true,
        );
        let appended = sig(
            SignatureStatus::DocumentModified,
            Some("SHA-256"),
            false,
            true,
            true,
        );
        assert_eq!(
            sig_rows(&altered, Lang::Pt)[0].severity,
            CheckSeverity::Failure
        );
        assert_eq!(
            sig_rows(&appended, Lang::Pt)[0].severity,
            CheckSeverity::Attention
        );
    }

    #[test]
    fn trust_chain_row_stays_not_verified_even_for_the_best_signature() {
        let s = sig(
            SignatureStatus::IntactButUntrusted,
            Some("SHA-256"),
            true,
            true,
            true,
        );
        let rows = sig_rows(&s, Lang::Pt);
        assert_eq!(rows[4].title, "Cadeia de confiança");
        assert_eq!(
            rows[4].text,
            "Não verificada. Sem âncoras de confiança neste app; isso não é confiança pública."
        );
        assert_eq!(rows[4].severity, CheckSeverity::Attention);
        // Nenhuma verificação vira "ok" global: o dot do escudo nunca fica verde.
        assert_eq!(
            rows.iter().map(|r| r.severity).max(),
            Some(CheckSeverity::Attention)
        );
    }

    #[test]
    fn facts_say_what_was_declared_and_not_verified() {
        let mut s = sig(
            SignatureStatus::IntactButUntrusted,
            Some("SHA-256"),
            true,
            true,
            true,
        );
        s.signing_time = Some("2024-05-01 10:00:00".into());
        assert_eq!(
            sig_facts(&s, Lang::Pt),
            vec!["Data declarada no PDF, não verificada: 2024-05-01 10:00:00".to_string()]
        );
        if let Some(cert) = s.certificate.as_mut() {
            cert.is_self_signed = true;
        }
        assert_eq!(
            sig_facts(&s, Lang::En)[0],
            "Subject and issuer have the same name"
        );
    }

    #[test]
    fn pdf_name_is_flagged_only_when_the_certificate_has_no_name() {
        let mut s = sig(
            SignatureStatus::IntactButUntrusted,
            Some("SHA-256"),
            true,
            true,
            true,
        );
        s.signer_name = Some("Banco".into());
        assert_eq!(signer_title(&s, Lang::Pt), "Maria");
        assert!(sig_facts(&s, Lang::Pt).is_empty());

        if let Some(cert) = s.certificate.as_mut() {
            cert.common_name = None;
        }
        assert_eq!(signer_title(&s, Lang::Pt), "Banco");
        assert_eq!(
            sig_facts(&s, Lang::Pt),
            vec!["Nome declarado no PDF, não conferido com o certificado".to_string()]
        );
        assert_eq!(
            sig_facts(&s, Lang::En),
            vec!["Name declared in the PDF, not checked against the certificate".to_string()]
        );

        s.certificate = None;
        s.signer_name = None;
        assert_eq!(signer_title(&s, Lang::Pt), "Assinante");
        assert_eq!(signer_title(&s, Lang::En), "Signer");
        assert!(sig_facts(&s, Lang::Pt).is_empty());
    }

    #[test]
    fn signed_contract_fixture_reads_as_separate_checks() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../public/samples/contrato-assinado.pdf");
        let bytes = std::fs::read(&path).expect("contrato-assinado.pdf");
        let analysis = tsuro_sign::analyze_pdf(&bytes).expect("parse");
        let [signature] = analysis.signatures.as_slice() else {
            panic!("one signature expected, got {:?}", analysis.signatures);
        };
        let checks = sig_checks(signature);
        assert_eq!(checks.content, ContentCheck::Matches);
        assert_eq!(checks.coverage, CoverageCheck::WholeFile);
        assert_eq!(checks.crypto, CryptoCheck::Passed);
        // Vencido ou dentro do prazo conforme o relógio do teste; nunca "não verificado".
        assert_ne!(checks.clock, ClockCheck::NotVerified);
        assert_eq!(
            sig_rows(signature, Lang::Pt)[4].severity,
            CheckSeverity::Attention
        );
    }
}
