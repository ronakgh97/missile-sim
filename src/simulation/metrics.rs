use nalgebra::Vector3;

pub struct SimulationMetrics {
    pub missile_trajectory: Vec<Vector3<f64>>,
    pub missile_velocity: Vec<Vector3<f64>>,
    pub target_trajectory: Vec<Vector3<f64>>,
    pub target_velocity: Vec<Vector3<f64>>,
    pub time_history: Vec<f64>,
    pub distance_history: Vec<f64>,
    pub acceleration_history: Vec<f64>,
    pub los_rate_history: Vec<f64>,

    /// Closing speed (rate of range decrease) - recorded for all guidance laws
    /// for analysis purposes. Used directly by TPN, recorded for PPN comparison.
    pub closing_speed_history: Vec<f64>,
    pub hit: bool,
    pub miss_distance: f64,
}

impl SimulationMetrics {
    pub fn new() -> Self {
        Self {
            missile_trajectory: Vec::new(),
            missile_velocity: Vec::new(),
            target_trajectory: Vec::new(),
            target_velocity: Vec::new(),
            time_history: Vec::new(),
            distance_history: Vec::new(),
            acceleration_history: Vec::new(),
            los_rate_history: Vec::new(),
            closing_speed_history: Vec::new(),
            hit: false,
            miss_distance: f64::INFINITY,
        }
    }

    /// Pre-allocate memory for all metric vectors
    pub fn pre_allocate_steps(&mut self, steps: usize) {
        self.missile_trajectory.reserve(steps);
        self.missile_velocity.reserve(steps);
        self.target_trajectory.reserve(steps);
        self.target_velocity.reserve(steps);
        self.time_history.reserve(steps);
        self.distance_history.reserve(steps);
        self.acceleration_history.reserve(steps);
        self.los_rate_history.reserve(steps);
        self.closing_speed_history.reserve(steps);
    }

    pub fn record(
        &mut self,
        time: f64,
        missile_pos: Vector3<f64>,
        missile_velocity: Vector3<f64>,
        target_pos: Vector3<f64>,
        target_velocity: Vector3<f64>,
        acceleration: f64,
        los_rate: f64,
        closing_speed: f64,
    ) {
        let distance = (missile_pos - target_pos).norm();

        self.time_history.push(time);
        self.missile_trajectory.push(missile_pos);
        self.missile_velocity.push(missile_velocity);
        self.target_trajectory.push(target_pos);
        self.target_velocity.push(target_velocity);
        self.distance_history.push(distance);
        self.acceleration_history.push(acceleration);
        self.los_rate_history.push(los_rate);
        self.closing_speed_history.push(closing_speed);

        // Track minimum miss distance
        if distance < self.miss_distance {
            self.miss_distance = distance;
        }
    }

    pub fn finalize(&mut self, hit_threshold: f64) {
        self.hit = self.miss_distance < hit_threshold;
    }

    pub fn console_print(&self) -> String {
        format!(
            "Travel Duration: {:.2} | Miss Distance: {:.2} | Hit: {}",
            self.time_history.last().unwrap_or(&0.0),
            self.miss_distance,
            if self.hit { "1" } else { "0" },
        )
    }
}

impl Default for SimulationMetrics {
    fn default() -> Self {
        Self::new()
    }
}
