use nalgebra::Vector3;
use missile_sim::prelude::*;

#[test]
fn test_ppn_guidance() {
    let missile = Missile::new(MissileConfig {
        position: Vector3::new(0.0, 0.0, 0.0),
        velocity: Vector3::new(100.0, 0.0, 0.0),
        max_acceleration: 30.0,
        navigation_constant: 3.0,
        max_closing_speed: 1000.0,
    });

    let target = Target::new(TargetConfig {
        position: Vector3::new(1000.0, 1000.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 0.0),
    });

    let ppn = PureProportionalNavigation;
    let acceleration = ppn.calculate_acceleration(&missile, &target);

    // Direction
    assert!(acceleration.x.abs() < 1e-6);
    assert!(acceleration.z.abs() < 1e-6);
    assert!(acceleration.y > 0.0);

    // Magnitude
    let los_rate = calculate_los_rate(
        &missile.state.position,
        &missile.state.velocity,
        &target.state.position,
        &target.state.velocity,
    );
    let expected_mag = missile.navigation_constant * missile.state.speed() * los_rate.norm();
    let bounded_mag = expected_mag.min(missile.max_acceleration);
    
    assert!((acceleration.norm() - bounded_mag).abs() < 1e-6);
}

#[test]
fn test_tpn_guidance() {
    let missile = Missile::new(MissileConfig {
        position: Vector3::new(0.0, 0.0, 0.0),
        velocity: Vector3::new(100.0, 0.0, 0.0),
        max_acceleration: 30.0,
        navigation_constant: 3.0,
        max_closing_speed: 1000.0,
    });

    let target = Target::new(TargetConfig {
        position: Vector3::new(1000.0, 1000.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 0.0),
    });

    let tpn = TrueProportionalNavigation;
    let acceleration = tpn.calculate_acceleration(&missile, &target);

    // Direction should be the same as PPN
    assert!(acceleration.x.abs() < 1e-6);
    assert!(acceleration.z.abs() < 1e-6);
    assert!(acceleration.y > 0.0);

    // Magnitude
    let los_rate = calculate_los_rate(
        &missile.state.position,
        &missile.state.velocity,
        &target.state.position,
        &target.state.velocity,
    );
    let closing_speed = calculate_closing_speed(
        &missile.state.position,
        &missile.state.velocity,
        &target.state.position,
        &target.state.velocity,
    );
    let safe_closing_speed = closing_speed.abs().clamp(1.0, missile.max_closing_speed);

    let expected_mag = missile.navigation_constant * safe_closing_speed * los_rate.norm();
    let bounded_mag = expected_mag.min(missile.max_acceleration);
    
    assert!((acceleration.norm() - bounded_mag).abs() < 1e-6);
}
