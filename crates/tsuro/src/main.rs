use std::path::PathBuf;

use tsuro::{boot, Session};

#[cfg(target_os = "macos")]
mod macos_menu;
#[cfg(target_os = "macos")]
mod native_open;

fn main() -> iced::Result {
    let session = match std::env::args().nth(1) {
        Some(path) => Session::open_path(PathBuf::from(path)),
        None => Session::empty(),
    };
    run(session)
}

#[cfg(not(target_os = "macos"))]
fn run(session: Session) -> iced::Result {
    iced::application("TsuroPDF", Session::update, Session::view)
        .subscription(Session::subscription)
        .default_font(tsuro::font::INTER)
        .exit_on_close_request(false)
        .run_with(move || boot(session))
}

#[cfg(target_os = "macos")]
fn run(session: Session) -> iced::Result {
    use crate::native_open::{Bridge, Desktop};
    use iced::window::settings::PlatformSpecific;

    let main_thread =
        objc2::MainThreadMarker::new().expect("macOS app must run on the main thread");
    let (bridge, events) = Bridge::install(main_thread);
    let (session, task) = boot(session);
    let menu = crate::macos_menu::install(main_thread, session.lang());
    let result = iced::application("TsuroPDF", Desktop::update, Desktop::view)
        .subscription(Desktop::subscription)
        .default_font(tsuro::font::INTER)
        .window(iced::window::Settings {
            // Barra unificada (spec §5): o conteúdo sobe sob a barra de título,
            // o título nativo some e os semáforos continuam os do sistema.
            platform_specific: PlatformSpecific {
                title_hidden: true,
                titlebar_transparent: true,
                fullsize_content_view: true,
            },
            ..iced::window::Settings::default()
        })
        .exit_on_close_request(false)
        .run_with(move || {
            (
                Desktop::new(session, events, menu),
                task.map(native_open::Message::Session),
            )
        });
    drop(bridge);
    result
}
