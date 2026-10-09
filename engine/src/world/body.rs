use glam::DVec3;

use super::{Appearance, Orbit};

/// Units are SI: metres and seconds. f64 lets one scene span planets to galaxies.
#[derive(Clone, Debug)]
pub struct Body {
    pub name: String,
    pub position: DVec3,
    pub radius: f64,
    pub appearance: Appearance,
    pub orbit: Option<Orbit>,
}
