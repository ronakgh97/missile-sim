pub mod export;
pub mod plotters;

// Direct function exports - no abstraction layers
pub use plotters::{render_metrics, render_trajectory_3d};
