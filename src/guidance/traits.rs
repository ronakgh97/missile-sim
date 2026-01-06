use crate::entity::{Missile, Target};
use nalgebra::Vector3;

pub(crate) trait GuidanceLaw: Send + Sync {
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64>;

    #[allow(dead_code)]
    fn name(&self) -> &str;
}
