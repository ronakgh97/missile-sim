use crate::core::{calculate_closing_speed, calculate_los_rate};
use crate::entity::{Missile, Target};
use crate::guidance::GuidanceLaw;
use nalgebra::Vector3;

/// True Proportional Navigation (TPN)
///
/// Uses closing speed instead of missile speed for the acceleration command.
/// The acceleration is perpendicular to the missile's velocity and directed
/// toward the line-of-sight (LOS) rate vector.
///
/// `a_c = N * (V_c) * λ̇`
///
/// TPN is more effective against maneuvering targets than PPN because it
/// accounts for the actual rate of range closure.
pub struct TrueProportionalNavigation;

impl GuidanceLaw for TrueProportionalNavigation {
    #[inline]
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        let los_vector = target.state.position - missile.state.position;
        if los_vector.norm_squared() < 1e-12 {
            return Vector3::zeros();
        }

        let range = los_vector.norm();
        let los_direction = los_vector / range;

        let los_rate_vector = calculate_los_rate(
            &missile.state.position,
            &missile.state.velocity,
            &target.state.position,
            &target.state.velocity,
        );

        let los_rate_magnitude = los_rate_vector.norm();
        if los_rate_magnitude < 1e-12 {
            return Vector3::zeros();
        }

        let closing_speed = calculate_closing_speed(
            &missile.state.position,
            &missile.state.velocity,
            &target.state.position,
            &target.state.velocity,
        );

        // Target is moving away and is unreachable at this point
        if closing_speed <= 0.0 {
            return Vector3::zeros();
        }

        let direction = los_rate_vector.cross(&los_direction);
        if direction.norm_squared() < 1e-12 {
            return Vector3::zeros();
        }

        // TODO; add few tweaks? alter factors?
        // - non-linear weighted closing speed
        // - range & tweaked los dependent
        // - cos theta alignment?
        // - varying N const on something, damping?

        let missile_speed = missile.state.velocity.norm();
        if missile_speed < 1e-6 {
            return Vector3::zeros();
        }

        // TODO: squared?
        //let vc_vm = closing_speed / missile_speed;

        // let direction = direction.normalize();
        let accel = direction * missile.navigation_constant * closing_speed;
        // TODO; mul los_rate_magnitude again?;

        if accel.norm() > missile.max_acceleration {
            accel * (missile.max_acceleration / accel.norm())
        } else {
            accel
        }
    }

    fn name(&self) -> &str {
        "TPN"
    }
}
