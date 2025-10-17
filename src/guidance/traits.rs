use crate::entity::{Missile, Target};
use nalgebra::Vector3;

pub trait GuidanceLaw: Send + Sync {
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64>;

    fn name(&self) -> &str;
}
