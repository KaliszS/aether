mod controls;
mod dom;
mod gestures;

use std::{cell::RefCell, rc::Rc};

use engine::OrbitCamera;
use web_sys::{Event, HtmlCanvasElement, PointerEvent, WheelEvent};

use gestures::PointerTracker;

/// Mouse: left drag pans, right drag orbits, wheel zooms.
/// Touch: one finger orbits, two fingers pan and pinch-zoom.
pub fn attach(canvas: &HtmlCanvasElement, camera: Rc<RefCell<OrbitCamera>>) {
    let tracker = Rc::new(RefCell::new(PointerTracker::default()));

    dom::listen(canvas, "pointerdown", {
        let (canvas, tracker) = (canvas.clone(), tracker.clone());
        move |e: PointerEvent| {
            let _ = canvas.set_pointer_capture(e.pointer_id());
            tracker
                .borrow_mut()
                .press(e.pointer_id(), dom::point(&e), dom::drag_mode(&e));
        }
    });

    dom::listen(canvas, "pointermove", {
        let (canvas, tracker, camera) = (canvas.clone(), tracker.clone(), camera.clone());
        move |e: PointerEvent| {
            let gestures = tracker.borrow_mut().move_to(e.pointer_id(), dom::point(&e));
            let view = dom::view_rect(&canvas);
            let mut camera = camera.borrow_mut();
            for gesture in gestures {
                controls::apply(&mut camera, gesture, &view);
            }
        }
    });

    for event in ["pointerup", "pointercancel"] {
        let tracker = tracker.clone();
        dom::listen(canvas, event, move |e: PointerEvent| {
            tracker.borrow_mut().release(e.pointer_id());
        });
    }

    dom::listen(canvas, "wheel", {
        let canvas = canvas.clone();
        move |e: WheelEvent| {
            e.prevent_default();
            let gesture = controls::wheel_zoom(dom::wheel_pixels(&e), dom::point(&e));
            controls::apply(&mut camera.borrow_mut(), gesture, &dom::view_rect(&canvas));
        }
    });

    dom::listen(canvas, "contextmenu", |e: Event| e.prevent_default());
}
