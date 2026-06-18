use crate::core::State3D;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

/// A missile entity in the simulation.
///
/// Contains both configuration parameters and runtime state.
/// The missile's state is updated each step by the simulation engine
/// using acceleration commands from a guidance law.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Missile {
    /// Current kinematic state (position and velocity).
    pub state: State3D,
    /// Maximum acceleration magnitude (m/s²).
    pub max_acceleration: f64,
    /// Navigation constant (N'), typical values are 3–5.
    /// This affects the aggressiveness of the missile's maneuvers. Higher values lead to more aggressive turns.
    pub navigation_constant: f64,
    /// Maximum closing speed for TPN/APN clamping.
    pub max_closing_speed: f64,
}

impl Missile {
    /// Updates the missile state by applying the given acceleration for `dt` seconds.
    ///
    /// The acceleration is clamped to `max_acceleration` if it exceeds the limit and projected perpendicular to the current velocity
    #[inline(always)]
    pub fn update(&mut self, acceleration: Vector3<f64>, dt: f64) {
        let clamped_accel = if acceleration.norm() > self.max_acceleration {
            acceleration.normalize() * self.max_acceleration
        } else {
            acceleration
        };

        // project acceleration perpendicular to velocity
        let speed = self.state.speed();
        let perp_accel = if speed > 1e-6 {
            let v_hat = self.state.velocity / speed;
            clamped_accel - v_hat * v_hat.dot(&clamped_accel)
        } else {
            clamped_accel
        };

        self.state.update(perp_accel, dt);
    }
}
