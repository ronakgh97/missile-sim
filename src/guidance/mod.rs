mod apn;
mod dp;
mod lp;
mod pp;
pub mod ppn;
pub mod tpn;
pub mod traits;

pub use apn::AugmentedProportionalNavigation;
pub use dp::DeviatedPursuit;
pub use lp::LeadPursuit;
pub use pp::PurePursuit;
pub use ppn::PureProportionalNavigation;
pub use tpn::TrueProportionalNavigation;

use crate::entity::{Missile, Target};
use nalgebra::Vector3;
use traits::GuidanceLaw;

/// Enum for static dispatch of guidance laws
#[derive(Debug, Clone)]
pub enum GuidanceLawType {
    PPN,
    TPN,
    APN(f64), // time_constant
    PP,
    DP,
    LP(f64), // lead_factor
}

impl GuidanceLawType {
    #[inline]
    pub fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
        match self {
            GuidanceLawType::PPN => {
                PureProportionalNavigation.calculate_acceleration(missile, target)
            }
            GuidanceLawType::TPN => {
                TrueProportionalNavigation.calculate_acceleration(missile, target)
            }
            GuidanceLawType::APN(time_constant) => {
                AugmentedProportionalNavigation::new(*time_constant)
                    .calculate_acceleration(missile, target)
            }
            GuidanceLawType::PP => PurePursuit.calculate_acceleration(missile, target),
            GuidanceLawType::DP => DeviatedPursuit.calculate_acceleration(missile, target),
            GuidanceLawType::LP(lead_factor) => {
                LeadPursuit::new(*lead_factor).calculate_acceleration(missile, target)
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            GuidanceLawType::PPN => "PPN",
            GuidanceLawType::TPN => "TPN",
            GuidanceLawType::APN(_) => "APN",
            GuidanceLawType::PP => "PP",
            GuidanceLawType::DP => "DP",
            GuidanceLawType::LP(_) => "LP",
        }
    }
}
