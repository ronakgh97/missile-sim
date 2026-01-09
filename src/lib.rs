pub mod args;
mod core;
mod entity;
mod guidance;
mod scenarios;
mod simulation;
mod visual;

pub mod prelude {
    pub use crate::core::{State3D, calculate_closing_speed, calculate_los_rate};
    pub use crate::entity::{Missile, MissileConfig, Target, TargetConfig};
    pub use crate::guidance::{
        AugmentedProportionalNavigation, DeviatedPursuit, GuidanceLawType, LeadPursuit,
        PureProportionalNavigation, PurePursuit, TrueProportionalNavigation,
    };
    pub use crate::scenarios::RandomData;
    pub use crate::scenarios::load_preset_scenarios;
    pub use crate::simulation::{Scenario, ScenarioBuilder, SimulationEngine, SimulationMetrics};
    pub use crate::visual::{render_metrics, render_trajectory_3d, render_master};
}
