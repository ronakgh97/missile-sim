use crate::entity::{Missile, Target};
use crate::guidance::GuidanceLaw;
use crate::simulation::engine::SimulationEngine;
use crate::simulation::metrics::SimulationMetrics;

/// A complete missile-target engagement scenario.
/// Contains all configuration needed to run a simulation: missile and target
/// parameters, timestep, guidance laws, duration, and hit threshold.
/// Use [`Scenario::builder`] for construction, or construct directly.
///
/// ```rust
/// use missile_sim::prelude::*;
///
/// fn main() {
///   let scenario = Scenario::builder("intercept")
///      .missile(Missile {
///         state: State3D {
///            position: Vector3::new(0.0, 0.0, 0.0),
///            velocity: Vector3::new(100.0, 0.0, 0.0),
///         },
///         max_acceleration: 30.0,
///         navigation_constant: 4.0,
///         max_closing_speed: 1000.0,
///      })
///      .target(Target {
///         state: State3D {
///            position: Vector3::new(1000.0, 0.0, 0.0),
///            velocity: Vector3::new(0.0, 0.0, 0.0),
///         },
///         acceleration: Vector3::zeros(),
///      })
///      .build()
///      .unwrap();
///
///  // this simulates the scenario and store the metrics
///  let metrics = scenario.simulate(&PureProportionalNavigation);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Scenario {
    /// Human-readable name for this scenario.
    pub name: String,
    /// Missile entity.
    pub missile: Missile,
    /// Target entity.
    pub target: Target,
    /// Simulation timestep in seconds.
    /// Smaller values give more accuracy at the cost of performance.
    /// Typical values: 0.0001–0.01.
    pub dt: f64,
    /// Maximum simulation duration in seconds.
    pub total_time: f64,
    /// Distance threshold below which the engagement is considered a hit.
    pub hit_threshold: f64,
}

impl Scenario {
    /// Runs the simulation with the given guidance law and returns metrics.
    /// This is a convenience method that creates a [`SimulationEngine`] internally
    /// and runs it to completion.
    ///
    /// `guidance` — Any type implementing [`GuidanceLaw`], such as one of the
    ///  built-in laws (e.g., `PureProportionalNavigation`) or a custom implementation.
    pub fn simulate(&self, guidance: &dyn GuidanceLaw) -> SimulationMetrics {
        let mut engine = SimulationEngine {
            missile: self.missile.clone(),
            target: self.target.clone(),
            time: 0.0,
            dt: self.dt,
            max_time: self.total_time,
            hit_threshold: self.hit_threshold,
        };

        engine.run(guidance)
    }

    /// Creates a new [`ScenarioBuilder`] with the given name.
    pub fn builder(name: &str) -> ScenarioBuilder {
        ScenarioBuilder::new(name)
    }
}

/// Builder for [`Scenario`].
/// * dt - `0.01`
/// * total_time - `60.0`
/// * hit_threshold - `5.0`
pub struct ScenarioBuilder {
    name: String,
    missile: Option<Missile>,
    target: Option<Target>,
    dt: f64,
    total_time: f64,
    hit_threshold: f64,
}

impl ScenarioBuilder {
    /// Creates a new builder with the given scenario name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            missile: None,
            target: None,
            dt: 0.01,
            total_time: 60.0,
            hit_threshold: 5.0,
        }
    }

    /// Sets the missile.
    pub fn missile(mut self, missile: Missile) -> Self {
        self.missile = Some(missile);
        self
    }

    /// Sets the target.
    pub fn target(mut self, target: Target) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets the simulation timestep in seconds.
    pub fn dt(mut self, dt: f64) -> Self {
        self.dt = dt;
        self
    }

    /// Sets the maximum simulation duration in seconds.
    pub fn total_time(mut self, total_time: f64) -> Self {
        self.total_time = total_time;
        self
    }

    /// Sets the hit threshold distance.
    pub fn hit_threshold(mut self, threshold: f64) -> Self {
        self.hit_threshold = threshold;
        self
    }

    /// Builds the scenario. Returns an error if missile or target is missing.
    pub fn build(self) -> anyhow::Result<Scenario> {
        Ok(Scenario {
            name: self.name,
            missile: self
                .missile
                .ok_or_else(|| anyhow::anyhow!("missile is required"))?,
            target: self
                .target
                .ok_or_else(|| anyhow::anyhow!("target is required"))?,
            dt: self.dt,
            total_time: self.total_time,
            hit_threshold: self.hit_threshold,
        })
    }
}
