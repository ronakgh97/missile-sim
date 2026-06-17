use crate::core::{calculate_closing_speed, calculate_los_rate};
use crate::entity::{Missile, Target};
use crate::guidance::GuidanceLaw;
use crate::simulation::metrics::SimulationMetrics;

/// The simulation engine that runs the missile-target engagement loop.
///
/// Use [`SimulationEngine::new`] to create an engine, then call [`run`] for
/// a complete simulation or [`step`] for manual control (useful for game loops).
///
/// For most use cases, prefer [`Scenario::simulate`] which handles engine
/// creation internally.
pub struct SimulationEngine {
    /// The missile entity.
    pub missile: Missile,
    /// The target entity.
    pub target: Target,
    /// Current simulation time in seconds.
    pub time: f64,
    /// Timestep in seconds.
    pub dt: f64,
    /// Maximum simulation duration in seconds.
    pub max_time: f64,
    /// Distance threshold for hit detection.
    pub hit_threshold: f64,
}

impl SimulationEngine {
    /// Creates a new simulation engine.
    ///
    /// * `missile` — The missile entity.
    /// * `target` — The target entity.
    /// * `dt` — Simulation timestep in seconds.
    /// * `max_time` — Maximum simulation duration.
    /// * `hit_threshold` — Distance below which the engagement is a hit.
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

    /// Runs the simulation to completion with the given guidance law.
    ///
    /// The loop terminates when:
    /// - Time exceeds `max_time`
    /// - Distance drops below `hit_threshold` (hit)
    /// - Distance increases rapidly (miss — target escaping)
    pub fn run(&mut self, guidance: &dyn GuidanceLaw) -> SimulationMetrics {
        const PRE_ALLOC: f64 = 768_000.0;
        let steps = ((self.max_time / self.dt).ceil() + 1.0).min(PRE_ALLOC) as usize;

        let mut metrics = SimulationMetrics::init(steps);
        self.record_metrics(&mut metrics, 0.0);

        // Step loop till terminate
        while !self.should_terminate(&metrics) {
            self.step(guidance, &mut metrics);
        }

        metrics.finalize(self.hit_threshold);
        metrics
    }

    /// Performs a single simulation step and records metrics.
    ///
    /// Use this method for real-time/game-loop style control where you
    /// want to advance the simulation one frame at a time and render
    /// or process data between steps.
    #[inline(always)]
    pub fn step(&mut self, guidance: &dyn GuidanceLaw, metrics: &mut SimulationMetrics) {
        let acceleration = guidance.calculate_acceleration(&self.missile, &self.target);

        // update entities with the calculated acceleration
        self.missile.update(acceleration, self.dt);
        self.target.update(self.dt);
        // advance time
        self.time += self.dt;
        // record
        self.record_metrics(metrics, acceleration.norm());
    }

    #[inline(always)]
    fn record_metrics(&self, metrics: &mut SimulationMetrics, accel_magnitude: f64) {
        let los_rate_vec = calculate_los_rate(
            &self.missile.state.position,
            &self.missile.state.velocity,
            &self.target.state.position,
            &self.target.state.velocity,
        );
        let los_rate = los_rate_vec.norm();

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

    /// Determine if the simulation should terminate
    #[inline(always)]
    fn should_terminate(&self, metrics: &SimulationMetrics) -> bool {
        if self.time >= self.max_time {
            return true;
        }

        let distance = metrics.distance_records.last().unwrap_or(&f64::INFINITY);

        // Hit threshold
        if *distance < self.hit_threshold {
            return true;
        }

        if metrics.distance_records.len() > 10 {
            let recent_dist = metrics.distance_records[metrics.distance_records.len() - 10];
            if *distance > recent_dist + 500.0 {
                return true;
            }
        }

        false
    }
}
