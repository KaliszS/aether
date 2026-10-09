use std::f64::consts::TAU;

use glam::DVec3;

use crate::world::{Orbit, World};

/// Places every orbiting body where it is at `time`. Computed from time directly
/// (not integrated step by step), so it never drifts.
pub fn update_orbits(world: &mut World, time: f64) {
    for index in 0..world.bodies().len() {
        if let Some(orbit) = world.bodies()[index].orbit {
            let parent = world.body(orbit.parent).position;
            world.bodies_mut()[index].position = parent + offset_at(&orbit, time);
        }
    }
}

/// Counter-clockwise seen from +Y, as the planets move seen from above the Sun's north pole.
fn offset_at(orbit: &Orbit, time: f64) -> DVec3 {
    let angle = orbit.phase + TAU * time / orbit.period;
    DVec3::new(angle.cos(), 0.0, -angle.sin()) * orbit.radius
}
