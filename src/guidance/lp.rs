use crate::entity::{Missile, Target};
use crate::guidance::traits::GuidanceLaw;
use nalgebra::Vector3;

/// Lead Pursuit (LP) - Predictive guidance
pub struct LeadPursuit {
    // How far ahead to predict target position (seconds)
    pub lead_time: f64,
}

impl LeadPursuit {
    // Create with explicit lead time
    pub fn new(lead_time: f64) -> Self {
        Self {
            lead_time: lead_time.max(0.0),
        }
    }

    // Create with default 1 second lead
    pub fn default() -> Self {
        Self::new(1.0)
    }
}

impl GuidanceLaw for LeadPursuit {
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        // Predict target's future position
        let predicted_target_pos = target.state.position + target.state.velocity * self.lead_time;

        // Vector from missile to predicted position
        let range_vec = predicted_target_pos - missile.state.position;
        let range = range_vec.norm();

        // Damping factor to reduce aggressiveness at close range
        let damping = if range < 1000.0 {
            (range / 1000.0).clamp(0.1, 1.0)
        } else {
            1.0
        };

        if range < 1e-6 {
            return Vector3::zeros();
        }

        // Desired direction (toward predicted position)
        let range_unit = range_vec / range;

        // Current missile state
        let missile_speed = missile.state.speed();

        if missile_speed < 1e-6 {
            // just accelerate toward predicted position
            return range_unit * missile.max_acceleration;
        }

        let velocity_unit = missile.state.velocity / missile_speed;

        // Compute lateral (perpendicular) component needed to turn
        let lateral_component = range_unit - velocity_unit * velocity_unit.dot(&range_unit);

        if lateral_component.norm() < 1e-12 {
            // Already pointing at predicted position
            return range_unit * missile.max_acceleration;
        }

        let lateral_unit = lateral_component.normalize();

        // Acceleration magnitude based on required turn rate
        // Use navigation constant as turn gain
        let accel_magnitude =
            missile.navigation_constant * missile_speed * lateral_component.norm() * damping;

        // Clamp to max acceleration
        lateral_unit * accel_magnitude.min(missile.max_acceleration)
    }

    fn name(&self) -> &str {
        "LP"
    }
}
