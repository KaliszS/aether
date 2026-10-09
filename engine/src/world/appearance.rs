#[derive(Clone, Copy, Debug)]
pub enum Appearance {
    Star,
    Planet {
        base: [f32; 3],
        accent: [f32; 3],
        /// 0 = patchy surface (rocky), 1 = latitude bands (gas giant).
        banding: f32,
    },
}
