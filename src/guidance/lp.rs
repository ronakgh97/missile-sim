use crate::core::calculate_closing_speed;
use crate::entity::{Missile, Target};
use crate::guidance::GuidanceLaw;
use nalgebra::Vector3;

/// Lead Pursuit (LP) guidance.
///
/// Predicts the target's future position based on its current velocity and
/// aims at that intercept point rather than the current position.
/// More efficient than pure pursuit for crossing targets.
///
/// a_c = lateral_unit * N * V_m * lateral_norm
/// Where:
/// - aim_dir = normalize(aim_point - M_pos)
/// - aim_point = T_pos + V_t * t_intercept
/// - t_intercept = r / V_closing
pub struct LeadPursuit {
    lead_time: f64,
}

impl LeadPursuit {
    /// Creates LP with the given lead time in seconds.
    pub fn new(lead_time: f64) -> Self {
        Self {
            lead_time: lead_time.max(0.0),
        }
    }

    /// Returns the lead time.
    pub fn lead_time(&self) -> f64 {
        self.lead_time
    }
}

impl Default for LeadPursuit {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl GuidanceLaw for LeadPursuit {
    #[inline]
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        let missile_speed = missile.state.speed();

        if missile_speed < 1e-6 {
            let range_vec = target.state.position - missile.state.position;
            let range = range_vec.norm();
            if range < 1e-6 {
                return Vector3::zeros();
            }
            return (range_vec / range) * missile.max_acceleration;
        }

        let range_vec = target.state.position - missile.state.position;
        let range = range_vec.norm();

        if range < 1e-6 {
            return Vector3::zeros();
        }

        // Closing speed (positive when approaching)
        let closing_speed = calculate_closing_speed(
            &missile.state.position,
            &missile.state.velocity,
            &target.state.position,
            &target.state.velocity,
        );

        // Time to intercept
        let t_intercept = if closing_speed > 1.0 {
            (range / closing_speed).clamp(0.1, self.lead_time * 2.0)
        } else {
            self.lead_time
        };

        // Predicted target position at intercept time
        let intercept_point = target.state.position + target.state.velocity * t_intercept;

        // Aim at predicted intercept point
        let aim_vec = intercept_point - missile.state.position;
        let aim_range = aim_vec.norm();

        if aim_range < 1e-6 {
            return Vector3::zeros();
        }

        let aim_unit = aim_vec / aim_range;
        let velocity_unit = missile.state.velocity / missile_speed;

        // Compute lateral (perpendicular) component needed to turn toward intercept
        let dot_product = velocity_unit.dot(&aim_unit);
        let lateral_component = aim_unit - velocity_unit * dot_product;

        let lateral_norm = lateral_component.norm();
        if lateral_norm < 1e-12 {
            return aim_unit * missile.max_acceleration;
        }

        let lateral_unit = lateral_component.normalize();

        // Acceleration magnitude based on required turn rate
        let accel_magnitude = missile.navigation_constant * missile_speed * lateral_norm;

        lateral_unit * accel_magnitude.min(missile.max_acceleration)
    }

    fn name(&self) -> &str {
        "LP"
    }
}
