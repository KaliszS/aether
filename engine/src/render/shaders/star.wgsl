const STAR_POINT_COLOR = vec3<f32>(1.0, 0.9, 0.75);

fn rotate_y(p: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(c * p.x + s * p.z, p.y, -s * p.x + c * p.z);
}

fn star_surface(n: vec3<f32>, rd: vec3<f32>) -> vec3<f32> {
    let t = frame.time;
    let p = rotate_y(n, t * 0.03);
    let granules = fbm(p * 18.0 + vec3<f32>(0.0, t * 0.08, 0.0));
    let spots = smoothstep(0.6, 0.68, fbm(p * 3.0 + 4.0));
    let mu = max(dot(n, -rd), 0.0);
    let limb = mix(vec3<f32>(0.9, 0.45, 0.2), vec3<f32>(1.0), mu) * (0.35 + 0.65 * mu);
    let base = mix(vec3<f32>(1.0, 0.42, 0.1), vec3<f32>(1.0, 0.86, 0.6), granules);
    return base * limb * (1.0 - 0.7 * spots) * 4.0;
}

// `offset` and `r` are in units of the star radius.
fn corona(offset: vec3<f32>, r: f32) -> vec3<f32> {
    let h = max(r - 1.0, 0.0);
    let streamers = fbm(normalize(offset) * 4.0 + vec3<f32>(frame.time * 0.02));
    let inner = exp(-h * 6.0) * (0.6 + 0.8 * streamers);
    let outer = 0.12 / (1.0 + h * h * 8.0);
    return vec3<f32>(1.0, 0.55, 0.25) * (inner * 1.5 + outer);
}

fn star_glow(rd: vec3<f32>, body: Body, s: SphereView, pixel: f32) -> vec3<f32> {
    // Point on the ray nearest to the center, relative to the center.
    let offset = rd * max(dot(body.center, rd), 0.0) - body.center;
    let halo = point_glow(s, pixel, STAR_POINT_COLOR * 0.4, 6.0);
    let core = point_glow(s, pixel, STAR_POINT_COLOR * 6.0, 1.5);
    return corona(offset / body.radius, length(offset) / body.radius) + halo + core;
}
