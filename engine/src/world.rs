mod appearance;
mod body;
mod orbit;

pub use appearance::Appearance;
pub use body::Body;
pub use orbit::Orbit;

pub type BodyId = usize;

#[derive(Default)]
pub struct World {
    bodies: Vec<Body>,
}

impl World {
    /// Orbiting bodies must be added after their parent.
    pub fn add(&mut self, body: Body) -> BodyId {
        self.bodies.push(body);
        self.bodies.len() - 1
    }

    pub fn bodies(&self) -> &[Body] {
        &self.bodies
    }

    pub fn body(&self, id: BodyId) -> &Body {
        &self.bodies[id]
    }

    pub(crate) fn bodies_mut(&mut self) -> &mut [Body] {
        &mut self.bodies
    }
}
