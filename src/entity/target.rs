use crate::core::State3D;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

/// A target entity in the simulation.
///
/// Contains both configuration parameters and runtime state.
/// The target moves with constant acceleration defined in its config.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    /// Current kinematic state (position and velocity).
    pub state: State3D,
    /// Constant acceleration applied each update step.
    pub acceleration: Vector3<f64>,
}

impl Target {
    /// Advances the target state by `dt` seconds using its constant acceleration.
    #[inline(always)]
    pub fn update(&mut self, dt: f64) {
        self.state.update(self.acceleration, dt);
    }
}
