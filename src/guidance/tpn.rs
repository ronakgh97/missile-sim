use crate::core::{
    calculate_closing_speed_simd, calculate_los_rate_simd, dot_simd, norm_simd, normalize_simd,
};
use crate::entity::{Missile, Target};
use crate::guidance::traits::GuidanceLaw;
use nalgebra::Vector3;

pub struct TrueProportionalNavigation;

impl GuidanceLaw for TrueProportionalNavigation {
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        let los_rate_vector = calculate_los_rate_simd(
            &missile.state.position,
            &missile.state.velocity,
            &target.state.position,
            &target.state.velocity,
        );

        let closing_speed = calculate_closing_speed_simd(
            &missile.state.position,
            &missile.state.velocity,
            &target.state.position,
            &target.state.velocity,
        );

        let safe_closing_speed = if missile.max_closing_speed > 1.0 {
            closing_speed.abs().clamp(1.0, missile.max_closing_speed)
        } else {
            closing_speed.abs().max(1.0)
        };

        let los_rate_magnitude = norm_simd(&los_rate_vector);

        if los_rate_magnitude < 1e-12 {
            return Vector3::zeros();
        }

        // TPN: uses closing speed instead of missile speed
        let velocity_unit = normalize_simd(&missile.state.velocity);
        let los_rate_unit = normalize_simd(&los_rate_vector);

        let dot_product = dot_simd(&velocity_unit, &los_rate_unit);
        let accel_direction = los_rate_unit - velocity_unit * dot_product;
        let accel_direction = normalize_simd(&accel_direction);

        let acceleration_magnitude =
            missile.navigation_constant * safe_closing_speed * los_rate_magnitude;

        let bounded_magnitude = acceleration_magnitude.min(missile.max_acceleration);

        accel_direction * bounded_magnitude
    }

    fn name(&self) -> &str {
        "TPN"
    }
}
