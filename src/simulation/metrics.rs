use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

/// Time-series metrics collected during a simulation run. (Can be de/serialized for logging or analysis.)
///
/// All trajectory and state data is stored as parallel vectors indexed by timestep.
/// After simulation completes, `hit` and `miss_distance` contain the final result.
#[derive(Serialize, Deserialize)]
pub struct SimulationMetrics {
    /// Missile positions at each timestep.
    pub missile_trajectory: Vec<Vector3<f64>>,
    /// Missile velocities at each timestep.
    pub missile_velocity: Vec<Vector3<f64>>,
    /// Target positions at each timestep.
    pub target_trajectory: Vec<Vector3<f64>>,
    /// Target velocities at each timestep.
    pub target_velocity: Vec<Vector3<f64>>,
    /// Simulation time at each timestep.
    pub time_history: Vec<f64>,
    /// Distance between missile and target at each timestep.
    pub distance_records: Vec<f64>,
    /// Missile acceleration magnitude at each timestep.
    pub acceleration_records: Vec<f64>,
    /// Line-of-sight rate magnitude at each timestep.
    pub los_rate_records: Vec<f64>,
    /// Closing speed at each timestep.
    pub closing_speed_records: Vec<f64>,
    /// Whether the engagement resulted in a hit.
    pub hit: bool,
    /// Minimum distance achieved during the engagement.
    pub miss_distance: f64,
}

impl Default for SimulationMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationMetrics {
    /// Creates empty metrics ready for recording.
    pub fn new() -> Self {
        Self {
            missile_trajectory: Vec::new(),
            missile_velocity: Vec::new(),
            target_trajectory: Vec::new(),
            target_velocity: Vec::new(),
            time_history: Vec::new(),
            distance_records: Vec::new(),
            acceleration_records: Vec::new(),
            los_rate_records: Vec::new(),
            closing_speed_records: Vec::new(),
            hit: false,
            miss_distance: f64::INFINITY,
        }
    }

    /// Pre-allocates capacity for all vectors to avoid reallocations.
    pub fn pre_allocate_steps(&mut self, steps: usize) {
        self.missile_trajectory.reserve(steps);
        self.missile_velocity.reserve(steps);
        self.target_trajectory.reserve(steps);
        self.target_velocity.reserve(steps);
        self.time_history.reserve(steps);
        self.distance_records.reserve(steps);
        self.acceleration_records.reserve(steps);
        self.los_rate_records.reserve(steps);
        self.closing_speed_records.reserve(steps);
    }

    #[allow(clippy::too_many_arguments)]
    /// Records a single timestep's data.
    pub fn record(
        &mut self,
        time: f64,
        missile_pos: Vector3<f64>,
        missile_vel: Vector3<f64>,
        target_pos: Vector3<f64>,
        target_vel: Vector3<f64>,
        accel: f64,
        los_rate: f64,
        closing_speed: f64,
    ) {
        let distance = (missile_pos - target_pos).norm();

        self.time_history.push(time);
        self.missile_trajectory.push(missile_pos);
        self.missile_velocity.push(missile_vel);
        self.target_trajectory.push(target_pos);
        self.target_velocity.push(target_vel);
        self.distance_records.push(distance);
        self.acceleration_records.push(accel);
        self.los_rate_records.push(los_rate);
        self.closing_speed_records.push(closing_speed);

        if distance < self.miss_distance {
            self.miss_distance = distance;
        }
    }

    /// Finalizes the metrics by determining hit/miss based on the threshold.
    pub fn finalize(&mut self, hit_threshold: f64) {
        self.hit = self.miss_distance < hit_threshold;
    }

    /// Returns a one-line summary of the simulation result.
    pub fn console_summary(&self) -> String {
        format!(
            "Duration: {:.2}s | Miss Distance: {:.2} | Hit: {}",
            self.time_history.last().unwrap_or(&0.0),
            self.miss_distance,
            if self.hit { "YES" } else { "NO" },
        )
    }
}
