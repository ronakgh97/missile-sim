use crate::core::calculate_closing_speed;
use crate::entity::{Missile, Target};
use crate::guidance::traits::GuidanceLaw;
use nalgebra::Vector3;

/// Deviated Pursuit (DP) - Pursuit with velocity matching and adaptive aggression
///
/// Improvements over Pure Pursuit:
/// - Short-term velocity vector alignment
/// - Range-dependent aggression (more aggressive at close range)
/// - Closing speed awareness for energy management
pub struct DeviatedPursuit;

impl GuidanceLaw for DeviatedPursuit {
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        let range_vec = target.state.position - missile.state.position;
        let range = range_vec.norm();

        if range < 1e-6 {
            return Vector3::zeros();
        }

        let missile_speed = missile.state.speed();

        if missile_speed < 1e-6 {
            // No speed, just accelerate toward target
            return (range_vec / range) * missile.max_acceleration;
        }

        let range_unit = range_vec / range;
        let velocity_unit = missile.state.velocity / missile_speed;

        // CLOSING SPEED AWARENESS
        let closing_speed = calculate_closing_speed(
            &missile.state.position,
            &missile.state.velocity,
            &target.state.position,
            &target.state.velocity,
        );

        // Adaptive aggression based on closing performance
        let closing_factor = if closing_speed > 0.0 {
            let relative_closing = closing_speed / missile_speed;
            if relative_closing < 0.2 {
                // Very slow closing - be more aggressive
                1.6
            } else if relative_closing < 0.5 {
                // Moderate closing - slightly more aggressive
                1.3
            } else {
                // Good closing rate - normal aggression
                1.0
            }
        } else {
            // Negative closing - maximum aggression
            2.0
        };

        // RANGE-BASED DAMPING (like LP, prevent oscillation)
        let range_damping = if range < 1000.0 {
            (range / 1000.0).clamp(0.3, 1.0)
        } else {
            1.0
        };

        // TURN RATE CALCULATION
        let lateral_component = range_unit - velocity_unit * velocity_unit.dot(&range_unit);

        if lateral_component.norm() < 1e-12 {
            // Already aligned, accelerate forward
            return range_unit * missile.max_acceleration;
        }

        let lateral_unit = lateral_component.normalize();

        // Apply closing-based aggression and range damping
        let accel_magnitude = missile.navigation_constant
            * missile_speed
            * lateral_component.norm()
            * closing_factor
            * range_damping;

        // Clamp to max acceleration
        lateral_unit * accel_magnitude.min(missile.max_acceleration)
    }

    fn name(&self) -> &str {
        "DP"
    }
}
