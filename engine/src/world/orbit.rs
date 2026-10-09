use super::BodyId;

/// Circular orbit in the ecliptic (XZ) plane.
#[derive(Clone, Copy, Debug)]
pub struct Orbit {
    pub parent: BodyId,
    pub radius: f64,
    pub period: f64,
    /// Angle at time 0, in radians.
    pub phase: f64,
}
