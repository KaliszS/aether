use std::{cell::RefCell, rc::Rc};

use engine::Renderer;
use leptos::prelude::window;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::HtmlCanvasElement;

/// Matches the canvas buffer to physical pixels. Returns true if the size changed.
pub fn fit_to_display(canvas: &HtmlCanvasElement) -> bool {
    let dpr = window().device_pixel_ratio();
    let width = (canvas.client_width() as f64 * dpr) as u32;
    let height = (canvas.client_height() as f64 * dpr) as u32;
    let changed = (width, height) != (canvas.width(), canvas.height());
    if changed {
        canvas.set_width(width);
        canvas.set_height(height);
    }
    changed
}

pub fn start_render_loop(canvas: HtmlCanvasElement, mut renderer: Renderer) {
    let frame: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let next = frame.clone();
    *frame.borrow_mut() = Some(Closure::new(move |time_ms: f64| {
        if fit_to_display(&canvas) {
            renderer.resize(canvas.width(), canvas.height());
        }
        renderer.render((time_ms / 1000.0) as f32);
        request_frame(next.borrow().as_ref().unwrap());
    }));
    request_frame(frame.borrow().as_ref().unwrap());
}

fn request_frame(callback: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .unwrap();
}
