use bevy::prelude::*;

/// Velocity component
#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity(pub Vec3);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

/// Acceleration component
#[derive(Component, Debug, Clone, Copy)]
pub struct Acceleration(pub Vec3);

impl Default for Acceleration {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

/// Angular velocity (rotation speed)
#[derive(Component, Debug, Clone, Copy)]
pub struct AngularVelocity(pub Vec3);

impl Default for AngularVelocity {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}
