use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use engine::{BodyId, OrbitCamera, World, update_orbits};

/// Shared simulation state: what exists, where we look from, and which body we follow.
#[derive(Clone)]
pub struct Scene {
    pub world: Rc<RefCell<World>>,
    pub camera: Rc<RefCell<OrbitCamera>>,
    followed: Rc<Cell<Option<BodyId>>>,
}

impl Scene {
    pub fn new(world: World, camera: OrbitCamera, followed: Option<BodyId>) -> Self {
        Self {
            world: Rc::new(RefCell::new(world)),
            camera: Rc::new(RefCell::new(camera)),
            followed: Rc::new(Cell::new(followed)),
        }
    }

    /// Flies the camera to a body and keeps following it.
    pub fn select(&self, id: BodyId) {
        let world = self.world.borrow();
        let body = world.body(id);
        self.camera
            .borrow_mut()
            .focus_on(body.position, body.radius);
        self.followed.set(Some(id));
    }

    /// Advances the simulation to `time` (simulated seconds) and the camera by `dt` (real seconds).
    pub fn step(&self, time: f64, dt: f64) {
        let mut world = self.world.borrow_mut();
        update_orbits(&mut world, time);
        let mut camera = self.camera.borrow_mut();
        if let Some(id) = self.followed.get() {
            camera.track(world.body(id).position);
        }
        camera.update(dt);
    }
}
