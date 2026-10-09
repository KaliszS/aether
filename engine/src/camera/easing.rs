/// Share of the remaining gap to close this frame; frame-rate independent.
pub fn ease_factor(rate: f64, dt: f64) -> f64 {
    1.0 - (-rate * dt).exp()
}
