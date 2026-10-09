use std::{cell::RefCell, rc::Rc};

use leptos::prelude::window;
use wasm_bindgen::{JsCast, closure::Closure};

const MAX_DT: f64 = 0.1;

type FrameCallback = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

/// Calls `on_frame(time, dt)` in seconds before every screen refresh (requestAnimationFrame).
/// `dt` is capped so a backgrounded tab does not cause a jump.
pub fn start(mut on_frame: impl FnMut(f64, f64) + 'static) {
    let mut last_ms: Option<f64> = None;
    let callback: FrameCallback = Rc::new(RefCell::new(None));
    let next = callback.clone();
    *callback.borrow_mut() = Some(Closure::new(move |time_ms: f64| {
        let dt = last_ms.map_or(0.0, |last| (time_ms - last) / 1000.0);
        last_ms = Some(time_ms);
        on_frame(time_ms / 1000.0, dt.min(MAX_DT));
        request_frame(next.borrow().as_ref().unwrap());
    }));
    request_frame(callback.borrow().as_ref().unwrap());
}

fn request_frame(callback: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .unwrap();
}
