use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use leptos::prelude::window;
use wasm_bindgen::{JsCast, closure::Closure};

const MAX_DT: f64 = 0.1;

type FrameCallback = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

/// A running requestAnimationFrame loop; stops when dropped.
pub struct FrameLoop {
    callback: FrameCallback,
    request_id: Rc<Cell<i32>>,
}

impl Drop for FrameLoop {
    fn drop(&mut self) {
        let _ = window().cancel_animation_frame(self.request_id.get());
        // Breaks the callback -> itself reference cycle, freeing everything it captured.
        self.callback.borrow_mut().take();
    }
}

/// Calls `on_frame(time, dt)` in seconds before every screen refresh.
/// `dt` is capped so a backgrounded tab does not cause a jump.
pub fn start(mut on_frame: impl FnMut(f64, f64) + 'static) -> FrameLoop {
    let callback: FrameCallback = Rc::new(RefCell::new(None));
    let request_id = Rc::new(Cell::new(0));
    let mut last_ms: Option<f64> = None;
    *callback.borrow_mut() = Some(Closure::new({
        let (callback, request_id) = (callback.clone(), request_id.clone());
        move |time_ms: f64| {
            let dt = last_ms.map_or(0.0, |last| (time_ms - last) / 1000.0);
            last_ms = Some(time_ms);
            on_frame(time_ms / 1000.0, dt.min(MAX_DT));
            if let Some(next) = callback.borrow().as_ref() {
                request_id.set(request_frame(next));
            }
        }
    }));
    request_id.set(request_frame(callback.borrow().as_ref().unwrap()));
    FrameLoop {
        callback,
        request_id,
    }
}

fn request_frame(callback: &Closure<dyn FnMut(f64)>) -> i32 {
    window()
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .unwrap()
}
