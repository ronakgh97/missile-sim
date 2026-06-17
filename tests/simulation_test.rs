use missile_sim::prelude::*;
use nalgebra::Vector3;

#[test]
fn test_simulation_engine_hit() {
    let missile = Missile::new(MissileConfig {
        position: Vector3::new(0.0, 0.0, 0.0),
        velocity: Vector3::new(100.0, 0.0, 0.0),
        max_acceleration: 30.0,
        navigation_constant: 3.0,
        max_closing_speed: 1000.0,
    });

    let target = Target::new(TargetConfig {
        position: Vector3::new(1000.0, 0.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 0.0),
        acceleration: Vector3::zeros(),
    });

    let mut engine = SimulationEngine::new(missile, target, 0.01, 20.0, 1.0);
    let guidance = PureProportionalNavigation;
    let metrics = engine.run(&guidance);

    assert!(metrics.hit);
    assert!(metrics.miss_distance <= 1.0);
    assert!(*metrics.time_history.last().unwrap() < 10.1);
}

#[test]
fn test_scenario_simulate() {
    let scenario = Scenario::builder("head-on")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(100.0, 0.0, 0.0),
            max_acceleration: 30.0,
            navigation_constant: 3.0,
            max_closing_speed: 1000.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(1000.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            acceleration: Vector3::zeros(),
        })
        .dt(0.01)
        .total_time(20.0)
        .hit_threshold(1.0)
        .build()
        .unwrap();

    let metrics = scenario.simulate(&PureProportionalNavigation);

    assert!(metrics.hit);
    assert!(metrics.miss_distance <= 1.0);
}

#[test]
fn test_maneuvering_target() {
    let missile = Missile::new(MissileConfig {
        position: Vector3::new(0.0, 0.0, 0.0),
        velocity: Vector3::new(300.0, 0.0, 0.0),
        max_acceleration: 500.0,
        navigation_constant: 4.0,
        max_closing_speed: 2000.0,
    });

    let target = Target::new(TargetConfig {
        position: Vector3::new(5000.0, 0.0, 0.0),
        velocity: Vector3::new(0.0, 50.0, 0.0),
        acceleration: Vector3::new(0.0, 5.0, 0.0),
    });

    let mut engine = SimulationEngine::new(missile, target, 0.001, 30.0, 10.0);
    let guidance = AugmentedProportionalNavigation::new(0.5);
    let metrics = engine.run(&guidance);

    assert!(!metrics.time_history.is_empty());
    assert!(metrics.hit);
}
