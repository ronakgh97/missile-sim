use nalgebra::Vector3;

/// Calculate line-of-sight (LOS) rate vector
pub fn calculate_los_rate(
    missile_pos: &Vector3<f64>,
    missile_vel: &Vector3<f64>,
    target_pos: &Vector3<f64>,
    target_vel: &Vector3<f64>,
) -> Vector3<f64> {
    let range_vec = target_pos - missile_pos;
    let range = range_vec.norm();

    if range < 1e-6 {
        // Too Close
        return Vector3::zeros();
    }

    let range_unit = range_vec / range;
    let relative_velocity = target_vel - missile_vel;

    let radial_velocity = relative_velocity.dot(&range_unit);
    let tangential_velocity = relative_velocity - range_unit * radial_velocity;

    tangential_velocity / range
}

/// Calculate closing speed (should negative when approaching)
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
