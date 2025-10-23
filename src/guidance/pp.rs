use crate::entity::{Missile, Target};
use crate::guidance::traits::GuidanceLaw;
use nalgebra::Vector3;

/// Pure Pursuit (Chase) Guidance
pub struct PurePursuit;

impl GuidanceLaw for PurePursuit {
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        let range_vec = target.state.position - missile.state.position;
        let range = range_vec.norm();

        if range < 1e-6 {
            return Vector3::zeros();
        }

        // Direction to target
        let range_unit = range_vec / range;

        // Current velocity direction
        let missile_speed = missile.state.speed();

        if missile_speed < 1e-6 {
            // just accelerate toward target
            return range_unit * missile.max_acceleration;
        }

        let velocity_unit = missile.state.velocity / missile_speed;

        // Compute required turn direction
        // Perpendicular component of desired direction
        let lateral_component = range_unit - velocity_unit * velocity_unit.dot(&range_unit);

        if lateral_component.norm() < 1e-12 {
            // Already aligned
            return range_unit * missile.max_acceleration;
        }

        let lateral_unit = lateral_component.normalize();

        // Acceleration perpendicular to velocity
        let accel_magnitude =
            missile.navigation_constant * missile_speed * lateral_component.norm();

        lateral_unit * accel_magnitude.min(missile.max_acceleration)
    }

    fn name(&self) -> &str {
        "PP"
    }
}
