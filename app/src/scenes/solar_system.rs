use engine::{Appearance, Body, BodyId, DVec3, Orbit, OrbitCamera, World};

use super::Scene;

const KM: f64 = 1_000.0;
const DAY: f64 = 86_400.0;
const SUN_RADIUS_KM: f64 = 695_700.0;
/// Starting view: far enough to see Neptune's orbit.
const OVERVIEW_DISTANCE: f64 = 1.2e13;
/// J2000.0 epoch (2000-01-01 12:00 TT) as Unix time in milliseconds.
const J2000_UNIX_MS: f64 = 946_728_000_000.0;

/// Mean orbital elements (circular approximation) with mean longitude at J2000.
struct Spec {
    name: &'static str,
    radius_km: f64,
    orbit_km: f64,
    period_days: f64,
    longitude_deg: f64,
    base: [f32; 3],
    accent: [f32; 3],
    banding: f32,
}

#[rustfmt::skip]
const PLANETS: [Spec; 8] = [
    Spec { name: "Mercury", radius_km: 2_439.7, orbit_km: 57_909_050.0, period_days: 87.969, longitude_deg: 252.25, base: [0.55, 0.52, 0.5], accent: [0.35, 0.33, 0.32], banding: 0.0 },
    Spec { name: "Venus", radius_km: 6_051.8, orbit_km: 108_208_000.0, period_days: 224.701, longitude_deg: 181.98, base: [0.93, 0.83, 0.62], accent: [0.8, 0.65, 0.45], banding: 0.3 },
    Spec { name: "Earth", radius_km: 6_371.0, orbit_km: 149_598_023.0, period_days: 365.256, longitude_deg: 100.46, base: [0.1, 0.25, 0.55], accent: [0.25, 0.45, 0.2], banding: 0.0 },
    Spec { name: "Mars", radius_km: 3_389.5, orbit_km: 227_939_200.0, period_days: 686.98, longitude_deg: 355.45, base: [0.75, 0.35, 0.18], accent: [0.5, 0.22, 0.12], banding: 0.0 },
    Spec { name: "Jupiter", radius_km: 69_911.0, orbit_km: 778_570_000.0, period_days: 4_332.59, longitude_deg: 34.40, base: [0.85, 0.75, 0.6], accent: [0.65, 0.45, 0.3], banding: 1.0 },
    Spec { name: "Saturn", radius_km: 58_232.0, orbit_km: 1_433_530_000.0, period_days: 10_759.22, longitude_deg: 49.94, base: [0.9, 0.82, 0.6], accent: [0.75, 0.65, 0.45], banding: 0.8 },
    Spec { name: "Uranus", radius_km: 25_362.0, orbit_km: 2_872_460_000.0, period_days: 30_688.5, longitude_deg: 313.23, base: [0.6, 0.85, 0.9], accent: [0.5, 0.75, 0.85], banding: 0.2 },
    Spec { name: "Neptune", radius_km: 24_622.0, orbit_km: 4_495_060_000.0, period_days: 60_182.0, longitude_deg: 304.88, base: [0.3, 0.45, 0.9], accent: [0.2, 0.3, 0.75], banding: 0.3 },
];

const EARTH_INDEX: usize = 2;

#[rustfmt::skip]
const MOON: Spec = Spec { name: "Moon", radius_km: 1_737.4, orbit_km: 384_399.0, period_days: 27.3217, longitude_deg: 218.32, base: [0.6, 0.6, 0.58], accent: [0.4, 0.4, 0.4], banding: 0.0 };

/// The Solar System seen from afar, following the Sun.
pub fn overview_scene() -> Scene {
    let (world, sun) = solar_system();
    let sun_body = world.body(sun);
    let camera = OrbitCamera::new(sun_body.position, OVERVIEW_DISTANCE, sun_body.radius * 1.3);
    Scene::new(world, camera, Some(sun))
}

/// The Sun, eight planets and the Moon at true scale. Returns the world and the Sun's id.
fn solar_system() -> (World, BodyId) {
    let mut world = World::default();
    let sun = world.add(Body {
        name: "Sun".into(),
        position: DVec3::ZERO,
        radius: SUN_RADIUS_KM * KM,
        appearance: Appearance::Star,
        orbit: None,
    });
    let planets: Vec<BodyId> = PLANETS
        .iter()
        .map(|spec| world.add(spec.body(sun)))
        .collect();
    world.add(MOON.body(planets[EARTH_INDEX]));
    (world, sun)
}

pub fn seconds_since_j2000(unix_ms: f64) -> f64 {
    (unix_ms - J2000_UNIX_MS) / 1000.0
}

impl Spec {
    fn body(&self, parent: BodyId) -> Body {
        Body {
            name: self.name.into(),
            position: DVec3::ZERO,
            radius: self.radius_km * KM,
            appearance: Appearance::Planet {
                base: self.base,
                accent: self.accent,
                banding: self.banding,
            },
            orbit: Some(Orbit {
                parent,
                radius: self.orbit_km * KM,
                period: self.period_days * DAY,
                phase: self.longitude_deg.to_radians(),
            }),
        }
    }
}
