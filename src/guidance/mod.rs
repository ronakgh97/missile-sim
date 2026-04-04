mod apn;
mod dp;
mod lp;
mod pp;
mod ppn;
mod tpn;
pub mod traits;

pub use apn::AugmentedProportionalNavigation;
pub use dp::DeviatedPursuit;
pub use lp::LeadPursuit;
pub use pp::PurePursuit;
pub use ppn::PureProportionalNavigation;
pub use tpn::TrueProportionalNavigation;
pub use traits::GuidanceLaw;

use crate::entity::{Missile, Target};
use nalgebra::Vector3;

/// Built-in guidance law types for convenient use.
///
/// This enum wraps all six guidance algorithms and implements [`GuidanceLaw`],
/// so it can be passed directly to [`Scenario::simulate`] or [`SimulationEngine::run`].
///
/// # Example
///
/// ```
/// use missile_sim::prelude::*;
///
/// let guidance = GuidanceLawType::ppn();
/// // or with parameters:
/// let apn = GuidanceLawType::apn(0.5);
/// let lp = GuidanceLawType::lp(1.0);
/// ```
#[derive(Debug, Clone)]
pub enum GuidanceLawType {
    /// Pure Proportional Navigation.
    PPN,
    /// True Proportional Navigation.
    TPN,
    /// Augmented Proportional Navigation with ZEM compensation.
    APN(AugmentedProportionalNavigation),
    /// Pure Pursuit.
    PP,
    /// Deviated Pursuit.
    DP,
    /// Lead Pursuit with intercept prediction.
    LP(LeadPursuit),
}

impl GuidanceLawType {
    /// Creates a Pure Proportional Navigation guidance law.
    pub fn ppn() -> Self {
        GuidanceLawType::PPN
    }

    /// Creates a True Proportional Navigation guidance law.
    pub fn tpn() -> Self {
        GuidanceLawType::TPN
    }

    /// Creates an Augmented Proportional Navigation guidance law.
    ///
    /// # Arguments
    ///
    /// * `time_constant` — ZEM (ZERO EFFECT MISS) augmentation time constant (typically 0.1–1.0 s).
    pub fn apn(time_constant: f64) -> Self {
        GuidanceLawType::APN(AugmentedProportionalNavigation::new(time_constant))
    }

    /// Creates a Pure Pursuit guidance law.
    pub fn pp() -> Self {
        GuidanceLawType::PP
    }

    /// Creates a Deviated Pursuit guidance law.
    pub fn dp() -> Self {
        GuidanceLawType::DP
    }

    /// Creates a Lead Pursuit guidance law.
    ///
    /// # Arguments
    ///
    /// * `lead_time` — How far ahead to predict target position (seconds).
    pub fn lp(lead_time: f64) -> Self {
        GuidanceLawType::LP(LeadPursuit::new(lead_time))
    }

    /// Returns this guidance law as a trait object for use with custom algorithms.
    pub fn as_dyn(&self) -> &dyn GuidanceLaw {
        match self {
            GuidanceLawType::PPN => &PureProportionalNavigation,
            GuidanceLawType::TPN => &TrueProportionalNavigation,
            GuidanceLawType::APN(apn) => apn,
            GuidanceLawType::PP => &PurePursuit,
            GuidanceLawType::DP => &DeviatedPursuit,
            GuidanceLawType::LP(lp) => lp,
        }
    }

    /// Returns the name of this guidance law.
    pub fn name(&self) -> &str {
        self.as_dyn().name()
    }
}

impl GuidanceLaw for GuidanceLawType {
    #[inline]
    fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        self.as_dyn().calculate_acceleration(missile, target)
    }

    fn name(&self) -> &str {
        self.as_dyn().name()
    }
}
