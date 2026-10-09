use engine::OrbitCamera;

use super::gestures::Gesture;
use crate::web::view_rect::{Point, ViewRect};

const ORBIT_RADIANS_PER_PIXEL: f64 = 0.006;
const ZOOM_PER_WHEEL_PIXEL: f64 = 0.0015;

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
