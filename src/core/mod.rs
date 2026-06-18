mod kinematics;
mod kinematics_simd;
mod state;

pub use kinematics::{calculate_closing_speed, calculate_los_rate};
pub use state::State3D;
