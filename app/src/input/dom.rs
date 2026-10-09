use wasm_bindgen::{JsCast, closure::Closure, convert::FromWasmAbi};
use web_sys::{AddEventListenerOptions, HtmlCanvasElement, MouseEvent, PointerEvent, WheelEvent};

use super::{
    controls::ViewRect,
    gestures::{DragMode, Point},
};

const LEFT_BUTTON: i16 = 0;

/// Non-passive so handlers can call `prevent_default` (stops page scroll/zoom).
pub fn listen<E: FromWasmAbi + 'static>(
    canvas: &HtmlCanvasElement,
    event: &str,
    handler: impl FnMut(E) + 'static,
) {
    let closure = Closure::<dyn FnMut(E)>::new(handler);
    let options = AddEventListenerOptions::new();
    options.set_passive(false);
    canvas
        .add_event_listener_with_callback_and_add_event_listener_options(
            event,
            closure.as_ref().unchecked_ref(),
            &options,
        )
        .unwrap();
    closure.forget();
}

pub fn point(e: &MouseEvent) -> Point {
    Point {
        x: e.client_x() as f64,
        y: e.client_y() as f64,
    }
}

/// Mouse: left drag pans, right/middle (or Shift+left) orbits. Touch: one finger orbits.
pub fn drag_mode(e: &PointerEvent) -> DragMode {
    let is_mouse = e.pointer_type() == "mouse";
    if is_mouse && e.button() == LEFT_BUTTON && !e.shift_key() {
        DragMode::Pan
    } else {
        DragMode::Orbit
    }
}

pub fn wheel_pixels(e: &WheelEvent) -> f64 {
    match e.delta_mode() {
        WheelEvent::DOM_DELTA_LINE => e.delta_y() * 16.0,
        WheelEvent::DOM_DELTA_PAGE => e.delta_y() * 800.0,
        _ => e.delta_y(),
    }
}

pub fn view_rect(canvas: &HtmlCanvasElement) -> ViewRect {
    let rect = canvas.get_bounding_client_rect();
    ViewRect {
        left: rect.left(),
        top: rect.top(),
        width: rect.width(),
        height: rect.height(),
    }
}
