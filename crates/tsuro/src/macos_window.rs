//! Ajustes na `NSWindow` que o iced não expõe.
//!
//! O conteúdo corre sob a barra de título (`fullsize_content_view`), então um
//! clique rápido em + / − na barra caía também no gesto de duplo clique do
//! macOS, que manda a janela para tela cheia. A janela do app é única, então a
//! correção troca o método na classe daquela janela em vez de mexer na
//! hierarquia de views.

use objc2::runtime::{AnyObject, MethodImplementation, Sel};
use objc2::{sel, MainThreadMarker};
use objc2_app_kit::NSApplication;

/// Desliga o zoom por duplo clique na faixa de título.
pub(crate) fn disable_titlebar_double_click_zoom() {
    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let Some(window) = app.windows().iter().next() else {
        return;
    };
    // `- performZoom:` é o que o duplo clique na barra chama. Devolver cedo
    // mantém arrasto, semáforos e o resto do comportamento nativo intactos.
    let handler: unsafe extern "C-unwind" fn(*mut AnyObject, Sel, *mut AnyObject) = no_zoom;
    unsafe {
        objc2::ffi::class_replaceMethod(
            window.class() as *const _ as *mut _,
            sel!(performZoom:),
            handler.__imp(),
            c"v@:@".as_ptr(),
        );
    }
}

unsafe extern "C-unwind" fn no_zoom(_this: *mut AnyObject, _sel: Sel, _sender: *mut AnyObject) {}
