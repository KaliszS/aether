use engine::DVec2;
use web_sys::HtmlCanvasElement;

/// A point on the page, in CSS pixels.
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Canvas position on the page, in CSS pixels.
pub struct ViewRect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

impl ViewRect {
    pub fn of(canvas: &HtmlCanvasElement) -> Self {
        let rect = canvas.get_bounding_client_rect();
        Self {
            left: rect.left(),
            top: rect.top(),
            width: rect.width(),
            height: rect.height(),
        }
    }

    pub fn shorter_side(&self) -> f64 {
        self.width.min(self.height).max(1.0)
    }

    /// Page point to view coords. Same convention as the shader: center is 0,
    /// shorter side spans -1..1, y down.
    pub fn to_view(&self, p: Point) -> DVec2 {
        (DVec2::new(p.x - self.left, p.y - self.top) - self.half_size()) / self.shorter_side() * 2.0
    }

    /// View coords to a point relative to the canvas' top-left corner.
    pub fn to_local(&self, view: DVec2) -> Point {
        let local = view * self.shorter_side() / 2.0 + self.half_size();
        Point {
            x: local.x,
            y: local.y,
        }
    }

    pub fn view_to_pixels(&self, view_length: f64) -> f64 {
        view_length * self.shorter_side() / 2.0
    }

    fn half_size(&self) -> DVec2 {
        DVec2::new(self.width, self.height) / 2.0
    }
}
