use missile_sim::prelude::*;
use nalgebra::Vector3;

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
        acceleration: Vector3::zeros(),
    });

    let guidance = PureProportionalNavigation;
    let acceleration = guidance.calculate_acceleration(&missile, &target);

    assert!(acceleration.x.abs() < 1e-6);
    assert!(acceleration.z.abs() < 1e-6);
    assert!(acceleration.y > 0.0);

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
        acceleration: Vector3::zeros(),
    });

    let guidance = TrueProportionalNavigation;
    let acceleration = guidance.calculate_acceleration(&missile, &target);

    assert!(acceleration.x.abs() < 1e-6);
    assert!(acceleration.z.abs() < 1e-6);
    assert!(acceleration.y > 0.0);

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

#[test]
fn test_apn_guidance() {
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
        acceleration: Vector3::zeros(),
    });

    let guidance = AugmentedProportionalNavigation::new(0.5);
    let acceleration = guidance.calculate_acceleration(&missile, &target);

    assert!(acceleration.norm() > 0.0);
    assert!(acceleration.norm() <= missile.max_acceleration + 1e-6);
}

#[test]
fn test_pure_pursuit_guidance() {
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
        acceleration: Vector3::zeros(),
    });

    let guidance = PurePursuit;
    let acceleration = guidance.calculate_acceleration(&missile, &target);

    assert!(acceleration.norm() > 0.0);
    assert!(acceleration.norm() <= missile.max_acceleration);
}

#[test]
fn test_deviated_pursuit_guidance() {
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
        acceleration: Vector3::zeros(),
    });

    let guidance = DeviatedPursuit::default();
    let acceleration = guidance.calculate_acceleration(&missile, &target);

    assert!(acceleration.norm() > 0.0);
    assert!(acceleration.norm() <= missile.max_acceleration);
}

#[test]
fn test_lead_pursuit_guidance() {
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
        acceleration: Vector3::zeros(),
    });

    let guidance = LeadPursuit::new(1.0);
    let acceleration = guidance.calculate_acceleration(&missile, &target);

    assert!(acceleration.norm() > 0.0);
    assert!(acceleration.norm() <= missile.max_acceleration);
}
