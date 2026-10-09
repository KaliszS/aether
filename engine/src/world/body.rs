use glam::DVec3;

/// World coordinates are f64 so the scene can span planetary to galactic scales.
#[derive(Clone, Debug)]
pub struct Body {
    pub position: DVec3,
    pub radius: f64,
}
