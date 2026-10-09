// A sphere described by angles as seen from the camera. Unlike plain distances,
// this stays precise in f32 when the sphere is tiny and astronomically far away.
struct SphereView {
    distance: f32,
    dir: vec3<f32>,
    // cos and sin of the angle between the ray and the direction to the center.
    along: f32,
    perp: f32,
    // radius / distance: sin of the angular radius.
    size: f32,
};

struct SphereHit {
    distance: f32,
    normal: vec3<f32>,
    coverage: f32,
};

fn view_sphere(rd: vec3<f32>, center: vec3<f32>, radius: f32) -> SphereView {
    let distance = length(center);
    let dir = center / distance;
    return SphereView(distance, dir, dot(rd, dir), length(cross(rd, dir)), radius / distance);
}

fn sphere_hit(rd: vec3<f32>, s: SphereView, pixel: f32) -> SphereHit {
    if (s.along <= 0.0 || s.perp >= s.size || s.size >= 1.0) {
        return SphereHit(0.0, vec3<f32>(0.0), 0.0);
    }
    let t = s.along - sqrt(s.size * s.size - s.perp * s.perp);
    let normal = (rd * t - s.dir) / s.size;
    let coverage = clamp((s.size - s.perp) / pixel + 0.5, 0.0, 1.0);
    return SphereHit(t * s.distance, normal, coverage);
}

// Keeps a sphere smaller than a pixel visible as a soft dot.
fn point_glow(s: SphereView, pixel: f32, color: vec3<f32>, width: f32) -> vec3<f32> {
    if (s.along <= 0.0) {
        return vec3<f32>(0.0);
    }
    let fade = 1.0 - smoothstep(0.5, 2.0, s.size / pixel);
    let d = s.perp / (pixel * width);
    return color * fade * exp(-d * d);
}
