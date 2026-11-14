//! Barra de menu do sistema no macOS (spec §8: o menu do macOS é a barra do
//! sistema). Windows e Linux continuam com a lista do ⋯ desenhada no iced.
//!
//! A tabela abaixo é a fonte dos itens: cada linha vira um `NSMenuItem` que
//! envia a sua `Message` de volta para a sessão. O app não guarda estado de
//! menu nativo — quem decide o que fazer com a mensagem é o `Session::update`.

use std::cell::RefCell;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use iced::futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use iced::futures::{stream, Stream};
use objc2::rc::Retained;
use objc2::{define_class, msg_send, sel, DefinedClass, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{NSApplication, NSMenu, NSMenuItem};
use objc2_foundation::{NSObjectProtocol, NSString};

use tsuro::i18n::Lang;
use tsuro::prefs::ThemePref;
use tsuro::session::{Message, ViewMode, Zoom};

/// Seção da barra: o título que abre o submenu correspondente.
struct Section {
    title: (&'static str, &'static str),
    items: &'static [Item],
}

/// Linha do submenu. `message` é chamada na montagem, então um item pode
/// carregar dados sem que a tabela vire estado.
struct Item {
    label: (&'static str, &'static str),
    message: fn() -> Message,
    separator_before: bool,
}

const FILE: &[Item] = &[
    Item {
        label: ("Abrir…", "Open…"),
        message: || Message::PickFile,
        separator_before: false,
    },
    Item {
        label: ("Imprimir…", "Print…"),
        message: || Message::OpenPrintDialog,
        separator_before: false,
    },
    Item {
        label: ("Salvar cópia com marcações…", "Save copy with marks…"),
        message: || Message::SaveCopyRequested,
        separator_before: true,
    },
    Item {
        label: ("Fechar documento", "Close document"),
        message: || Message::CloseTabActive,
        separator_before: true,
    },
];

const EDIT: &[Item] = &[
    Item {
        label: ("Copiar seleção", "Copy selection"),
        message: || Message::CopySelection,
        separator_before: false,
    },
    Item {
        label: (
            "Copiar destaques como Markdown",
            "Copy highlights as Markdown",
        ),
        message: || Message::CopyAnnotations,
        separator_before: false,
    },
    Item {
        label: ("Exportar marcações…", "Export annotations…"),
        message: || Message::ExportAnnotations,
        separator_before: true,
    },
    Item {
        label: ("Apagar marcação", "Delete mark"),
        message: || Message::DeleteSelectedAnnot,
        separator_before: true,
    },
    Item {
        label: ("Desfazer marcação", "Undo mark"),
        message: || Message::AnnotUndo,
        separator_before: false,
    },
    Item {
        label: ("Refazer marcação", "Redo mark"),
        message: || Message::AnnotRedo,
        separator_before: false,
    },
];

const VIEW: &[Item] = &[
    Item {
        label: ("Ajustar página inteira", "Fit page"),
        message: || Message::SetZoom(Zoom::Page),
        separator_before: false,
    },
    Item {
        label: ("Girar vista (90°)", "Rotate view (90°)"),
        message: || Message::RotateView,
        separator_before: false,
    },
    Item {
        label: ("Página única", "Single page"),
        message: || Message::SetViewMode(ViewMode::Single),
        separator_before: true,
    },
    Item {
        label: ("Rolagem contínua", "Continuous scroll"),
        message: || Message::SetViewMode(ViewMode::Continuous),
        separator_before: false,
    },
    Item {
        label: ("Tema do sistema", "System theme"),
        message: || Message::SetTheme(ThemePref::System),
        separator_before: true,
    },
    Item {
        label: ("Tema escuro", "Dark theme"),
        message: || Message::SetTheme(ThemePref::Dark),
        separator_before: false,
    },
    Item {
        label: ("Tema claro", "Light theme"),
        message: || Message::SetTheme(ThemePref::Light),
        separator_before: false,
    },
];

const GO: &[Item] = &[
    Item {
        label: ("Voltar", "Back"),
        message: || Message::HistoryBack,
        separator_before: false,
    },
    Item {
        label: ("Avançar", "Forward"),
        message: || Message::HistoryForward,
        separator_before: false,
    },
];

const HELP: &[Item] = &[
    Item {
        label: ("Configurações…", "Settings…"),
        message: || Message::ToggleSettings,
        separator_before: false,
    },
    Item {
        label: ("Definir como leitor padrão…", "Set as default reader…"),
        message: || Message::SetDefaultReader,
        separator_before: false,
    },
];

const SECTIONS: &[Section] = &[
    Section {
        title: ("Arquivo", "File"),
        items: FILE,
    },
    Section {
        title: ("Editar", "Edit"),
        items: EDIT,
    },
    Section {
        title: ("Ver", "View"),
        items: VIEW,
    },
    Section {
        title: ("Ir", "Go"),
        items: GO,
    },
    Section {
        title: ("Ajuda", "Help"),
        items: HELP,
    },
];

fn pick(label: (&'static str, &'static str), lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => label.0,
        Lang::En => label.1,
    }
}

