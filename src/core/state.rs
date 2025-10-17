use nalgebra::Vector3;

#[derive(Clone, Debug)]
pub struct State3D {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
}

impl State3D {
    pub fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            position: Vector3::new(x, y, z),
            velocity: Vector3::new(vx, vy, vz),
        }
    }

    pub fn update(&mut self, acceleration: Vector3<f64>, dt: f64) {
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
    }

    pub fn speed(&self) -> f64 {
        self.velocity.norm()
    }
}
