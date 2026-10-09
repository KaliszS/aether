use glam::{DVec2, DVec3};

/// Where an orbit camera is: a focus point plus a direction and distance from it.
#[derive(Clone, Copy)]
pub struct Pose {
    pub focus: DVec3,
    pub yaw: f64,
    pub pitch: f64,
    pub log_distance: f64,
}

impl Pose {
    pub fn distance(&self) -> f64 {
        self.log_distance.exp()
    }

    pub fn eye(&self) -> DVec3 {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let direction = DVec3::new(cos_pitch * sin_yaw, sin_pitch, cos_pitch * cos_yaw);
        self.focus + direction * self.distance()
    }

    /// Returns (right, up, forward).
    pub fn basis(&self) -> (DVec3, DVec3, DVec3) {
        let forward = (self.focus - self.eye()).normalize();
        let right = forward.cross(DVec3::Y).normalize();
        let up = right.cross(forward);
        (right, up, forward)
    }

    /// Point on the plane through `focus` facing the camera, seen at view coords `at`
    /// (center is 0, shorter side spans -1..1, y down).
    pub fn focal_point(&self, at: DVec2, tan_half_fov: f64) -> DVec3 {
        let (right, up, _) = self.basis();
        self.focus + (right * at.x - up * at.y) * tan_half_fov * self.distance()
    }

    pub fn lerp(&self, goal: &Pose, t: f64) -> Pose {
        Pose {
            focus: self.focus.lerp(goal.focus, t),
            yaw: self.yaw + (goal.yaw - self.yaw) * t,
            pitch: self.pitch + (goal.pitch - self.pitch) * t,
            log_distance: self.log_distance + (goal.log_distance - self.log_distance) * t,
        }
    }
}
