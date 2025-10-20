use crate::entity::{MissileConfig, TargetConfig};
use crate::simulation::{Scenario, ScenarioBuilder};
use nalgebra::Vector3;

pub fn load_preset_scenarios() -> Vec<Scenario> {
    vec![
        test_0(),
        test_1(),
        test_2(),
        test_3(),
        test_4(),
        test_5(),
        test_6(),
        test_7(),
    ]
}

fn test_0() -> Scenario {
    ScenarioBuilder::new("Perpendicular-Intercept")
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
        .dt(0.0001) // 1000 Hz update rate
        .total_time(30.0)
        .hit_threshold(5.0)
        .build()
}

fn test_1() -> Scenario {
    ScenarioBuilder::new("VTOL-Urban-Strike")
        .missile_config(MissileConfig {
            position: Vector3::new(-800.0, 50.0, 0.0),
            velocity: Vector3::new(500.0, 200.0, 0.0),
            max_acceleration: 1800.0,
            navigation_constant: 5.0,
            max_closing_speed: 1000.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(1500.0, 300.0, 0.0),
            velocity: Vector3::new(-350.0, -50.0, 100.0),
        })
        .dt(0.0001)
        .total_time(15.0)
        .hit_threshold(5.0)
        .build()
}

fn test_2() -> Scenario {
    ScenarioBuilder::new("Jet-Head-On-Intercept")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 1000.0),
            velocity: Vector3::new(0.0, 600.0, -100.0),
            max_acceleration: 2200.0,
            navigation_constant: 5.0,
            max_closing_speed: 1400.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(0.0, 3500.0, 1200.0),
            velocity: Vector3::new(0.0, -700.0, -50.0), // Supersonic approach
        })
        .dt(0.0001)
        .total_time(10.0)
        .hit_threshold(5.0)
        .build()
}

fn test_3() -> Scenario {
    ScenarioBuilder::new("Ground-Attack-Intercept")
        .missile_config(MissileConfig {
            position: Vector3::new(-2000.0, 0.0, 500.0),
            velocity: Vector3::new(400.0, 100.0, -50.0),
            max_acceleration: 1600.0,
            navigation_constant: 4.5,
            max_closing_speed: 950.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(2500.0, 1500.0, 800.0),
            velocity: Vector3::new(-500.0, -200.0, -100.0),
        })
        .dt(0.0001)
        .total_time(12.0)
        .hit_threshold(5.0)
        .build()
}

fn test_4() -> Scenario {
    ScenarioBuilder::new("Spiral-Evasion")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(400.0, 750.0, 0.0),
            max_acceleration: 2800.0,
            navigation_constant: 7.0, // Aggressive tracking
            max_closing_speed: 1500.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(1200.0, 1800.0, 0.0),
            velocity: Vector3::new(200.0, 400.0, 300.0), // Corkscrew maneuver
        })
        .dt(0.0001)
        .total_time(15.0)
        .hit_threshold(5.0)
        .build()
}

fn test_5() -> Scenario {
    ScenarioBuilder::new("Terrain-Hugging-Chase")
        .missile_config(MissileConfig {
            position: Vector3::new(-500.0, 0.0, 250.0),
            velocity: Vector3::new(500.0, 750.0, 0.0),
            max_acceleration: 2200.0,
            navigation_constant: 6.0,
            max_closing_speed: 1300.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(1500.0, 1200.0, 50.0),
            velocity: Vector3::new(300.0, 400.0, -10.0),
        })
        .dt(0.0001)
        .total_time(10.0)
        .hit_threshold(5.0)
        .build()
}

fn test_6() -> Scenario {
    ScenarioBuilder::new("Hypersonic-Intercept")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 2000.0),
            velocity: Vector3::new(0.0, 1200.0, -300.0),
            max_acceleration: 4000.0,
            navigation_constant: 7.0,
            max_closing_speed: 2500.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(0.0, 4500.0, 2200.0),
            velocity: Vector3::new(0.0, -900.0, -100.0),
        })
        .dt(0.0001)
        .total_time(8.0)
        .hit_threshold(5.0)
        .build()
}

fn test_7() -> Scenario {
    ScenarioBuilder::new("Cinematic-Perpendicular")
        .missile_config(MissileConfig {
            position: Vector3::new(-3000.0, 0.0, 500.0),
            velocity: Vector3::new(600.0, 0.0, -50.0), // Lateral approach
            max_acceleration: 1900.0,
            navigation_constant: 5.0,
            max_closing_speed: 1300.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(0.0, 3000.0, 500.0),
            velocity: Vector3::new(0.0, -550.0, 0.0), // Perpendicular crossing
        })
        .dt(0.0001)
        .total_time(12.0)
        .hit_threshold(5.0)
        .build()
}
