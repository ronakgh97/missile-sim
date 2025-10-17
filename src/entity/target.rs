use crate::core::State3D;
use nalgebra::Vector3;

#[derive(Clone, Debug)]
pub struct TargetConfig {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
}

#[derive(Clone, Debug)]
pub struct Target {
    pub state: State3D,
}

impl Target {
    pub fn new(config: TargetConfig) -> Self {
        Self {
            state: State3D {
                position: config.position,
                velocity: config.velocity,
            },
        }
    }

    pub fn update(&mut self, dt: f64) {
        // Target moves with constant velocity (no acceleration for now)
        self.state.update(Vector3::zeros(), dt);
    }
}
