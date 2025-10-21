use crate::core::{calculate_closing_speed, calculate_los_rate};
use crate::entity::{Missile, Target};
use crate::guidance::GuidanceLaw;
use crate::simulation::metrics::SimulationMetrics;

pub struct SimulationEngine {
    pub missile: Missile,
    pub target: Target,
    pub time: f64,
    pub dt: f64,
    pub max_time: f64,
    pub hit_threshold: f64,
}

impl SimulationEngine {
    pub fn new(
        missile: Missile,
        target: Target,
        dt: f64,
        max_time: f64,
        hit_threshold: f64,
    ) -> Self {
        Self {
            missile,
            target,
            time: 0.0,
            dt,
            max_time,
            hit_threshold,
        }
    }

    pub fn run(&mut self, guidance: &dyn GuidanceLaw) -> SimulationMetrics {
        let mut metrics = SimulationMetrics::new();

        // Record initial state
        self.record_metrics(&mut metrics, 0.0);

        while !self.should_terminate(&metrics) {
            self.step(guidance, &mut metrics);
        }

        metrics.finalize(self.hit_threshold);
        metrics
    }

    pub fn step(&mut self, guidance: &dyn GuidanceLaw, metrics: &mut SimulationMetrics) {
        // Calculate guidance command
        let acceleration = guidance.calculate_acceleration(&self.missile, &self.target);

        // Update missile
        self.missile.update(acceleration, self.dt);

        // Update target (constant velocity)
        self.target.update(self.dt);

        // Advance time
        self.time += self.dt;

        // Record metrics
        self.record_metrics(metrics, acceleration.norm());
    }

    fn record_metrics(&self, metrics: &mut SimulationMetrics, accel_magnitude: f64) {
        let los_rate = calculate_los_rate(
            &self.missile.state.position,
            &self.missile.state.velocity,
            &self.target.state.position,
            &self.target.state.velocity,
        )
        .norm();

        let closing_speed = calculate_closing_speed(
            &self.missile.state.position,
            &self.missile.state.velocity,
            &self.target.state.position,
            &self.target.state.velocity,
        );

        metrics.record(
            self.time,
            self.missile.state.position,
            self.missile.state.velocity,
            self.target.state.position,
            self.target.state.velocity,
            accel_magnitude,
            los_rate,
            closing_speed,
        );
    }

    fn should_terminate(&self, metrics: &SimulationMetrics) -> bool {
        if self.time >= self.max_time {
            return true;
        }

        let distance = metrics.distance_history.last().unwrap_or(&f64::INFINITY);

        // Hit threshold
        if *distance < self.hit_threshold {
            return true;
        }

        // Miss (distance increasing rapidly)
        if metrics.distance_history.len() > 10 {
            let recent_dist = metrics.distance_history[metrics.distance_history.len() - 10];
            if *distance > recent_dist + 500.0 {
                return true;
            }
        }

        false
    }
}
