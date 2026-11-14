//! Ajustes na `NSWindow` que o iced não expõe.
//!
//! O conteúdo corre sob a barra de título (`fullsize_content_view`), então a
//! janela se comporta como uma janela sem barra: o AppKit decide o arrasto pelo
//! `-mouseDownCanMoveWindow` da view de conteúdo, e a view do winit cobre a
//! janela inteira e não é opaca — logo responde `YES` e qualquer clique que o
//! iced não consuma vira arrasto da janela. Com a barra em uma linha só, isso
//! pegava até o número da página.
//!
//! A correção desliga o arrasto implícito do AppKit e deixa o arrasto explícito
//! para a faixa vazia da barra (`view::drag_region`), que pede
//! [`begin_drag`] só onde não há controle. Na mesma troca some o gesto nativo de
//! duplo clique, que mandava a janela para tela cheia.
//!
//! A janela do app é única, então trocar o método na classe da view e da janela
//! vale para todas as janelas do processo sem mexer na hierarquia de views.

use objc2::runtime::{AnyObject, MethodImplementation, Sel};
use objc2::{sel, MainThreadMarker};
use objc2_app_kit::NSApplication;

/// Aplica os ajustes na janela principal. Idempotente.
pub(crate) fn install() {
    let Some(window) = window() else {
        return;
    };
    let zoom: unsafe extern "C-unwind" fn(*mut AnyObject, Sel, *mut AnyObject) = no_zoom;
    let movable: unsafe extern "C-unwind" fn(*mut AnyObject, Sel) -> i8 = no_move;
    unsafe {
        // `- performZoom:` é o que o duplo clique na faixa chama.
        objc2::ffi::class_replaceMethod(
            window.class() as *const _ as *mut _,
            sel!(performZoom:),
            zoom.__imp(),
            c"v@:@".as_ptr(),
        );
        if let Some(view) = window.contentView() {
            // `BOOL` é um byte nas duas arquiteturas; `B` codifica o retorno.
            objc2::ffi::class_replaceMethod(
                view.class() as *const _ as *mut _,
                sel!(mouseDownCanMoveWindow),
                movable.__imp(),
                c"B@:".as_ptr(),
            );
        }
    }
}

/// Começa o arrasto da janela. Precisa rodar durante o `mouseDown` que o pediu:
/// o AppKit arrasta a partir do evento corrente, então isto é chamado direto do
/// `update`, não por uma `Task` (que chegaria depois do evento, sem evento
/// corrente para arrastar).
pub(crate) fn begin_drag() -> bool {
    let Some(window) = window() else {
        return false;
    };
    let Some(mtm) = MainThreadMarker::new() else {
        return false;
    };
    let Some(event) = NSApplication::sharedApplication(mtm).currentEvent() else {
        return false;
    };
    unsafe { window.performWindowDragWithEvent(&event) };
    true
}

fn window() -> Option<objc2::rc::Retained<objc2_app_kit::NSWindow>> {
    let mtm = MainThreadMarker::new()?;
    let app = NSApplication::sharedApplication(mtm);
    app.windows().iter().next()
}

unsafe extern "C-unwind" fn no_zoom(_this: *mut AnyObject, _sel: Sel, _sender: *mut AnyObject) {}

unsafe extern "C-unwind" fn no_move(_this: *mut AnyObject, _sel: Sel) -> i8 {
    0
}
