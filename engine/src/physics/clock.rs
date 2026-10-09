/// Simulated time in seconds, running `speed` times faster than real time.
pub struct SimulationClock {
    pub seconds: f64,
    pub speed: f64,
}

impl SimulationClock {
    pub fn new(seconds: f64, speed: f64) -> Self {
        Self { seconds, speed }
    }

    pub fn advance(&mut self, real_dt: f64) {
        self.seconds += real_dt * self.speed;
    }
}
