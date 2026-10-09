use web_sys::{MouseEvent, PointerEvent, WheelEvent};

use super::gestures::DragMode;
use crate::web::view_rect::Point;

const LEFT_BUTTON: i16 = 0;

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
