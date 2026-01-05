use crate::core::{calculate_closing_speed_simd, calculate_los_rate_simd, norm_simd};
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

        // Estimate number of steps for pre-allocation
        let memory_cap: f64 = 128_0000.0; // Cap at 128 MB, BECAUSE IT GONNA BLOW UP 😼
        let steps = ((self.max_time / self.dt).ceil() + 1.0).min(memory_cap) as usize;
        metrics.pre_allocate_steps(steps);

        // Record initial state
        self.record_metrics(&mut metrics, 0.0);

        while !self.should_terminate(&metrics) {
            self.step(guidance, &mut metrics);
        }

        metrics.finalize(self.hit_threshold);
        metrics
    }

    #[inline]
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

    #[inline]
    fn record_metrics(&self, metrics: &mut SimulationMetrics, accel_magnitude: f64) {
        let los_rate_vec = calculate_los_rate_simd(
            &self.missile.state.position,
            &self.missile.state.velocity,
            &self.target.state.position,
            &self.target.state.velocity,
        );
        let los_rate = norm_simd(&los_rate_vec);

        let closing_speed = calculate_closing_speed_simd(
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

    #[inline]
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
