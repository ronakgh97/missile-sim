use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

/// Configuration parameters for a missile.
///
/// This struct is serializable and can be loaded from JSON or other formats.
/// Use `MissileConfig::builder()` for convenient construction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MissileConfig {
    /// Initial position of the missile.
    pub position: Vector3<f64>,
    /// Initial velocity of the missile.
    pub velocity: Vector3<f64>,
    /// Maximum acceleration magnitude the missile can produce (m/s^2).
    pub max_acceleration: f64,
    /// Navigation constant (N') used in proportional navigation laws.
    /// Typical values are 3–5.
    /// This affects the aggressiveness of the missile's maneuvers. Higher values lead to more aggressive turns.
    pub navigation_constant: f64,
    /// Maximum expected closing speed, used for clamping in TPN/APN.
    pub max_closing_speed: f64,
}

impl MissileConfig {
    /// Creates a builder with sensible defaults.
    pub fn builder() -> MissileConfigBuilder {
        MissileConfigBuilder::new()
    }
}

/// Builder for [`MissileConfig`].
///
/// # Defaults
///
/// | Field | Default |
/// |-------|---------|
/// | position | `(0, 0, 0)` |
/// | velocity | `(0, 0, 0)` |
/// | max_acceleration | `1000.0` |
/// | navigation_constant | `4.0` |
/// | max_closing_speed | `5000.0` |
pub struct MissileConfigBuilder {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
    max_acceleration: f64,
    navigation_constant: f64,
    max_closing_speed: f64,
}

impl MissileConfigBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            position: Vector3::zeros(),
            velocity: Vector3::zeros(),
            max_acceleration: 1000.0,
            navigation_constant: 3.0,
            max_closing_speed: 5000.0,
        }
    }

    /// Sets the initial position.
    pub fn position(mut self, pos: Vector3<f64>) -> Self {
        self.position = pos;
        self
    }

    /// Sets the initial velocity.
    pub fn velocity(mut self, vel: Vector3<f64>) -> Self {
        self.velocity = vel;
        self
    }

    /// Sets the maximum acceleration magnitude.
    pub fn max_acceleration(mut self, val: f64) -> Self {
        self.max_acceleration = val;
        self
    }

    /// Sets the navigation constant (N').
    pub fn navigation_constant(mut self, val: f64) -> Self {
        self.navigation_constant = val;
        self
    }

    /// Sets the maximum closing speed.
    pub fn max_closing_speed(mut self, val: f64) -> Self {
        self.max_closing_speed = val;
        self
    }

    /// Builds the [`MissileConfig`].
    pub fn build(self) -> MissileConfig {
        MissileConfig {
            position: self.position,
            velocity: self.velocity,
            max_acceleration: self.max_acceleration,
            navigation_constant: self.navigation_constant,
            max_closing_speed: self.max_closing_speed,
        }
    }
}

impl Default for MissileConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A missile entity in the simulation.
///
/// Created from a [`MissileConfig`] via `Missile::new(config)`.
/// The missile's state is updated each step by the simulation engine
/// using acceleration commands from a guidance law.
#[derive(Clone, Debug)]
pub struct Missile {
    /// Current kinematic state (position and velocity).
    pub state: crate::core::State3D,
    /// Maximum acceleration magnitude (m/s²).
    pub max_acceleration: f64,
    /// Navigation constant (N').
    pub navigation_constant: f64,
    /// Maximum closing speed for TPN clamping.
    pub max_closing_speed: f64,
}

impl Missile {
    /// Creates a missile from configuration.
    pub fn new(config: MissileConfig) -> Self {
        Self {
            state: crate::core::State3D {
                position: config.position,
                velocity: config.velocity,
            },
            max_acceleration: config.max_acceleration,
            navigation_constant: config.navigation_constant,
            max_closing_speed: config.max_closing_speed,
        }
    }

    /// Updates the missile state by applying the given acceleration for `dt` seconds.
    ///
    /// The acceleration is clamped to `max_acceleration` if it exceeds the limit.
    #[inline]
    pub fn update(&mut self, acceleration: Vector3<f64>, dt: f64) {
        let clamped_accel = if acceleration.norm() > self.max_acceleration {
            acceleration.normalize() * self.max_acceleration
        } else {
            acceleration
        };

        self.state.update(clamped_accel, dt);
    }
}
