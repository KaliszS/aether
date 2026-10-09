use glam::{DVec2, DVec3};

use super::{easing::ease_factor, pose::Pose};

const MAX_DISTANCE: f64 = 1e14;
const MAX_PITCH: f64 = 1.5;
const EASE_RATE: f64 = 6.0;
const MIN_DISTANCE_RADII: f64 = 1.3;
const FRAMING_RADII: f64 = 4.0;
const FOV_Y_DEG: f64 = 45.0;

/// Orbits a movable focus point. Input changes the goal pose; `update` eases towards it.
pub struct OrbitCamera {
    /// World point the poses are relative to; moves with the followed body.
    origin: DVec3,
    current: Pose,
    goal: Pose,
    min_distance: f64,
}

impl OrbitCamera {
    pub fn new(origin: DVec3, distance: f64, min_distance: f64) -> Self {
        let pose = Pose {
            focus: DVec3::ZERO,
            yaw: 0.0,
            pitch: 0.5,
            log_distance: distance.ln(),
        };
        Self {
            origin,
            current: pose,
            goal: pose,
            min_distance,
        }
    }

    pub fn with_pitch(mut self, pitch: f64) -> Self {
        self.current.pitch = pitch;
        self.goal.pitch = pitch;
        self
    }

    /// Distance at which an object of `half_extent` fills the shorter side of the view.
    pub fn distance_to_fit(half_extent: f64) -> f64 {
        half_extent / tan_half_fov()
    }

    /// Flies to `origin` and frames an object of `radius` there.
    pub fn focus_on(&mut self, origin: DVec3, radius: f64) {
        self.current.focus += self.origin - origin;
        self.goal.focus = DVec3::ZERO;
        self.origin = origin;
        self.min_distance = radius * MIN_DISTANCE_RADII;
        self.goal.log_distance = (radius * FRAMING_RADII).ln();
    }

    /// Moves the camera along with its origin, e.g. to follow an orbiting body.
    pub fn track(&mut self, origin: DVec3) {
        self.origin = origin;
    }

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
            (self.goal.log_distance + factor.ln()).clamp(self.min_distance.ln(), MAX_DISTANCE.ln());
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

    /// View coords of a world point (center 0, shorter side spans -1..1, y down);
    /// `None` when it is behind the camera.
    pub fn project(&self, point: DVec3) -> Option<DVec2> {
        let (right, up, forward) = self.basis();
        let relative = point - self.eye();
        let depth = relative.dot(forward);
        (depth > 0.0).then(|| {
            DVec2::new(relative.dot(right), -relative.dot(up)) / (depth * self.tan_half_fov())
        })
    }

    /// Radius of a sphere on screen, in view units.
    pub fn apparent_radius(&self, center: DVec3, radius: f64) -> f64 {
        radius / ((center - self.eye()).length() * self.tan_half_fov())
    }

    pub(crate) fn eye(&self) -> DVec3 {
        self.origin + self.current.eye()
    }

    pub(crate) fn basis(&self) -> (DVec3, DVec3, DVec3) {
        self.current.basis()
    }

    pub(crate) fn tan_half_fov(&self) -> f64 {
        tan_half_fov()
    }
}

fn tan_half_fov() -> f64 {
    (FOV_Y_DEG.to_radians() * 0.5).tan()
}
