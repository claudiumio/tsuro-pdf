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
//! para as faixas vazias da barra (`view::drag_band` e as bandas do
//! `toolbar_frame`): só onde não há controle nem texto. Na mesma troca some o
//! gesto nativo de duplo clique, que mandava a janela para tela cheia.
//!
//! A janela do app é única, então trocar o método na classe da view e da janela
//! vale para todas as janelas do processo sem mexer na hierarquia de views.

use objc2::runtime::{AnyObject, MethodImplementation, Sel};
use objc2::{sel, MainThreadMarker};
use objc2_app_kit::{NSApplication, NSWindowButton};

/// Aplica os ajustes na janela principal. Idempotente e devolve se achou a
/// janela: no primeiro update ela ainda pode não existir, e sem os ajustes o
/// arrasto implícito do AppKit volta — então quem chama tenta de novo.
pub(crate) fn install() -> bool {
    let Some(window) = window() else {
        return false;
    };
    let zoom: unsafe extern "C-unwind" fn(*mut AnyObject, Sel, *mut AnyObject) = no_zoom;
    let movable: unsafe extern "C-unwind" fn(*mut AnyObject, Sel) -> i8 = no_move;
    unsafe {
        // `- performZoom:` é o que o duplo clique na faixa chama. Isso desliga
        // o zoom da janela por inteiro, inclusive o Option+clique no semáforo
        // verde; o clique simples no verde usa `toggleFullScreen:` e continua.
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
    align_lights();
    true
}

/// Desce os semáforos nativos até a linha dos controles da barra. O AppKit os
/// põe a 15,5 do topo e a barra põe os controles a `bar_air_top` mais meio
/// botão, então a diferença é o que falta para os dois lerem na mesma linha.
///
/// Idempotente e chamada de novo a cada resize: o AppKit refaz o layout da
/// barra de título no meio do arrasto e devolveria os botões para cima. O alvo
/// é recalculado da posição atual, então repetir a chamada não acumula.
pub(crate) fn align_lights() {
    let Some(window) = window() else {
        return;
    };
    let metrics = tsuro::kiri::Metrics::current();
    let target = f64::from(metrics.bar_air_top() + metrics.button / 2.0);
    for kind in [
        NSWindowButton::CloseButton,
        NSWindowButton::MiniaturizeButton,
        NSWindowButton::ZoomButton,
    ] {
        let Some(button) = window.standardWindowButton(kind) else {
            continue;
        };
        // O frame vem no espaço da superview, que é a barra de título encostada
        // no topo da janela: o centro a partir do topo sai da altura dela.
        // Premissa: a superview dos botões começa no topo da janela (hoje
        // verdade com `fullsize_content_view`). Converter o retângulo para a
        // janela mente aqui, porque a view de conteúdo cobre a janela inteira e
        // o retângulo volta deslocado.
        let Some(superview) = (unsafe { button.superview() }) else {
            continue;
        };
        let frame = button.frame();
        let superview_height = superview.bounds().size.height;
        // Em tela cheia e com janela em aba os botões somem e a barra de
        // título encolhe; medir ali daria um alvo sem sentido.
        if button.isHidden() || superview_height < frame.size.height {
            continue;
        }
        let from_top = superview_height - frame.mid().y;
        let delta = target - from_top;
        if delta.abs() < 0.5 {
            continue;
        }
        // A barra de título do AppKit não é invertida (y cresce para cima),
        // então descer é subtrair; numa view invertida é somar.
        let y = if superview.isFlipped() {
            frame.origin.y + delta
        } else {
            frame.origin.y - delta
        };
        button.setFrameOrigin(objc2_foundation::NSPoint::new(frame.origin.x, y));
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
    window.performWindowDragWithEvent(&event);
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
