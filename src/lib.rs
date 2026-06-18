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
//! Implement the [`GuidanceLaw`] trait and pass your type to [`Scenario::simulate`] or
//! [`SimulationEngine::run`]:
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
//! ![1000-run](https://raw.githubusercontent.com/ronakgh97/missile-sim/refs/heads/master/assets/Summary_1000.png)
//! - PPN: 38.9% hit rate (389 hits)
//! - TPN: 31.0% hit rate (310 hits)
//! - APN: 51.9% hit rate (519 hits)
//! - PP: 73.8% hit rate (738 hits)
//! - DP: 67.0% hit rate (670 hits)
//! - LP: 85.6% hit rate (856 hits)
//!
//! ![5000-run](https://raw.githubusercontent.com/ronakgh97/missile-sim/refs/heads/master/assets/Summary_5000.png)
//! - PPN: 40.0% hit rate (2000 hits)
//! - TPN: 32.1% hit rate (1606 hits)
//! - APN: 52.9% hit rate (2648 hits)
//! - PP: 74.8% hit rate (3741 hits)
//! - DP: 68.2% hit rate (3411 hits)
//! - LP: 85.6% hit rate (4279 hits)
//!
//! ![10000-run](https://raw.githubusercontent.com/ronakgh97/missile-sim/refs/heads/master/assets/Summary_5000.png)
//! - PPN: 40.3% hit rate (4031 hits)
//! - TPN: 32.2% hit rate (3217 hits)
//! - APN: 54.1% hit rate (5412 hits)
//! - PP: 74.7% hit rate (7473 hits)
//! - DP: 68.4% hit rate (6835 hits)
//! - LP: 85.5% hit rate (8552 hits)

pub mod core;
pub mod entity;
pub mod guidance;
pub mod simulation;

/// Re-exports of the most commonly used types for convenient `use missile_sim::prelude::*;`.
pub mod prelude {
    pub use crate::core::{State3D, calculate_closing_speed, calculate_los_rate};
    pub use crate::entity::{Missile, Target};
    pub use crate::guidance::{
        AugmentedProportionalNavigation, DeviatedPursuit, GuidanceLaw, LeadPursuit,
        PureProportionalNavigation, PurePursuit, TrueProportionalNavigation,
    };
    pub use crate::simulation::{Scenario, ScenarioBuilder, SimulationEngine, SimulationMetrics};
    pub use nalgebra::*;
}
