fn star_layer(dir: vec3<f32>, density: f32, fill: f32, pixel: f32) -> vec3<f32> {
    let cell = floor(dir * density);
    let h = hash3(cell + vec3<f32>(density));
    if (h.z > fill) {
        return vec3<f32>(0.0);
    }
    let star_dir = normalize(cell + 0.2 + 0.6 * h);
    let d = length(star_dir - dir) / pixel;
    let tint = mix(vec3<f32>(0.6, 0.72, 1.0), vec3<f32>(1.0, 0.82, 0.62), h.y);
    let twinkle = 0.8 + 0.2 * sin(frame.time * (1.0 + 3.0 * h.y) + h.x * 50.0);
    return tint * pow(h.x, 4.0) * 4.0 * twinkle * exp(-d * d);
}

fn nebula(dir: vec3<f32>) -> vec3<f32> {
    let violet = vec3<f32>(0.3, 0.12, 0.4) * pow(fbm(dir * 2.5), 3.0) * 0.12;
    let blue = vec3<f32>(0.05, 0.12, 0.25) * pow(fbm(dir * 4.0 + 5.0), 4.0) * 0.15;
    return violet + blue;
}

fn sky(dir: vec3<f32>, pixel: f32) -> vec3<f32> {
    let stars = star_layer(dir, 60.0, 0.3, pixel)
        + star_layer(dir, 150.0, 0.2, pixel)
        + star_layer(dir, 400.0, 0.12, pixel) * 0.5;
    return stars + nebula(dir);
}
