struct Frame {
    right: vec3<f32>,
    tan_half_fov: f32,
    up: vec3<f32>,
    time: f32,
    forward: vec3<f32>,
    body_count: u32,
    resolution: vec2<f32>,
    _pad: vec2<f32>,
    // xyz: center relative to the camera, w: radius.
    bodies: array<vec4<f32>, 16>,
};

@group(0) @binding(0) var<uniform> frame: Frame;

struct Hit {
    distance: f32,
    color: vec3<f32>,
    coverage: f32,
};

fn no_hit() -> Hit {
    return Hit(1e30, vec3<f32>(0.0), 0.0);
}

fn nearer(a: Hit, b: Hit) -> Hit {
    if (b.distance < a.distance) {
        return b;
    }
    return a;
}

// Fullscreen triangle.
@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let uv = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
}

fn view_scale() -> f32 {
    return min(frame.resolution.x, frame.resolution.y);
}

fn camera_ray(frag: vec2<f32>) -> vec3<f32> {
    let uv = (frag - 0.5 * frame.resolution) / view_scale() * 2.0;
    return normalize(frame.forward + (frame.right * uv.x - frame.up * uv.y) * frame.tan_half_fov);
}

fn tone_map(color: vec3<f32>) -> vec3<f32> {
    return 1.0 - exp(-color);
}

@fragment
fn fs_main(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
    let rd = camera_ray(frag.xy);
    let pixel = 2.0 * frame.tan_half_fov / view_scale();

    var background = sky(rd, pixel);
    var hit = no_hit();
    for (var i = 0u; i < frame.body_count; i++) {
        let body = frame.bodies[i];
        background += star_glow(rd, body);
        hit = nearer(hit, star_hit(rd, body, pixel));
    }

    return vec4<f32>(tone_map(mix(background, hit.color, hit.coverage)), 1.0);
}
