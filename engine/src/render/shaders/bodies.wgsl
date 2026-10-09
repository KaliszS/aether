fn light_dir(p: vec3<f32>) -> vec3<f32> {
    if (frame.has_light == 0u) {
        return -normalize(p);
    }
    return normalize(frame.light - p);
}

fn body_glow(rd: vec3<f32>, body: Body, s: SphereView, pixel: f32) -> vec3<f32> {
    if (body.kind == KIND_STAR) {
        return star_glow(rd, body, s, pixel);
    }
    return planet_glow(body, s, pixel);
}

fn body_hit(rd: vec3<f32>, body: Body, s: SphereView, pixel: f32) -> Hit {
    let g = sphere_hit(rd, s, pixel);
    if (g.coverage <= 0.0) {
        return no_hit();
    }
    var color: vec3<f32>;
    if (body.kind == KIND_STAR) {
        color = star_surface(g.normal, rd);
    } else {
        color = planet_surface(g.normal, body, light_dir(rd * g.distance));
    }
    return Hit(g.distance, color, g.coverage);
}
