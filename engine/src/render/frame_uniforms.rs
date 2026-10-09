use bytemuck::{Pod, Zeroable};
use glam::DVec3;

use crate::{
    camera::OrbitCamera,
    world::{Appearance, Body, World},
};

pub const MAX_BODIES: usize = 16;

const KIND_STAR: u32 = 0;
const KIND_PLANET: u32 = 1;

/// Per-frame data for the GPU. Mirrors `Frame` in shaders/scene.wgsl: same field order and padding.
/// All positions are relative to the camera.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct FrameUniforms {
    right: [f32; 3],
    tan_half_fov: f32,
    up: [f32; 3],
    time: f32,
    forward: [f32; 3],
    body_count: u32,
    light: [f32; 3],
    has_light: u32,
    resolution: [f32; 2],
    _pad: [f32; 2],
    bodies: [BodyUniform; MAX_BODIES],
}

/// Mirrors `Body` in shaders/scene.wgsl.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct BodyUniform {
    center: [f32; 3],
    radius: f32,
    base_color: [f32; 3],
    kind: u32,
    accent_color: [f32; 3],
    banding: f32,
    orbit_center: [f32; 3],
    orbit_radius: f32,
}

impl FrameUniforms {
    pub fn new(
        world: &World,
        camera: &OrbitCamera,
        (width, height): (u32, u32),
        time: f32,
    ) -> Self {
        let eye = camera.eye();
        let (right, up, forward) = camera.basis();
        let light = find_light(world);
        let mut uniforms = Self {
            right: right.as_vec3().to_array(),
            tan_half_fov: camera.tan_half_fov() as f32,
            up: up.as_vec3().to_array(),
            time,
            forward: forward.as_vec3().to_array(),
            body_count: 0,
            light: light.map_or([0.0; 3], |p| relative_to(eye, p)),
            has_light: light.is_some() as u32,
            resolution: [width as f32, height as f32],
            ..Zeroable::zeroed()
        };
        for (slot, body) in uniforms.bodies.iter_mut().zip(world.bodies()) {
            *slot = BodyUniform::new(body, world, eye);
            uniforms.body_count += 1;
        }
        uniforms
    }
}

impl BodyUniform {
    fn new(body: &Body, world: &World, eye: DVec3) -> Self {
        let (kind, base_color, accent_color, banding) = match body.appearance {
            Appearance::Star => (KIND_STAR, [0.0; 3], [0.0; 3], 0.0),
            Appearance::Planet {
                base,
                accent,
                banding,
            } => (KIND_PLANET, base, accent, banding),
        };
        let (orbit_center, orbit_radius) = body.orbit.map_or(([0.0; 3], 0.0), |orbit| {
            let parent = world.body(orbit.parent).position;
            (relative_to(eye, parent), orbit.radius as f32)
        });
        Self {
            center: relative_to(eye, body.position),
            radius: body.radius as f32,
            base_color,
            kind,
            accent_color,
            banding,
            orbit_center,
            orbit_radius,
        }
    }
}

/// Subtracting in f64 before converting to f32 keeps precision near the camera,
/// however far it is from the world origin.
fn relative_to(eye: DVec3, point: DVec3) -> [f32; 3] {
    (point - eye).as_vec3().to_array()
}

fn find_light(world: &World) -> Option<DVec3> {
    world
        .bodies()
        .iter()
        .find(|body| matches!(body.appearance, Appearance::Star))
        .map(|star| star.position)
}
