mod controls;
mod dom;
mod gestures;

use std::{cell::RefCell, rc::Rc};

use engine::OrbitCamera;
use web_sys::{Event, HtmlCanvasElement, PointerEvent, WheelEvent};

use gestures::PointerTracker;

use crate::web::{
    dom_events::{Listener, listen},
    view_rect::ViewRect,
};

/// Mouse: left drag pans, right drag orbits, wheel zooms.
/// Touch: one finger orbits, two fingers pan and pinch-zoom.
pub fn attach(canvas: &HtmlCanvasElement, camera: Rc<RefCell<OrbitCamera>>) -> Vec<Listener> {
    let tracker = Rc::new(RefCell::new(PointerTracker::default()));
    let mut listeners = Vec::new();

    listeners.push(listen(canvas, "pointerdown", {
        let (canvas, tracker) = (canvas.clone(), tracker.clone());
        move |e: PointerEvent| {
            let _ = canvas.set_pointer_capture(e.pointer_id());
            tracker
                .borrow_mut()
                .press(e.pointer_id(), dom::point(&e), dom::drag_mode(&e));
        }
    }));

    listeners.push(listen(canvas, "pointermove", {
        let (canvas, tracker, camera) = (canvas.clone(), tracker.clone(), camera.clone());
        move |e: PointerEvent| {
            let gestures = tracker.borrow_mut().move_to(e.pointer_id(), dom::point(&e));
            let view = ViewRect::of(&canvas);
            let mut camera = camera.borrow_mut();
            for gesture in gestures {
                controls::apply(&mut camera, gesture, &view);
            }
        }
    }));

    for event in ["pointerup", "pointercancel"] {
        let tracker = tracker.clone();
        listeners.push(listen(canvas, event, move |e: PointerEvent| {
            tracker.borrow_mut().release(e.pointer_id());
        }));
    }

    listeners.push(listen(canvas, "wheel", {
        let canvas = canvas.clone();
        move |e: WheelEvent| {
            e.prevent_default();
            let gesture = controls::wheel_zoom(dom::wheel_pixels(&e), dom::point(&e));
            controls::apply(&mut camera.borrow_mut(), gesture, &ViewRect::of(&canvas));
        }
    }));

    listeners.push(listen(canvas, "contextmenu", |e: Event| {
        e.prevent_default()
    }));
    listeners
}
