pub mod charts;
pub mod trajectory_3d;

// Direct function exports - no wrapper struct needed
pub use charts::render_metrics;
pub use trajectory_3d::render_trajectory_3d;
