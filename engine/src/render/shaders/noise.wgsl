// Integer hash (PCG3D): stable on mobile GPUs, unlike sin()-based hashes.
fn pcg3d(input: vec3<u32>) -> vec3<u32> {
    var v = input * 1664525u + 1013904223u;
    v.x += v.y * v.z;
    v.y += v.z * v.x;
    v.z += v.x * v.y;
    v ^= v >> vec3<u32>(16u);
    v.x += v.y * v.z;
    v.y += v.z * v.x;
    v.z += v.x * v.y;
    return v;
}

fn hash3(cell: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(pcg3d(bitcast<vec3<u32>>(vec3<i32>(floor(cell))))) / 4294967295.0;
}

fn value_noise(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    let x00 = mix(hash3(i).x, hash3(i + vec3<f32>(1.0, 0.0, 0.0)).x, u.x);
    let x10 = mix(hash3(i + vec3<f32>(0.0, 1.0, 0.0)).x, hash3(i + vec3<f32>(1.0, 1.0, 0.0)).x, u.x);
    let x01 = mix(hash3(i + vec3<f32>(0.0, 0.0, 1.0)).x, hash3(i + vec3<f32>(1.0, 0.0, 1.0)).x, u.x);
    let x11 = mix(hash3(i + vec3<f32>(0.0, 1.0, 1.0)).x, hash3(i + vec3<f32>(1.0, 1.0, 1.0)).x, u.x);
    return mix(mix(x00, x10, u.y), mix(x01, x11, u.y), u.z);
}

// Fractal Brownian motion: octaves of noise at doubling frequency.
fn fbm(start: vec3<f32>) -> f32 {
    var p = start;
    var amplitude = 0.5;
    var sum = 0.0;
    for (var i = 0; i < 5; i++) {
        sum += amplitude * value_noise(p);
        p = p * 2.03 + vec3<f32>(17.1);
        amplitude *= 0.5;
    }
    return sum;
}
