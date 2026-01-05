use nalgebra::Vector3;
use wide::f64x4;

/// SIMD-optimized calculation of line-of-sight (LOS) rate vector
#[inline]
pub fn calculate_los_rate_simd(
    missile_pos: &Vector3<f64>,
    missile_vel: &Vector3<f64>,
    target_pos: &Vector3<f64>,
    target_vel: &Vector3<f64>,
) -> Vector3<f64> {
    // Pack positions into SIMD registers (x, y, z, 0)
    let m_pos = f64x4::new([missile_pos.x, missile_pos.y, missile_pos.z, 0.0]);
    let t_pos = f64x4::new([target_pos.x, target_pos.y, target_pos.z, 0.0]);

    // Calculate range vector: target - missile
    let range_vec = t_pos - m_pos;
    let range_arr = range_vec.to_array();

    // Calculate range magnitude (norm)
    let range_sq =
        range_arr[0] * range_arr[0] + range_arr[1] * range_arr[1] + range_arr[2] * range_arr[2];
    let range = range_sq.sqrt();

    if range < 1e-6 {
        return Vector3::zeros();
    }

    // Calculate range unit vector
    let range_inv = 1.0 / range;
    let range_unit = range_vec * f64x4::splat(range_inv);

    // Pack velocities
    let m_vel = f64x4::new([missile_vel.x, missile_vel.y, missile_vel.z, 0.0]);
    let t_vel = f64x4::new([target_vel.x, target_vel.y, target_vel.z, 0.0]);

    // Relative velocity: target - missile
    let rel_vel = t_vel - m_vel;
    let rel_vel_arr = rel_vel.to_array();
    let range_unit_arr = range_unit.to_array();

    // Radial velocity (dot product)
    let radial_vel = rel_vel_arr[0] * range_unit_arr[0]
        + rel_vel_arr[1] * range_unit_arr[1]
        + rel_vel_arr[2] * range_unit_arr[2];

    // Tangential velocity
    let radial_component = range_unit * f64x4::splat(radial_vel);
    let tangential_vel = rel_vel - radial_component;

    // LOS rate = tangential velocity / range
    let range_inv_simd = f64x4::splat(range_inv);
    let los_rate = tangential_vel * range_inv_simd;
    let los_arr = los_rate.to_array();

    Vector3::new(los_arr[0], los_arr[1], los_arr[2])
}

/// SIMD-optimized closing speed calculation
#[inline]
pub fn calculate_closing_speed_simd(
    missile_pos: &Vector3<f64>,
    missile_vel: &Vector3<f64>,
    target_pos: &Vector3<f64>,
    target_vel: &Vector3<f64>,
) -> f64 {
    // Pack positions into SIMD
    let m_pos = f64x4::new([missile_pos.x, missile_pos.y, missile_pos.z, 0.0]);
    let t_pos = f64x4::new([target_pos.x, target_pos.y, target_pos.z, 0.0]);

    let range_vec = t_pos - m_pos;
    let range_arr = range_vec.to_array();

    let range_sq =
        range_arr[0] * range_arr[0] + range_arr[1] * range_arr[1] + range_arr[2] * range_arr[2];
    let range = range_sq.sqrt();

    if range < 1e-6 {
        return 0.0;
    }

    let range_unit = range_vec * f64x4::splat(1.0 / range);
    let range_unit_arr = range_unit.to_array();

    // Relative velocity: missile - target
    let m_vel = f64x4::new([missile_vel.x, missile_vel.y, missile_vel.z, 0.0]);
    let t_vel = f64x4::new([target_vel.x, target_vel.y, target_vel.z, 0.0]);
    let rel_vel = m_vel - t_vel;
    let rel_vel_arr = rel_vel.to_array();

    // Dot product for closing speed
    rel_vel_arr[0] * range_unit_arr[0]
        + rel_vel_arr[1] * range_unit_arr[1]
        + rel_vel_arr[2] * range_unit_arr[2]
}

/// SIMD-optimized vector normalization
#[inline]
pub fn normalize_simd(v: &Vector3<f64>) -> Vector3<f64> {
    let v_simd = f64x4::new([v.x, v.y, v.z, 0.0]);
    let v_arr = v_simd.to_array();

    let norm_sq = v_arr[0] * v_arr[0] + v_arr[1] * v_arr[1] + v_arr[2] * v_arr[2];

    if norm_sq < 1e-12 {
        return Vector3::zeros();
    }

    let inv_norm = f64x4::splat(1.0 / norm_sq.sqrt());
    let normalized = v_simd * inv_norm;
    let result = normalized.to_array();

    Vector3::new(result[0], result[1], result[2])
}

/// SIMD-optimized dot product
#[inline]
pub fn dot_simd(a: &Vector3<f64>, b: &Vector3<f64>) -> f64 {
    let a_simd = f64x4::new([a.x, a.y, a.z, 0.0]);
    let b_simd = f64x4::new([b.x, b.y, b.z, 0.0]);

    let product = a_simd * b_simd;
    let arr = product.to_array();

    arr[0] + arr[1] + arr[2]
}

/// SIMD-optimized vector magnitude
#[inline]
pub fn norm_simd(v: &Vector3<f64>) -> f64 {
    let v_simd = f64x4::new([v.x, v.y, v.z, 0.0]);
    let arr = v_simd.to_array();

    let sq = arr[0] * arr[0] + arr[1] * arr[1] + arr[2] * arr[2];
    sq.sqrt()
}

/// SIMD-optimized vector subtraction and scaling
#[allow(dead_code)]
#[inline]
pub fn sub_and_scale_simd(a: &Vector3<f64>, b: &Vector3<f64>, scale: f64) -> Vector3<f64> {
    let a_simd = f64x4::new([a.x, a.y, a.z, 0.0]);
    let b_simd = f64x4::new([b.x, b.y, b.z, 0.0]);
    let scale_simd = f64x4::splat(scale);

    let result = (a_simd - b_simd) * scale_simd;
    let arr = result.to_array();

    Vector3::new(arr[0], arr[1], arr[2])
}
