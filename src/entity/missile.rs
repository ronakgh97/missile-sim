use crate::core::State3D;
use nalgebra::Vector3;

#[derive(Clone, Debug)]
pub struct MissileConfig {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub max_acceleration: f64,
    pub navigation_constant: f64,
    pub max_closing_speed: f64,
}

#[derive(Clone, Debug)]
pub struct Missile {
    pub state: State3D,
    pub max_acceleration: f64,
    pub navigation_constant: f64,
    pub max_closing_speed: f64,
}

impl Missile {
    pub fn new(config: MissileConfig) -> Self {
        Self {
            state: State3D {
                position: config.position,
                velocity: config.velocity,
            },
            max_acceleration: config.max_acceleration,
            navigation_constant: config.navigation_constant,
            max_closing_speed: config.max_closing_speed,
        }
    }

    pub fn update(&mut self, acceleration: Vector3<f64>, dt: f64) {
        // Clamp acceleration to max
        let clamped_accel = if acceleration.norm() > self.max_acceleration {
            acceleration.normalize() * self.max_acceleration
        } else {
            acceleration
        };

        self.state.update(clamped_accel, dt);
    }
}
