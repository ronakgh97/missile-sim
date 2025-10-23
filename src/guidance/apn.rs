use crate::core::{calculate_closing_speed, calculate_los_rate};
use crate::entity::{Missile, Target};
use crate::guidance::traits::GuidanceLaw;
use nalgebra::Vector3;

/// Augmented Proportional Navigation (APN) with Zero Effort Miss (ZEM) compensation
///
/// APN enhances PN by anticipating target maneuvers using ZEM:
/// a_APN = a_PN + (1/τ) * ZEM
///
/// Where:
/// - a_PN is standard PN acceleration
/// - τ is time constant (derived from navigation gain)
/// - ZEM is zero effort miss vector (predicted miss if no corrections)
pub struct AugmentedProportionalNavigation {
    /// Time constant for ZEM augmentation (typically 0.1-1.0 seconds)
    pub time_constant: f64,
}

impl AugmentedProportionalNavigation {
    pub fn new(time_constant: f64) -> Self {
        Self {
            time_constant: time_constant.max(0.01), // Minimum 10ms
        }
    }

    pub fn default() -> Self {
        Self::new(0.5)
    }
}

impl GuidanceLaw for AugmentedProportionalNavigation {
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        let missile_speed = missile.state.speed();

        if missile_speed < 1e-6 {
            // No speed, just accelerate toward target
            let range_vec = target.state.position - missile.state.position;
            let range = range_vec.norm();
            if range < 1e-6 {
                return Vector3::zeros();
            }
            return (range_vec / range) * missile.max_acceleration;
        }

        // STANDARD PN COMPONENT
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

        let velocity_unit = missile.state.velocity.normalize();
        let los_rate_unit = los_rate_vector.normalize();

        // PN acceleration direction (perpendicular to velocity)
        let pn_accel_direction = los_rate_unit - velocity_unit * velocity_unit.dot(&los_rate_unit);
        let pn_accel_direction = pn_accel_direction.normalize();

        // PN acceleration magnitude (using missile speed)
        let pn_accel_magnitude = missile.navigation_constant * missile_speed * los_rate_magnitude;

        // --- ZERO EFFORT MISS (ZEM) COMPONENT ---
        let closing_speed = calculate_closing_speed(
            &missile.state.position,
            &missile.state.velocity,
            &target.state.position,
            &target.state.velocity,
        );

        // Calculate ZEM vector with better handling
        let zem_accel = if closing_speed > 1e-6 {
            let range = (target.state.position - missile.state.position).norm();
            let time_to_go = range / closing_speed;

            // Clamp time-to-go to reasonable bounds
            let clamped_tgo = time_to_go.clamp(0.1, 5.0);

            // Predict positions at intercept time
            let missile_final_pos = missile.state.position + missile.state.velocity * clamped_tgo;
            let target_final_pos = target.state.position + target.state.velocity * clamped_tgo;

            // ZEM is the vector from missile to target at intercept time
            let zem_vector = target_final_pos - missile_final_pos;

            // ZEM acceleration component: (1/τ) * ZEM, but scale by navigation constant
            zem_vector / self.time_constant * (missile.navigation_constant * 0.5)
        } else {
            // No closing speed, minimal ZEM contribution
            Vector3::zeros()
        };

        // COMBINED APN ACCELERATION
        let pn_accel = pn_accel_direction * pn_accel_magnitude;
        let total_accel = pn_accel + zem_accel;

        // Clamp to max acceleration while preserving direction
        let total_magnitude = total_accel.norm();
        if total_magnitude > missile.max_acceleration {
            total_accel * (missile.max_acceleration / total_magnitude)
        } else {
            total_accel
        }
    }

    fn name(&self) -> &str {
        "APN"
    }
}
