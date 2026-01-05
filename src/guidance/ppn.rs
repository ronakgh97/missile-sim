use crate::core::{calculate_los_rate_simd, dot_simd, norm_simd, normalize_simd};
use crate::entity::{Missile, Target};
use crate::guidance::traits::GuidanceLaw;
use nalgebra::Vector3;

pub struct PureProportionalNavigation;

impl GuidanceLaw for PureProportionalNavigation {
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        let los_rate_vector = calculate_los_rate_simd(
            &missile.state.position,
            &missile.state.velocity,
            &target.state.position,
            &target.state.velocity,
        );

        let missile_speed = missile.state.speed();
        let los_rate_magnitude = norm_simd(&los_rate_vector);

        if los_rate_magnitude < 1e-12 {
            return Vector3::zeros();
        }

        // PPN: acceleration perpendicular to velocity, toward LOS rate direction
        let velocity_unit = normalize_simd(&missile.state.velocity);
        let los_rate_unit = normalize_simd(&los_rate_vector);

        // Project LOS rate perpendicular to velocity
        let dot_product = dot_simd(&velocity_unit, &los_rate_unit);
        let accel_direction = los_rate_unit - velocity_unit * dot_product;
        let accel_direction = normalize_simd(&accel_direction);

        let acceleration_magnitude =
            missile.navigation_constant * missile_speed * los_rate_magnitude;

        let bounded_magnitude = acceleration_magnitude.min(missile.max_acceleration);

        accel_direction * bounded_magnitude
    }

    fn name(&self) -> &str {
        "PPN"
    }
}
