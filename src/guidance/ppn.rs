use crate::core::calculate_los_rate;
use crate::entity::{Missile, Target};
use crate::guidance::GuidanceLaw;
use nalgebra::Vector3;

/// Pure Proportional Navigation (PPN) is base PN without closing speed compensation (V_c),
/// acceleration is perpendicular to velocity toward LOS rate direction
///
/// `a_c = N * (w_LOS x V_m) where w_LOS = (R x V_rel) / |R|^2`
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

        let direction = los_rate_vector.cross(&missile.state.velocity);
        if direction.norm_squared() < 1e-12 {
            return Vector3::zeros();
        }

        let accel = direction * missile.navigation_constant;

        if accel.norm() > missile.max_acceleration {
            accel * (missile.max_acceleration / accel.norm())
        } else {
            accel
        }
    }

    fn name(&self) -> &str {
        "PPN"
    }
}
