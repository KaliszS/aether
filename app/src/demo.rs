use engine::{Body, DVec3, World};

pub fn demo_world() -> World {
    let mut world = World::default();
    world.add(Body {
        position: DVec3::ZERO,
        radius: 1.0,
    });
    world.add(Body {
        position: DVec3::new(6.0, 1.0, -4.0),
        radius: 0.4,
    });
    world
}
