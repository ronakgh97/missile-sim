use crate::entity::{Missile, Target};
use crate::guidance::GuidanceLaw;
use nalgebra::Vector3;

/// Deviated Pursuit (DP) — blend of Pure Pursuit and Lead Pursuit.
///
/// Combination of PP with LP:
/// aim = α * PP_direction + (1 - α) * LP_direction
/// - α = 1.0 → Pure Pursuit
/// - α = 0.0 → Lead Pursuit
/// - α = 0.5 → Equal blend
pub struct DeviatedPursuit {
    /// Blend factor between PP and LP (0.0 = full LP, 1.0 = full PP).
    alpha: f64,
    /// Lead time in seconds for the LP component.
    lead_time: f64,
}

impl DeviatedPursuit {
    /// Creates a DP with the given blend factor and lead time.
    /// * `alpha` — Blend factor in [0, 1]. 1.0 = pure pursuit, 0.0 = lead pursuit.
    /// * `lead_time` — Lead time for LP component in seconds.
    pub fn new(alpha: f64, lead_time: f64) -> Self {
        Self {
            alpha: alpha.clamp(0.0, 1.0),
            lead_time: lead_time.max(0.0),
        }
    }
}

impl Default for DeviatedPursuit {
    fn default() -> Self {
        Self::new(0.5, 1.0)
    }
}

impl GuidanceLaw for DeviatedPursuit {
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

        let velocity_unit = missile.state.velocity / missile_speed;

        // Pure Pursuit direction
        let pp_dir = range_vec / range;

        // Lead Pursuit direction (simple linear intercept prediction)
        let closing_speed = crate::core::calculate_closing_speed(
            &missile.state.position,
            &missile.state.velocity,
            &target.state.position,
            &target.state.velocity,
        );

        let t_intercept = if closing_speed > 1.0 {
            (range / closing_speed).clamp(0.1, self.lead_time * 2.0)
        } else {
            self.lead_time
        };

        let intercept_point = target.state.position + target.state.velocity * t_intercept;
        let lp_vec = intercept_point - missile.state.position;
        let lp_range = lp_vec.norm();

        let lp_dir = if lp_range > 1e-6 {
            lp_vec / lp_range
        } else {
            pp_dir
        };

        // blend alpha * PP + (1-alpha) * LP
        let blended_dir = pp_dir * self.alpha + lp_dir * (1.0 - self.alpha);
        let blended_norm = blended_dir.norm();

        if blended_norm < 1e-12 {
            return pp_dir * missile.max_acceleration;
        }

        let aim_unit = blended_dir / blended_norm;

        // lateral component perpendicular to velocity
        let dot_product = velocity_unit.dot(&aim_unit);
        let lateral_component = aim_unit - velocity_unit * dot_product;

        let lateral_norm = lateral_component.norm();
        if lateral_norm < 1e-12 {
            return aim_unit * missile.max_acceleration;
        }

        let lateral_unit = lateral_component.normalize();

        let accel_magnitude = missile.navigation_constant * missile_speed * lateral_norm;

        lateral_unit * accel_magnitude.min(missile.max_acceleration)
    }

    fn name(&self) -> &str {
        "DP"
    }
}
