use std::path::PathBuf;

use tsuro::{boot, Session};

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
        .exit_on_close_request(false)
        .run_with(move || boot(session))
}

#[cfg(target_os = "macos")]
fn run(session: Session) -> iced::Result {
    use crate::native_open::{Bridge, Desktop};

    let main_thread =
        objc2::MainThreadMarker::new().expect("macOS app must run on the main thread");
    let (bridge, events) = Bridge::install(main_thread);
    let (session, task) = boot(session);
    let result = iced::application("TsuroPDF", Desktop::update, Desktop::view)
        .subscription(Desktop::subscription)
        .exit_on_close_request(false)
        .run_with(move || {
            (
                Desktop::new(session, events),
                task.map(native_open::Message::Session),
            )
        });
    drop(bridge);
    result
}
