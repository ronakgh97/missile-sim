pub mod exports;
mod gui;
mod plotters;

// Direct function exports - no abstraction layers
pub use plotters::{render_metrics, render_trajectory_3d};
pub use gui::rendering::render_master;
