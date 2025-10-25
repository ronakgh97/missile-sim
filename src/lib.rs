mod core;
mod entity;
pub mod game;
mod guidance;
mod scenarios;
mod simulation;
mod visualization;

pub mod game_prelude {
    pub use crate::game::bundles::players::*;
    pub use crate::game::components::aircraft::*;
    pub use crate::game::components::controller::*;
    pub use crate::game::components::health::*;
    pub use crate::game::components::input::*;
    pub use crate::game::components::marker::*;
    pub use crate::game::components::missile::*;
    pub use crate::game::components::movement::*;
    pub use crate::game::components::player::*;
}

pub mod prelude {
    pub use crate::core::{State3D, calculate_closing_speed, calculate_los_rate};
    pub use crate::entity::{Missile, MissileConfig, Target, TargetConfig};
    pub use crate::guidance::{
        AugmentedProportionalNavigation, DeviatedPursuit, GuidanceLaw, LeadPursuit,
        PureProportionalNavigation, PurePursuit, TrueProportionalNavigation,
    };
    pub use crate::scenarios::load_preset_scenarios;
    pub use crate::scenarios::load_train_data;
    pub use crate::simulation::{Scenario, ScenarioBuilder, SimulationEngine, SimulationMetrics};
    pub use crate::visualization::{PlottersRenderer, RenderConfig, Renderer};
}
