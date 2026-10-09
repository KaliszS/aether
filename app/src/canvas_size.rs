use leptos::prelude::window;
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
