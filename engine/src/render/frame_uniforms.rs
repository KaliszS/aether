use bytemuck::{Pod, Zeroable};

use crate::{camera::OrbitCamera, world::World};

pub const MAX_BODIES: usize = 16;

/// Per-frame data for the GPU. Mirrors `Frame` in shaders/scene.wgsl: same field order and padding.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct FrameUniforms {
    right: [f32; 3],
    tan_half_fov: f32,
    up: [f32; 3],
    time: f32,
    forward: [f32; 3],
    body_count: u32,
    resolution: [f32; 2],
    _pad: [f32; 2],
    /// xyz: center relative to the camera, w: radius.
    bodies: [[f32; 4]; MAX_BODIES],
}

impl FrameUniforms {
    pub fn new(
        world: &World,
        camera: &OrbitCamera,
        (width, height): (u32, u32),
        time: f32,
    ) -> Self {
        let (right, up, forward) = camera.basis();
        let (bodies, body_count) = camera_relative_bodies(world, camera);
        Self {
            right: right.as_vec3().to_array(),
            tan_half_fov: camera.tan_half_fov() as f32,
            up: up.as_vec3().to_array(),
            time,
            forward: forward.as_vec3().to_array(),
            body_count,
            resolution: [width as f32, height as f32],
            _pad: [0.0; 2],
            bodies,
        }
    }
}

/// Subtracting the camera position in f64 before converting to f32 keeps
/// precision near the camera no matter how far it is from the world origin.
fn camera_relative_bodies(world: &World, camera: &OrbitCamera) -> ([[f32; 4]; MAX_BODIES], u32) {
    let eye = camera.eye();
    let mut slots = [[0.0; 4]; MAX_BODIES];
    let mut count = 0;
    for (slot, body) in slots.iter_mut().zip(world.bodies()) {
        let center = (body.position - eye).as_vec3();
        *slot = [center.x, center.y, center.z, body.radius as f32];
        count += 1;
    }
    (slots, count)
}
