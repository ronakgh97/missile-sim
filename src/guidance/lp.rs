use crate::core::{dot_simd, norm_simd, normalize_simd};
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
        let missile_speed = missile.state.speed();

        if missile_speed < 1e-6 {
            // No speed, just accelerate toward current target position
            let range_vec = target.state.position - missile.state.position;
            let range = norm_simd(&range_vec);
            if range < 1e-6 {
                return Vector3::zeros();
            }
            return (range_vec / range) * missile.max_acceleration;
        }

        // Calculate intercept point using vector triangle method
        let range_vec = target.state.position - missile.state.position;
        let range = norm_simd(&range_vec);

        if range < 1e-6 {
            return Vector3::zeros();
        }

        // Solve quadratic equation for time-to-intercept
        let vt = &target.state.velocity;
        let vt_sq = vt.norm_squared();
        let vm_sq = missile_speed * missile_speed;

        let a = vt_sq - vm_sq;
        let b = 2.0 * dot_simd(&range_vec, vt);
        let c = range * range;

        let mut intercept_point = target.state.position + vt * self.lead_time; // fallback

        // Solve quadratic equation
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 && a.abs() > 1e-6 {
            let sqrt_disc = discriminant.sqrt();
            let t1 = (-b + sqrt_disc) / (2.0 * a);
            let t2 = (-b - sqrt_disc) / (2.0 * a);

            // Choose positive, smaller time
            let t_intercept = if t1 > 0.0 && t2 > 0.0 {
                t1.min(t2)
            } else if t1 > 0.0 {
                t1
            } else if t2 > 0.0 {
                t2
            } else {
                self.lead_time // no valid solution, use fallback
            };

            // Clamp to reasonable time horizon
            let t_clamped = t_intercept.min(self.lead_time * 2.0).max(0.1);
            intercept_point = target.state.position + vt * t_clamped;
        } else if a.abs() <= 1e-6 && b.abs() > 1e-6 {
            // Linear case (missile and target speeds are equal)
            let t_intercept = -c / b;
            if t_intercept > 0.0 {
                let t_clamped = t_intercept.min(self.lead_time * 2.0);
                intercept_point = target.state.position + vt * t_clamped;
            }
        }

        // PURSUIT LOGIC TO INTERCEPT POINT
        let aim_vec = intercept_point - missile.state.position;
        let aim_range = norm_simd(&aim_vec);

        // Damping factor to reduce aggressiveness at close range
        let damping = if aim_range < 1000.0 {
            (aim_range / 1000.0).clamp(0.1, 1.0)
        } else {
            1.0
        };

        if aim_range < 1e-6 {
            return Vector3::zeros();
        }

        let aim_unit = aim_vec / aim_range;
        let velocity_unit = missile.state.velocity / missile_speed;

        // Compute lateral (perpendicular) component needed to turn toward intercept
        let dot_product = dot_simd(&velocity_unit, &aim_unit);
        let lateral_component = aim_unit - velocity_unit * dot_product;

        let lateral_norm = norm_simd(&lateral_component);
        if lateral_norm < 1e-12 {
            // Already aligned with intercept point
            return aim_unit * missile.max_acceleration;
        }

        let lateral_unit = normalize_simd(&lateral_component);

        // Acceleration magnitude based on required turn rate
        let accel_magnitude = missile.navigation_constant * missile_speed * lateral_norm * damping;

        // Clamp to max acceleration
        lateral_unit * accel_magnitude.min(missile.max_acceleration)
    }

    fn name(&self) -> &str {
        "LP"
    }
}
