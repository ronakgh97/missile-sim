use crate::entity::{Missile, Target};
use crate::guidance::GuidanceLaw;
use nalgebra::Vector3;

/// Augmented Proportional Navigation (APN) with Zero Effort Miss (ZEM) compensation.
///
/// APN enhances PN by predicting the zero-effort miss (miss distance if no
/// further corrections are applied) and adding a corrective term:
/// a_APN = a_PN + ZEM * 2 / t_go²
/// Where:
/// - `a_PN = N * V_m * (λ̇ × V̂)` — standard PN acceleration
/// - `ZEM = R + Ṙ * t_go` — predicted miss at time-to-go
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
            time_constant: time_constant.max(0.01),
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

        // rel geometry
        let range_vec = target_pos - missile_pos;
        let range_sq = range_vec.norm_squared();

        if range_sq < 1e-12 {
            return Vector3::zeros();
        }

        let range = range_sq.sqrt();
        let rel_vel = target_vel - missile_vel;

        // los rate
        let los_rate = range_vec.cross(&rel_vel) / range_sq;

        // positive when closing
        let closing_speed = -(rel_vel.dot(&range_vec)) / range;

        // PN TERM
        let velocity_unit = missile_vel / missile_speed;

        let pn_accel =
            los_rate.cross(&velocity_unit) * (missile.navigation_constant * missile_speed);

        // ZEM TERM
        let zem_accel = if closing_speed > 1e-6 {
            let t_go = (range / closing_speed).clamp(0.1, 5.0);

            // Zero-Effort-Miss; future miss if no further acceleration occurs
            let zem = range_vec + rel_vel * t_go;

            // projected perpendicular to velocity (missiles can only produce lateral acceleration)
            let zem_raw = zem * (2.0 / (t_go * t_go));
            zem_raw - velocity_unit * velocity_unit.dot(&zem_raw)
        } else {
            Vector3::zeros()
        };

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
