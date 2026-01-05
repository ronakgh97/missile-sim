use crate::entity::{MissileConfig, TargetConfig};
use crate::simulation::{Scenario, ScenarioBuilder};
use nalgebra::Vector3;

pub fn load_preset_scenarios() -> Vec<Scenario> {
    vec![test_0(), test_1(), test_2(), test_3()]
}

fn test_0() -> Scenario {
    ScenarioBuilder::new("Perpendicular-Intercept")
        .missile_config(MissileConfig {
            position: Vector3::new(500.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 1250.0, 0.0),
            max_acceleration: 1500.0,
            navigation_constant: 5.0,
            max_closing_speed: 8000.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(-5000.0, 2000.0, 0.0),
            velocity: Vector3::new(1200.0, 0.0, 0.0),
        })
        .dt(0.000001) // 1000 Hz update rate
        .total_time(60.0)
        .hit_threshold(10.0)
        .build()
}

fn test_1() -> Scenario {
    ScenarioBuilder::new("Ground-Launch-Strike")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(500.0, 1000.0, 800.0),
            max_acceleration: 2500.0,
            navigation_constant: 8.0,
            max_closing_speed: 8500.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(5000.0, 0.0, 0.0),
            velocity: Vector3::new(100.0, 0.0, 1000.0),
        })
        .dt(0.000001)
        .total_time(60.0)
        .hit_threshold(10.0)
        .build()
}

fn test_2() -> Scenario {
    ScenarioBuilder::new("Air-Strike")
        .missile_config(MissileConfig {
            position: Vector3::new(-5000.0, 5000.0, 0.0),
            velocity: Vector3::new(2555.0, -1250.0, 0.0),
            max_acceleration: 4500.0,
            navigation_constant: 6.0,
            max_closing_speed: 8550.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(5000.0, 100.0, 0.0),
            velocity: Vector3::new(750.0, 0.0, 0.0),
        })
        .dt(0.000001)
        .total_time(60.0)
        .hit_threshold(10.0)
        .build()
}

fn test_3() -> Scenario {
    ScenarioBuilder::new("Side-Intercept")
        .missile_config(MissileConfig {
            position: Vector3::new(500.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 1250.0, 0.0), // Steep dive
            max_acceleration: 5500.0,
            navigation_constant: 8.0,
            max_closing_speed: 8000.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(-5000.0, 2000.0, 5000.0),
            velocity: Vector3::new(1200.0, 0.0, 0.0),
        })
        .dt(0.000001)
        .total_time(60.0)
        .hit_threshold(10.0)
        .build()
}
