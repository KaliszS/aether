const ORBIT_COLOR = vec3<f32>(0.3, 0.45, 0.7);

// Circular orbit in the horizontal plane through its center, drawn as a ~1px line.
fn orbit_line(rd: vec3<f32>, body: Body, pixel: f32) -> vec3<f32> {
    if (body.orbit_radius <= 0.0 || abs(rd.y) < 1e-6) {
        return vec3<f32>(0.0);
    }
    let t = body.orbit_center.y / rd.y;
    if (t <= 0.0) {
        return vec3<f32>(0.0);
    }
    let q = rd * t - body.orbit_center;
    let r = length(q.xz);
    // Distance to the circle, converted to screen pixels (shrinks when seen edge-on).
    let radial = vec3<f32>(q.x, 0.0, q.z) / max(r, 1e-6);
    let foreshortening = sqrt(max(1.0 - pow(dot(radial, rd), 2.0), 0.0));
    let px = abs(r - body.orbit_radius) * foreshortening / (t * pixel);
    return ORBIT_COLOR * 0.35 * exp(-px * px);
}
