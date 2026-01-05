pub mod kinematics;
pub mod kinematics_simd;
pub mod state;

pub use kinematics::{calculate_closing_speed, calculate_los_rate};
pub use kinematics_simd::{
    calculate_closing_speed_simd, calculate_los_rate_simd, dot_simd, norm_simd, normalize_simd,
};
pub use state::State3D;
