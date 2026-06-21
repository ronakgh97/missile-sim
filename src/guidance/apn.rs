use crate::core::{calculate_closing_speed, calculate_los_rate};
use crate::entity::{Missile, Target};
use crate::guidance::GuidanceLaw;
use nalgebra::Vector3;

/// Augmented Proportional Navigation (APN) with Zero Effort Miss (ZEM) compensation.
///
/// APN enhances PN by predicting the zero-effort miss (miss distance if no
/// further corrections are applied) and adding a corrective term:
/// `a_APN = a_PN + N * ZEM / t_go^2`
/// Where:
/// - `a_PN = N * V_m * (λ̇ × V̂)` — standard PN acceleration
/// - `ZEM = R + (V_rel * t_go) + (0.5 * a_Target * t_go)` — predicted miss at time-to-go
/// - `t_go = range / V_closing` — estimated time to intercept
///
/// This approach implicitly accounts for target maneuvers through the ZEM
/// projection rather than explicitly measuring target acceleration.
#[derive(Clone, Debug)]
pub struct AugmentedProportionalNavigation {
    /// Time constant for ZEM augmentation (typically 0.1-1.0 seconds)
    time_constant: f64,
}

impl AugmentedProportionalNavigation {
    pub fn new(time_constant: f64) -> Self {
        Self {
            time_constant: time_constant.max(0.1),
        }
    }

    pub fn time_constant(&self) -> f64 {
        self.time_constant
    }
}

impl Default for AugmentedProportionalNavigation {
    fn default() -> Self {
        Self::new(0.5)
    }
}

impl GuidanceLaw for AugmentedProportionalNavigation {
    #[inline]
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        let missile_pos = missile.state.position;
        let missile_vel = missile.state.velocity;

        let target_pos = target.state.position;
        let target_vel = target.state.velocity;

        let missile_speed = missile_vel.norm();
        if missile_speed < 1e-6 {
            return Vector3::zeros();
        }

        let range_vec = target_pos - missile_pos;
        if range_vec.norm_squared() < 1e-12 {
            return Vector3::zeros();
        }

        let range = range_vec.norm();
        let rel_velocity = target_vel - missile_vel;

        let los_rate_vector =
            calculate_los_rate(&missile_pos, &missile_vel, &target_pos, &target_vel);

        let closing_speed =
            calculate_closing_speed(&missile_pos, &missile_vel, &target_pos, &target_vel);

        // PN TERM
        let velocity_unit = missile_vel / missile_speed;
        let pn_accel = los_rate_vector.cross(&missile_vel) * missile.navigation_constant;

        // ZEM TERM
        let zem_accel = if closing_speed > 1e-6 {
            let t_go = (range / closing_speed).max(0.1);

            let target_accel_effect = target.acceleration * (0.5 * t_go * t_go);

            // zem predicts the target's curve
            let zem = range_vec + (rel_velocity * t_go) + target_accel_effect;
            // TODO; use N here?
            let zem_raw = zem * missile.navigation_constant / (t_go * t_go);
            // projected perpendicular to velocity (i.e. missiles can only produce lateral acceleration)
            zem_raw - velocity_unit * velocity_unit.dot(&zem_raw)
        } else {
            Vector3::zeros()
        };

        // TODO: weighting factors for PN vs ZEM? based on range, closing speed or use T etc
        let total_accel = pn_accel + zem_accel;
        let accel_mag = total_accel.norm();

        if accel_mag > missile.max_acceleration {
            total_accel * (missile.max_acceleration / accel_mag)
        } else {
            total_accel
        }
    }

    fn name(&self) -> &str {
        "APN"
    }
}
