use glam::{DVec2, DVec3};

use super::{easing::ease_factor, pose::Pose};

const MIN_DISTANCE: f64 = 1.15;
const MAX_DISTANCE: f64 = 200.0;
const MAX_PITCH: f64 = 1.5;
const EASE_RATE: f64 = 10.0;

/// Orbits a movable focus point. Input changes the goal pose; `update` eases towards it.
pub struct OrbitCamera {
    current: Pose,
    goal: Pose,
    fov_y: f64,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        let pose = Pose {
            focus: DVec3::ZERO,
            yaw: 0.0,
            pitch: 0.2,
            log_distance: 4f64.ln(),
        };
        Self {
            current: pose,
            goal: pose,
            fov_y: 45f64.to_radians(),
        }
    }
}

impl OrbitCamera {
    pub fn orbit(&mut self, d_yaw: f64, d_pitch: f64) {
        self.goal.yaw -= d_yaw;
        self.goal.pitch = (self.goal.pitch + d_pitch).clamp(-MAX_PITCH, MAX_PITCH);
    }

    /// Moves the focus in the view plane; `dx`, `dy` are fractions of the view size.
    pub fn pan(&mut self, dx: f64, dy: f64) {
        let (right, up, _) = self.goal.basis();
        let view_size = 2.0 * self.tan_half_fov() * self.goal.distance();
        self.goal.focus += (up * dy - right * dx) * view_size;
    }

    /// `factor` < 1 moves closer, > 1 moves away.
    pub fn zoom(&mut self, factor: f64) {
        self.goal.log_distance =
            (self.goal.log_distance + factor.ln()).clamp(MIN_DISTANCE.ln(), MAX_DISTANCE.ln());
    }

    /// Zooms so the point under `cursor` (view coords, see `Pose::focal_point`) stays put.
    pub fn zoom_at(&mut self, factor: f64, cursor: DVec2) {
        let anchor = self.goal.focal_point(cursor, self.tan_half_fov());
        let log_before = self.goal.log_distance;
        self.zoom(factor);
        let applied = (self.goal.log_distance - log_before).exp();
        self.goal.focus = anchor + (self.goal.focus - anchor) * applied;
    }

    pub fn update(&mut self, dt: f64) {
        self.current = self.current.lerp(&self.goal, ease_factor(EASE_RATE, dt));
    }

    pub(crate) fn eye(&self) -> DVec3 {
        self.current.eye()
    }

    pub(crate) fn basis(&self) -> (DVec3, DVec3, DVec3) {
        self.current.basis()
    }

    pub(crate) fn tan_half_fov(&self) -> f64 {
        (self.fov_y * 0.5).tan()
    }
}
