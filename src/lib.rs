//! # missile-sim — A high-performance 3D missile guidance simulation library.
//!
//! This library simulates missile-target engagement scenarios using six different
//! guidance laws. It is designed for use in games, simulations, and research experiments.
//!
//! ### Quick Start
//!
//! ```rust
//! use missile_sim::prelude::*;
//!
//! fn main() {
//!     let scenario = Scenario::builder("head-on")
//!         .missile(Missile {
//!             state: State3D {
//!                 position: Vector3::new(0.0, 0.0, 0.0),
//!                 velocity: Vector3::new(100.0, 0.0, 0.0),
//!             },
//!             max_acceleration: 30.0,
//!             navigation_constant: 4.0,
//!             max_closing_speed: 1000.0,
//!         })
//!         .target(Target {
//!             state: State3D {
//!                 position: Vector3::new(1000.0, 0.0, 0.0),
//!                 velocity: Vector3::new(0.0, 0.0, 0.0),
//!             },
//!             acceleration: Vector3::zeros(),
//!         })
//!         .dt(0.01) // steps
//!         .total_time(20.0)
//!         .hit_threshold(5.0) // proximity hit
//!         .build()
//!         .unwrap();
//!
//!     let metrics = scenario.simulate(&PureProportionalNavigation);
//!     println!("{}", metrics.console_summary());
//!
//! }
//! ```
//!
//! ### Guidance Laws
//!
//! | Law | Description |
//! |-----|-------------|
//! | **PPN** | Pure Proportional Navigation — acceleration perpendicular to missile velocity |
//! | **TPN** | True Proportional Navigation — uses closing speed instead of missile speed |
//! | **APN** | Augmented PN with Zero Effort Miss compensation for maneuvering targets |
//! | **PP**  | Pure Pursuit — steers directly toward current target position |
//! | **DP**  | Deviated Pursuit — adaptive pursuit with closing-speed awareness |
//! | **LP**  | Lead Pursuit — predicts intercept point and aims there |
//!
//! ### Custom Guidance Laws
//!
//! Implement the [`crate::guidance::GuidanceLaw`] trait and pass your type to [`crate::simulation::Scenario::simulate`] or
//! [`crate::simulation::SimulationEngine::run`]:
//!
//! ```no_run
//! use missile_sim::prelude::*;
//! use nalgebra::Vector3;
//!
//! struct MyGuidance;
//!
//! impl GuidanceLaw for MyGuidance {
//!     fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
//!         let range = target.state.position - missile.state.position;
//!         range.normalize() * missile.max_acceleration // acc based on range mag
//!     }
//!
//!     fn name(&self) -> &str {
//!         "MyGuidance"
//!     }
//! }
//!
//! ```
//!
//! ### Performance Comparison
//!
//! These runs showcase the performance & accuracy of the six guidance laws against a maneuvering target over 1000, 5000, and 10000 simulations.
//! #### 1000
//! ![1000-run](https://raw.githubusercontent.com/ronakgh97/missile-sim/refs/heads/master/assets/Summary_1000.png)
//!
//! #### 5000
//! ![5000-run](https://raw.githubusercontent.com/ronakgh97/missile-sim/refs/heads/master/assets/Summary_5000.png)
//!
//! #### 10000
//! ![10000-run](https://raw.githubusercontent.com/ronakgh97/missile-sim/refs/heads/master/assets/Summary_10000.png)

pub mod core;
pub mod entity;
pub mod guidance;
pub mod simulation;

/// Re-exports of the most commonly used types for convenient `use missile_sim::prelude::*;`.
pub mod prelude {
    pub use crate::core::{State3D, calculate_closing_speed, calculate_los_rate};
    pub use crate::entity::{Missile, Target};
    pub use crate::guidance::{
        AugmentedProportionalNavigation, GuidanceLaw, LeadPursuit, PureProportionalNavigation,
        PurePursuit, TrueProportionalNavigation,
    };
    pub use crate::simulation::{Scenario, ScenarioBuilder, SimulationEngine, SimulationMetrics};
    pub use nalgebra::*;
}
