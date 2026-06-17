use nalgebra::Vector3;

/// The kinematic state of an entity in 3D space.
/// 
/// Holds position and velocity vectors. Updated each simulation step
/// by applying acceleration and integrating forward in time.
#[derive(Clone, Debug)]
pub struct State3D {
    /// Current position in world coordinates.
    pub position: Vector3<f64>,
    /// Current velocity vector.
    pub velocity: Vector3<f64>,
}

impl State3D {
    /// Init a state from individual components.
    /// * `x`, `y`, `z` — Position coordinates.
    /// * `vx`, `vy`, `vz` — Velocity components.
    pub fn init(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            position: Vector3::new(x, y, z),
            velocity: Vector3::new(vx, vy, vz),
        }
    }

    /// Advances the state forward by `dt` seconds under the given acceleration.
    ///
    /// Uses `semi-implicit Euler`: position is updated first using the current
    /// velocity, then velocity is updated with the acceleration.
    #[inline(always)]
    pub fn update(&mut self, acceleration: Vector3<f64>, dt: f64) {
        self.position += self.velocity * dt;
        self.velocity += acceleration * dt;
    }

    /// Returns the scalar speed (magnitude of the velocity vector).
    pub fn speed(&self) -> f64 {
        self.velocity.norm()
    }
}
