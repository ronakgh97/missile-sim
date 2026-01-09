pub mod exports;
pub mod gui;
mod plotters;

// Direct function exports - no abstraction layers
pub use gui::rendering::render_master;
pub use plotters::{render_metrics, render_trajectory_3d};
