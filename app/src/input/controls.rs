use engine::{DVec2, OrbitCamera};

use super::gestures::{Gesture, Point};

const ORBIT_RADIANS_PER_PIXEL: f64 = 0.006;
const ZOOM_PER_WHEEL_PIXEL: f64 = 0.0015;

/// Canvas position on the page, in CSS pixels.
pub struct ViewRect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

impl ViewRect {
    fn shorter_side(&self) -> f64 {
        self.width.min(self.height).max(1.0)
    }

    /// Same convention as the shader: center is 0, shorter side spans -1..1, y down.
    fn to_view(&self, p: Point) -> DVec2 {
        let center = DVec2::new(self.left + self.width / 2.0, self.top + self.height / 2.0);
        (DVec2::new(p.x, p.y) - center) / self.shorter_side() * 2.0
    }
}

/// Pans move the scene with the pointer; zooms keep the point under the pointer fixed.
pub fn apply(camera: &mut OrbitCamera, gesture: Gesture, view: &ViewRect) {
    match gesture {
        Gesture::Orbit { dx, dy } => {
            camera.orbit(dx * ORBIT_RADIANS_PER_PIXEL, dy * ORBIT_RADIANS_PER_PIXEL)
        }
        Gesture::Pan { dx, dy } => camera.pan(dx / view.shorter_side(), dy / view.shorter_side()),
        Gesture::Zoom { factor, at } => camera.zoom_at(factor, view.to_view(at)),
    }
}

pub fn wheel_zoom(pixels: f64, at: Point) -> Gesture {
    Gesture::Zoom {
        factor: (pixels * ZOOM_PER_WHEEL_PIXEL).exp(),
        at,
    }
}
