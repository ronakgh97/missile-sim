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
    });

    let mut engine = SimulationEngine::new(missile, target, 0.01, 20.0, 1.0);
    let guidance = GuidanceLawType::PPN;
    let metrics = engine.run(&guidance);

    assert!(metrics.hit);
    assert!(metrics.miss_distance <= 1.0);
    assert!(*metrics.time_history.last().unwrap() < 10.1); // Should be around 10s
}