struct ItemIvars {
    message: RefCell<Option<Message>>,
    sender: UnboundedSender<Message>,
}

define_class!(
    // NSMenuItem é o alvo da própria ação: o item se envia de volta para a
    // sessão, então não existe objeto-alvo para manter vivo à parte.
    #[unsafe(super = NSMenuItem)]
    #[thread_kind = MainThreadOnly]
    #[ivars = ItemIvars]
    struct TsuroMenuItem;

    unsafe impl NSObjectProtocol for TsuroMenuItem {}

    impl TsuroMenuItem {
        #[unsafe(method(tsuroFired:))]
        fn fired(&self, _sender: &NSMenuItem) {
            let message = self.ivars().message.borrow().clone();
            if let Some(message) = message {
                let _ = self.ivars().sender.unbounded_send(message);
            }
        }
    }
);

impl TsuroMenuItem {
    fn new(
        mtm: MainThreadMarker,
        title: &NSString,
        message: Message,
        sender: UnboundedSender<Message>,
        key: &str,
    ) -> Retained<Self> {
        let this = mtm.alloc().set_ivars(ItemIvars {
            message: RefCell::new(Some(message)),
            sender,
        });
        unsafe {
            msg_send![
                super(this),
                initWithTitle: title,
                action: Some(sel!(tsuroFired:)),
                keyEquivalent: &*NSString::from_str(key)
            ]
        }
    }
}

pub(crate) struct MenuEvents {
    receiver: Arc<Mutex<UnboundedReceiver<Message>>>,
    bar: Option<Retained<NSMenu>>,
    sender: UnboundedSender<Message>,
}

impl MenuEvents {
    /// Canal inerte para testes: nenhuma escolha chega, nada toca o AppKit.
    #[cfg(test)]
    pub(crate) fn for_test() -> Self {
        let (sender, receiver) = mpsc::unbounded();
        Self {
            receiver: Arc::new(Mutex::new(receiver)),
            bar: None,
            sender,
        }
    }

    /// Reaplica a barra. O iced monta a dele quando o laço de eventos começa,
    /// depois deste `install`, então quem devolve a nossa é o primeiro update.
    pub(crate) fn apply(&self) {
        let (Some(mtm), Some(bar)) = (MainThreadMarker::new(), self.bar.as_ref()) else {
            return;
        };
        NSApplication::sharedApplication(mtm).setMainMenu(Some(bar));
    }

    /// Remonta a barra no idioma atual. Os rótulos saem da tabela na montagem,
    /// então trocar o idioma com o app aberto deixaria a barra velha.
    pub(crate) fn rebuild(&mut self, lang: Lang) {
        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };
        let bar = build_bar(mtm, lang, &self.sender);
        NSApplication::sharedApplication(mtm).setMainMenu(Some(&bar));
        self.bar = Some(bar);
    }

    pub(crate) fn stream(&self) -> impl Stream<Item = Message> + Send + 'static {
        let receiver = Arc::clone(&self.receiver);
        stream::poll_fn(move |context| {
            Pin::new(&mut *receiver.lock().expect("menu receiver poisoned")).poll_next(context)
        })
    }
}

