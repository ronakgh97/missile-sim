use crate::entity::{Missile, Target};
use crate::guidance::GuidanceLaw;
use nalgebra::Vector3;

/// Pure Pursuit (PP) Guidance
///
/// Simplest guidance strategy: point velocity vector directly at the target.
/// No lead angle — always chases the target's current position.
///
/// `a_c = lateral_unit * a_max * lateral_norm`
pub struct PurePursuit;

impl GuidanceLaw for PurePursuit {
    #[inline]
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        let range_vec = target.state.position - missile.state.position;
        let range = range_vec.norm();

        if range < 1e-6 {
            return Vector3::zeros();
        }

        // Direction to target (LOS unit vector)
        let range_unit = range_vec / range;

        let missile_speed = missile.state.speed();

        if missile_speed < 1e-6 {
            return range_unit * missile.max_acceleration;
        }

        let velocity_unit = missile.state.velocity / missile_speed;

        // Perpendicular component of desired direction relative to velocity
        let dot_product = velocity_unit.dot(&range_unit);
        let lateral_component = range_unit - velocity_unit * dot_product;

        let lateral_norm = lateral_component.norm();
        if lateral_norm < 1e-12 {
            // Already aligned with target
            return range_unit * missile.max_acceleration;
        }

        let lateral_unit = lateral_component.normalize();

        // Acceleration perpendicular to velocity, scaled by lateral error
        let accel_magnitude = missile.max_acceleration * lateral_norm;

        lateral_unit * accel_magnitude.min(missile.max_acceleration)
    }

    fn name(&self) -> &str {
        "PP"
    }
}
