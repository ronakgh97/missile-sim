use serde::{Deserialize, Serialize};

/// Configuration parameters for a target.
///
/// Supports maneuvering targets via the `acceleration` field.
/// Use `TargetConfig::builder()` for convenient construction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetConfig {
    /// Initial position of the target.
    pub position: nalgebra::Vector3<f64>,
    /// Initial velocity of the target.
    pub velocity: nalgebra::Vector3<f64>,
    /// Constant acceleration applied to the target each step.
    /// Set to `Vector3::zeros()` for a non-maneuvering target.
    pub acceleration: nalgebra::Vector3<f64>,
}

impl TargetConfig {
    /// Creates a builder with all fields defaulting to zero vectors.
    pub fn builder() -> TargetConfigBuilder {
        TargetConfigBuilder::new()
    }
}

/// Builder for [`TargetConfig`].
///
/// All fields default to zero vectors.
pub struct TargetConfigBuilder {
    position: nalgebra::Vector3<f64>,
    velocity: nalgebra::Vector3<f64>,
    acceleration: nalgebra::Vector3<f64>,
}

impl TargetConfigBuilder {
    /// Creates a new builder with zero position, velocity, and acceleration.
    pub fn new() -> Self {
        Self {
            position: nalgebra::Vector3::zeros(),
            velocity: nalgebra::Vector3::zeros(),
            acceleration: nalgebra::Vector3::zeros(),
        }
    }

    /// Sets the initial position.
    pub fn position(mut self, pos: nalgebra::Vector3<f64>) -> Self {
        self.position = pos;
        self
    }

    /// Sets the initial velocity.
    pub fn velocity(mut self, vel: nalgebra::Vector3<f64>) -> Self {
        self.velocity = vel;
        self
    }

    /// Sets the constant acceleration for maneuvering targets.
    pub fn acceleration(mut self, accel: nalgebra::Vector3<f64>) -> Self {
        self.acceleration = accel;
        self
    }

    /// Builds the [`TargetConfig`].
    pub fn build(self) -> TargetConfig {
        TargetConfig {
            position: self.position,
            velocity: self.velocity,
            acceleration: self.acceleration,
        }
    }
}

impl Default for TargetConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A target entity in the simulation.
///
/// Created from a [`TargetConfig`] via `Target::new(config)`.
/// The target moves with constant acceleration defined in its config.
#[derive(Clone, Debug)]
pub struct Target {
    /// Current kinematic state (position and velocity).
    pub state: crate::core::State3D,
    /// Constant acceleration applied each update step.
    pub acceleration: nalgebra::Vector3<f64>,
}

impl Target {
    /// Creates a target from configuration.
    pub fn new(config: TargetConfig) -> Self {
        Self {
            state: crate::core::State3D {
                position: config.position,
                velocity: config.velocity,
            },
            acceleration: config.acceleration,
        }
    }

    /// Advances the target state by `dt` seconds using its constant acceleration.
    pub fn update(&mut self, dt: f64) {
        self.state.update(self.acceleration, dt);
    }
}