/// Monta a barra do sistema e devolve o canal por onde as escolhas chegam.
pub(crate) fn install(mtm: MainThreadMarker, lang: Lang) -> MenuEvents {
    let (sender, receiver) = mpsc::unbounded();
    let bar = build_bar(mtm, lang, &sender);
    NSApplication::sharedApplication(mtm).setMainMenu(Some(&bar));
    MenuEvents {
        receiver: Arc::new(Mutex::new(receiver)),
        bar: Some(bar),
        sender,
    }
}

/// Monta a barra inteira: menu do app, seções da tabela e o item de Sair.
fn build_bar(
    mtm: MainThreadMarker,
    lang: Lang,
    sender: &UnboundedSender<Message>,
) -> Retained<NSMenu> {
    let bar = NSMenu::new(mtm);
    bar.setAutoenablesItems(false);

    // Menu do app: Sobre e Sair, os dois que a barra do sistema precisa ter.
    let app_menu = submenu(mtm, "TsuroPDF");
    app_menu.addItem(&message_item(
        mtm,
        pick(("Sobre o Tsuro PDF", "About Tsuro PDF"), lang),
        Message::ToggleAbout,
        sender,
        "",
    ));
    app_menu.addItem(&NSMenuItem::separatorItem(mtm));
    // ⌘Q fala com a sessão, não com o `terminate:` do AppKit: o fechamento
    // passa por `request_quit` e pergunta antes de descartar marcação suja.
    app_menu.addItem(&message_item(
        mtm,
        pick(("Sair do Tsuro PDF", "Quit Tsuro PDF"), lang),
        Message::QuitApp,
        sender,
        "q",
    ));
    let app_parent = submenu_item(mtm, "TsuroPDF");
    app_parent.setSubmenu(Some(&app_menu));
    bar.addItem(&app_parent);

    for section in SECTIONS {
        let menu = submenu(mtm, pick(section.title, lang));
        for entry in section.items {
            if entry.separator_before {
                menu.addItem(&NSMenuItem::separatorItem(mtm));
            }
            menu.addItem(&message_item(
                mtm,
                pick(entry.label, lang),
                (entry.message)(),
                sender,
                // Sem atalho: as teclas da janela são do iced, que respeita o
                // modal aberto. O único equivalente é o ⌘Q do Sair.
                "",
            ));
        }
        let parent = submenu_item(mtm, pick(section.title, lang));
        parent.setSubmenu(Some(&menu));
        bar.addItem(&parent);
    }
    bar
}

fn submenu(mtm: MainThreadMarker, title: &str) -> Retained<NSMenu> {
    let menu = NSMenu::new(mtm);
    menu.setTitle(&NSString::from_str(title));
    menu.setAutoenablesItems(false);
    menu
}

/// Item pai de um submenu: sem ação própria, só o título que abre a lista.
fn submenu_item(mtm: MainThreadMarker, title: &str) -> Retained<NSMenuItem> {
    unsafe {
        NSMenuItem::initWithTitle_action_keyEquivalent(
            mtm.alloc(),
            &NSString::from_str(title),
            None,
            &NSString::new(),
        )
    }
}

/// Linha que envia a sua `Message`. O item é o próprio alvo da ação, então
/// não existe objeto-alvo separado para manter vivo.
fn message_item(
    mtm: MainThreadMarker,
    title: &'static str,
    message: Message,
    sender: &UnboundedSender<Message>,
    key: &str,
) -> Retained<NSMenuItem> {
    let item = TsuroMenuItem::new(
        mtm,
        &NSString::from_str(title),
        message,
        sender.clone(),
        key,
    );
    unsafe { item.setTarget(Some(&item)) };
    Retained::into_super(item)
}
