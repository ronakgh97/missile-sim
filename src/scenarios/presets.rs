use crate::entity::{MissileConfig, TargetConfig};
use crate::simulation::{Scenario, ScenarioBuilder};
use nalgebra::Vector3;

pub fn load_preset_scenarios() -> Vec<Scenario> {
    vec![tail_chase()]
}

fn head_on() -> Scenario {
    ScenarioBuilder::new("headon")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 400.0, 0.0),
            max_acceleration: 1200.0,
            navigation_constant: 4.0,
            max_closing_speed: 800.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(0.0, 2000.0, 0.0),
            velocity: Vector3::new(0.0, -300.0, 0.0),
        })
        .dt(0.01)
        .total_time(30.0)
        .hit_threshold(5.0)
        .build()
}

fn tail_chase() -> Scenario {
    ScenarioBuilder::new("tailchase")
        .missile_config(MissileConfig {
            position: Vector3::new(500.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 700.0, 0.0),
            max_acceleration: 1200.0,
            navigation_constant: 4.0,
            max_closing_speed: 800.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(-5000.0, 2000.0, 0.0),
            velocity: Vector3::new(500.0, 0.0, 0.0),
        })
        .dt(0.01)
        .total_time(30.0)
        .hit_threshold(5.0)
        .build()
}

fn crossing() -> Scenario {
    ScenarioBuilder::new("crossing")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(400.0, 0.0, 0.0),
            max_acceleration: 1200.0,
            navigation_constant: 4.0,
            max_closing_speed: 800.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(2000.0, 0.0, 0.0),
            velocity: Vector3::new(-300.0, 0.0, 0.0),
        })
        .dt(0.01)
        .total_time(30.0)
        .hit_threshold(5.0)
        .build()
}

fn offset_intercept() -> Scenario {
    ScenarioBuilder::new("offset")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(200.0, 400.0, 0.0),
            max_acceleration: 1200.0,
            navigation_constant: 4.0,
            max_closing_speed: 800.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(1000.0, 1000.0, 0.0),
            velocity: Vector3::new(0.0, 300.0, 0.0),
        })
        .dt(0.01)
        .total_time(30.0)
        .hit_threshold(5.0)
        .build()
}

fn fast_missile() -> Scenario {
    ScenarioBuilder::new("fastmissile")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 800.0, 0.0),
            max_acceleration: 2500.0,
            navigation_constant: 4.0,
            max_closing_speed: 1600.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(0.0, 2000.0, 0.0),
            velocity: Vector3::new(0.0, 300.0, 0.0),
        })
        .dt(0.01)
        .total_time(30.0)
        .hit_threshold(5.0)
        .build()
}
