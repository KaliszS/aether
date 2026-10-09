struct Globals {
    resolution: vec2<f32>,
    time: f32,
    _pad: f32,
};

@group(0) @binding(0) var<uniform> g: Globals;

// Fullscreen triangle.
@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let uv = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
}

fn hash2(p: vec2<f32>) -> vec2<f32> {
    let q = vec2<f32>(dot(p, vec2<f32>(127.1, 311.7)), dot(p, vec2<f32>(269.5, 183.3)));
    return fract(sin(q) * 43758.5453);
}

fn star_layer(p: vec2<f32>, density: f32, t: f32) -> vec3<f32> {
    let q = p * density;
    let cell = floor(q);
    let h = hash2(cell);
    let d = length(fract(q) - 0.5 - (h - 0.5) * 0.8);
    let brightness = pow(h.x, 12.0) * 3.0;
    let twinkle = 0.7 + 0.3 * sin(t * (0.5 + 2.0 * h.y) + h.x * 40.0);
    let tint = mix(vec3<f32>(0.65, 0.75, 1.0), vec3<f32>(1.0, 0.85, 0.7), h.y);
    return tint * brightness * twinkle * exp(-d * d * 1600.0);
}

@fragment
fn fs_main(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
    let p = (frag.xy - 0.5 * g.resolution) / g.resolution.y;
    let t = g.time;

    var col = star_layer(p, 30.0, t) + star_layer(p + 7.3, 60.0, t) + star_layer(p - 3.1, 120.0, t);

    let radius = 0.06 * (1.0 + 0.01 * sin(t * 1.5));
    let r = length(p);
    let x = clamp(r / radius, 0.0, 1.0);
    let limb = 1.0 - 0.6 * (1.0 - sqrt(1.0 - x * x));
    let disc = smoothstep(1.0, 0.98, r / radius);
    let angle = atan2(p.y, p.x);
    let rays = 1.0 + 0.25 * pow(abs(sin(angle * 3.0 + t * 0.1)), 16.0);
    let glow = 0.0025 / (r * r + 0.0001) * rays;

    col = mix(col, vec3<f32>(1.0, 0.9, 0.75) * 4.0 * limb, disc);
    col += vec3<f32>(1.0, 0.6, 0.3) * glow;

    return vec4<f32>(1.0 - exp(-col), 1.0);
}
