use engine::{Body, BodyId, DVec3, OrbitCamera, World};

use super::Scene;

/// Gap between the two bodies, as a fraction of the larger radius.
const GAP_RADII: f64 = 0.3;
/// Extra room around the pair.
const MARGIN: f64 = 1.25;
const MIN_DISTANCE_RADII: f64 = 1.3;

/// The two bodies side by side, at true relative size, framed by a camera looking straight on.
pub fn comparison_scene(world: &World, [a, b]: [BodyId; 2]) -> Scene {
    let (a, b) = (world.body(a), world.body(b));
    let gap = a.radius.max(b.radius) * GAP_RADII;
    let width = 2.0 * (a.radius + b.radius) + gap;
    let left = -width / 2.0;

    let mut pair = World::default();
    pair.add(placed_at(a, left + a.radius));
    pair.add(placed_at(b, left + 2.0 * a.radius + gap + b.radius));

    let distance = OrbitCamera::distance_to_fit(width / 2.0) * MARGIN;
    let min_distance = a.radius.min(b.radius) * MIN_DISTANCE_RADII;
    let camera = OrbitCamera::new(DVec3::ZERO, distance, min_distance).with_pitch(0.0);
    Scene::new(pair, camera, None)
}

/// E.g. "Sun is 109× wider than Earth. Earth would fit inside it about 1.3 million times."
pub fn caption(world: &World, [a, b]: [BodyId; 2]) -> String {
    let (a, b) = (world.body(a), world.body(b));
    let (big, small) = if a.radius >= b.radius { (a, b) } else { (b, a) };
    let ratio = big.radius / small.radius;
    if ratio < 1.05 {
        return format!("{} and {} are about the same size.", big.name, small.name);
    }
    format!(
        "{} is {}× wider than {}. {} would fit inside it about {} times.",
        big.name,
        format_ratio(ratio),
        small.name,
        small.name,
        format_count(ratio.powi(3)),
    )
}

fn placed_at(body: &Body, x: f64) -> Body {
    Body {
        position: DVec3::new(x, 0.0, 0.0),
        orbit: None,
        ..body.clone()
    }
}

fn format_ratio(ratio: f64) -> String {
    if ratio < 10.0 {
        format!("{ratio:.1}")
    } else {
        format!("{ratio:.0}")
    }
}

fn format_count(count: f64) -> String {
    match count {
        c if c >= 1e9 => format!("{:.1} billion", c / 1e9),
        c if c >= 1e6 => format!("{:.1} million", c / 1e6),
        c => group_thousands(c.round() as u64),
    }
}

/// 1321 -> "1,321"
fn group_thousands(n: u64) -> String {
    let digits = n.to_string();
    let mut grouped = String::new();
    for (i, digit) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}
