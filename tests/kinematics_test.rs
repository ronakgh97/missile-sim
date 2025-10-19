use missile_sim::prelude::{calculate_closing_speed, calculate_los_rate};
use nalgebra::Vector3;

#[test]
fn test_calculate_closing_speed_head_on() {
    let missile_pos = Vector3::new(0.0, 0.0, 0.0);
    let missile_vel = Vector3::new(100.0, 0.0, 0.0);
    let target_pos = Vector3::new(1000.0, 0.0, 0.0);
    let target_vel = Vector3::new(0.0, 0.0, 0.0);

    let closing_speed =
        calculate_closing_speed(&missile_pos, &missile_vel, &target_pos, &target_vel);
    assert!((closing_speed - 100.0).abs() < 1e-6);
}

#[test]
fn test_calculate_closing_speed_overtake() {
    let missile_pos = Vector3::new(0.0, 0.0, 0.0);
    let missile_vel = Vector3::new(100.0, 0.0, 0.0);
    let target_pos = Vector3::new(1000.0, 0.0, 0.0);
    let target_vel = Vector3::new(50.0, 0.0, 0.0);

    let closing_speed =
        calculate_closing_speed(&missile_pos, &missile_vel, &target_pos, &target_vel);
    assert!((closing_speed - 50.0).abs() < 1e-6);
}

#[test]
fn test_calculate_closing_speed_moving_apart() {
    let missile_pos = Vector3::new(0.0, 0.0, 0.0);
    let missile_vel = Vector3::new(100.0, 0.0, 0.0);
    let target_pos = Vector3::new(1000.0, 0.0, 0.0);
    let target_vel = Vector3::new(150.0, 0.0, 0.0);

    let closing_speed =
        calculate_closing_speed(&missile_pos, &missile_vel, &target_pos, &target_vel);
    assert!((closing_speed - (-50.0)).abs() < 1e-6);
}

#[test]
fn test_calculate_los_rate() {
    let missile_pos = Vector3::new(0.0, 0.0, 0.0);
    let missile_vel = Vector3::new(10.0, 0.0, 0.0);
    let target_pos = Vector3::new(100.0, 0.0, 0.0);
    let target_vel = Vector3::new(0.0, 10.0, 0.0);

    let los_rate = calculate_los_rate(&missile_pos, &missile_vel, &target_pos, &target_vel);

    let expected_los_rate = Vector3::new(0.0, 0.1, 0.0);

    assert!((los_rate - expected_los_rate).norm() < 1e-6);
}
