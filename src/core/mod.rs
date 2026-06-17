mod kinematics;
mod state;
mod kinematics_simd;

pub use kinematics::{calculate_closing_speed, calculate_los_rate};
pub use state::State3D;
