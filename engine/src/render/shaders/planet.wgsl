const SUNLIGHT: f32 = 2.0;
const AMBIENT: f32 = 0.015;

fn planet_pattern(n: vec3<f32>, body: Body) -> f32 {
    let seed = body.base_color * 10.0;
    let patches = smoothstep(0.42, 0.58, fbm(n * 3.0 + seed));
    let bands = 0.5 + 0.5 * sin(n.y * 20.0 + fbm(n * 4.0 + seed) * 4.0);
    return mix(patches, bands, body.banding);
}

fn planet_surface(n: vec3<f32>, body: Body, light_dir: vec3<f32>) -> vec3<f32> {
    let albedo = mix(body.base_color, body.accent_color, planet_pattern(n, body));
    let diffuse = max(dot(n, light_dir), 0.0);
    return albedo * (diffuse * SUNLIGHT + AMBIENT);
}

fn planet_glow(body: Body, s: SphereView, pixel: f32) -> vec3<f32> {
    return point_glow(s, pixel, body.base_color * 2.0, 1.0);
}
