use nalgebra::Vector3;

/// Calculates the line-of-sight (LOS) rate vector.
///
/// The LOS rate is the angular velocity of the line connecting the missile to the target.
/// It is computed as the tangential component of relative velocity divided by range.
///
/// # Arguments
///
/// * `missile_pos` — Missile position.
/// * `missile_vel` — Missile velocity.
/// * `target_pos` — Target position.
/// * `target_vel` — Target velocity.
///
/// # Returns
///
/// A 3D vector representing the LOS rate. The magnitude is the angular rate (rad/s).
/// Returns zero if range is negligible.
pub fn calculate_los_rate(
    missile_pos: &Vector3<f64>,
    missile_vel: &Vector3<f64>,
    target_pos: &Vector3<f64>,
    target_vel: &Vector3<f64>,
) -> Vector3<f64> {
    let range_vec = target_pos - missile_pos;
    let range = range_vec.norm();

    if range < 1e-6 {
        return Vector3::zeros();
    }

    let range_unit = range_vec / range;
    let relative_velocity = target_vel - missile_vel;

    let radial_velocity = relative_velocity.dot(&range_unit);
    let tangential_velocity = relative_velocity - range_unit * radial_velocity;

    tangential_velocity / range
}

/// Calculates the closing speed between missile and target.
///
/// Closing speed is the rate at which the distance between missile and target is decreasing.
/// Positive values mean the entities are approaching; negative means they are separating.
///
/// # Arguments
///
/// * `missile_pos` — Missile position.
/// * `missile_vel` — Missile velocity.
/// * `target_pos` — Target position.
/// * `target_vel` — Target velocity.
///
/// # Returns
///
/// The scalar closing speed. Returns zero if range is negligible.
pub fn calculate_closing_speed(
    missile_pos: &Vector3<f64>,
    missile_vel: &Vector3<f64>,
    target_pos: &Vector3<f64>,
    target_vel: &Vector3<f64>,
) -> f64 {
    let range_vec = target_pos - missile_pos;
    let range = range_vec.norm();

    if range < 1e-6 {
        return 0.0;
    }

    let range_unit = range_vec / range;
    let relative_velocity = missile_vel - target_vel;

    relative_velocity.dot(&range_unit)
}
