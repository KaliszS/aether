fn rotate_y(p: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(c * p.x + s * p.z, p.y, -s * p.x + c * p.z);
}

// Point on the ray nearest to the center, relative to the center.
fn closest_approach(rd: vec3<f32>, center: vec3<f32>) -> vec3<f32> {
    return rd * max(dot(center, rd), 0.0) - center;
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

fn star_glow(rd: vec3<f32>, body: vec4<f32>) -> vec3<f32> {
    let offset = closest_approach(rd, body.xyz) / body.w;
    return corona(offset, length(offset));
}

fn star_hit(rd: vec3<f32>, body: vec4<f32>, pixel: f32) -> Hit {
    let center = body.xyz;
    let radius = body.w;
    let along = dot(center, rd);
    let h = along * along - dot(center, center) + radius * radius;
    if (along <= 0.0 || h <= 0.0) {
        return no_hit();
    }
    let t = along - sqrt(h);
    if (t <= 0.0) {
        return no_hit();
    }
    let r = length(closest_approach(rd, center)) / radius;
    let edge_width = pixel * length(center) / radius;
    let coverage = clamp((1.0 - r) / edge_width + 0.5, 0.0, 1.0);
    let n = (rd * t - center) / radius;
    return Hit(t, star_surface(n, rd), coverage);
}
