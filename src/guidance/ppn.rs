use crate::core::calculate_los_rate;
use crate::entity::{Missile, Target};
use crate::guidance::traits::GuidanceLaw;
use nalgebra::Vector3;

/// Pure Proportional Navigation (PPN)
/// Base PN without closing speed compensation, acceleration perpendicular to velocity toward LOS rate direction
pub struct PureProportionalNavigation;

impl GuidanceLaw for PureProportionalNavigation {
    #[inline]
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        let los_rate_vector = calculate_los_rate(
            &missile.state.position,
            &missile.state.velocity,
            &target.state.position,
            &target.state.velocity,
        );

        let missile_speed = missile.state.speed();
        let los_rate_magnitude = los_rate_vector.norm();

        if los_rate_magnitude < 1e-12 {
            return Vector3::zeros();
        }

        // PPN; acceleration perpendicular to velocity, toward LOS rate direction
        let velocity_unit = missile.state.velocity.normalize();
        let los_rate_unit = los_rate_vector.normalize();

        // project LOS rate perpendicular to velocity
        let dot_product = velocity_unit.dot(&los_rate_unit);
        let accel_direction = los_rate_unit - velocity_unit * dot_product;
        let accel_direction = accel_direction.normalize();

        let acceleration_magnitude =
            missile.navigation_constant * missile_speed * los_rate_magnitude;

        let bounded_magnitude = acceleration_magnitude.min(missile.max_acceleration);

        accel_direction * bounded_magnitude
    }

    fn name(&self) -> &str {
        "PPN"
    }
}
