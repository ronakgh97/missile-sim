use nalgebra::Vector3;

/// Calculates the line-of-sight (LOS) rate vector.
/// 
/// The LOS rate is the angular velocity of the line connecting the missile to the target.
/// It is computed as the tangential component of relative velocity divided by range.
/// Returns a 3D vector representing the LOS rate. The magnitude is the angular rate (rad/s), and
/// zero if range is negligible.
#[inline(always)]
pub fn calculate_los_rate(
    missile_pos: &Vector3<f64>,
    missile_vel: &Vector3<f64>,
    target_pos: &Vector3<f64>,
    target_vel: &Vector3<f64>,
) -> Vector3<f64> {
    let range_vec = target_pos - missile_pos;
    let range = range_vec.norm_squared();

    if range < 1e-12 {
        return Vector3::zeros();
    }
    
    let range = range.sqrt();
    let relative_velocity = target_vel - missile_vel;

    let inv_range = 1.0 / range;
    let radial_velocity = relative_velocity.dot(&range_vec) * inv_range;
    let radial_component = range_vec * (radial_velocity * inv_range);
    let tangential_velocity = relative_velocity - radial_component;

    tangential_velocity * inv_range
}

/// Calculates the closing speed between missile and target.
///
/// Closing speed is BASICALLY the rate at which the distance between missile and target is decreasing.
/// Positive values mean the entities are approaching; negative means they are separating.
/// Returns the scalar closing speed, and zero if range is negligible.
#[inline(always)]
pub fn calculate_closing_speed(
    missile_pos: &Vector3<f64>,
    missile_vel: &Vector3<f64>,
    target_pos: &Vector3<f64>,
    target_vel: &Vector3<f64>,
) -> f64 {
    let range_vec = target_pos - missile_pos;
    let range_sq = range_vec.norm_squared();

    if range_sq < 1e-12 {
        return 0.0;
    }

    let inv_range = range_sq.sqrt().recip();
    let relative_velocity = missile_vel - target_vel;
    relative_velocity.dot(&range_vec) * inv_range
}
