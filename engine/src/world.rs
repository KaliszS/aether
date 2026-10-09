mod body;

pub use body::Body;

#[derive(Default)]
pub struct World {
    bodies: Vec<Body>,
}

impl World {
    pub fn add(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn bodies(&self) -> &[Body] {
        &self.bodies
    }
}
