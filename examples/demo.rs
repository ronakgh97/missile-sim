//! Comprehensive demo of missile-sim library features.
//!
//! Run with: `cargo run --example demo`

use missile_sim::prelude::*;
use nalgebra::Vector3;

fn main() {
    demo_basic_scenario();
    demo_all_guidance_laws();
    demo_maneuvering_target();
    demo_custom_guidance();
    demo_game_loop_style();
    demo_2d_usage();
}

/// Basic scenario using the builder pattern and Scenario::simulate().
fn demo_basic_scenario() {
    println!("Basic Scenario");

    let scenario = Scenario::builder("Head-on-intercept")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(100.0, 0.0, 0.0),
            max_acceleration: 30.0,
            navigation_constant: 4.0,
            max_closing_speed: 1000.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(1000.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            acceleration: Vector3::zeros(),
        })
        .dt(0.01)
        .total_time(20.0)
        .hit_threshold(5.0)
        .build()
        .unwrap();

    let metrics = scenario.simulate(&PureProportionalNavigation);
    println!("Scenario: {}", scenario.name);
    println!("Result:   {}", metrics.console_summary());
    println!("Steps:    {}", metrics.time_history.len());
    println!();
}

/// Runs the same scenario with all six guidance laws and compares results.
fn demo_all_guidance_laws() {
    println!("All Guidance Laws");

    let guidance_laws: Vec<(&str, Box<dyn GuidanceLaw>)> = vec![
        ("PPN", Box::new(PureProportionalNavigation)),
        ("TPN", Box::new(TrueProportionalNavigation)),
        ("APN", Box::new(AugmentedProportionalNavigation::new(0.575))),
        ("PP", Box::new(PurePursuit)),
        ("DP", Box::new(DeviatedPursuit)),
        ("LP", Box::new(LeadPursuit::new(1.05))),
    ];

    let scenario = Scenario::builder("comparison")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(200.0, 100.0, 0.0),
            max_acceleration: 50.0,
            navigation_constant: 4.0,
            max_closing_speed: 2000.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(5000.0, 2000.0, 0.0),
            velocity: Vector3::new(-50.0, 0.0, 0.0),
            acceleration: Vector3::zeros(),
        })
        .dt(0.00001)
        .total_time(30.0)
        .hit_threshold(10.0)
        .build()
        .unwrap();

    println!(
        "{:<6} {:>10} {:>12} {:>8}",
        "Law", "Duration", "Miss Dist", "Hit"
    );
    println!("{}", "-".repeat(42));

    for (name, law) in &guidance_laws {
        let metrics = scenario.simulate(law.as_ref());
        println!(
            "{:<6} {:>8.2}s {:>10.2} {:>8}",
            name,
            metrics.time_history.last().unwrap_or(&0.0),
            metrics.miss_distance,
            if metrics.hit { "YES" } else { "NO" },
        );
    }
    println!();
}

/// Demonstrates a maneuvering target with constant acceleration.
fn demo_maneuvering_target() {
    println!("Maneuvering Target");

    let scenario = Scenario::builder("evasive-target")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(150.0, 50.0, 0.0),
            max_acceleration: 80.0,
            navigation_constant: 5.0,
            max_closing_speed: 2000.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(3000.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 80.0, 0.0),
            acceleration: Vector3::new(0.0, 15.0, 0.0),
        })
        .dt(0.001)
        .total_time(30.0)
        .hit_threshold(10.0)
        .build()
        .unwrap();

    let apn_metrics = scenario.simulate(&AugmentedProportionalNavigation::new(0.5));
    let ppn_metrics = scenario.simulate(&PureProportionalNavigation);

    println!("APN vs PPN against maneuvering target:");
    println!("  APN: {}", apn_metrics.console_summary());
    println!("  PPN: {}", ppn_metrics.console_summary());
    println!();
}

/// Shows how to implement a custom guidance law.
fn demo_custom_guidance() {
    println!("Custom Guidance Law");

    struct DirectAim;

    impl GuidanceLaw for DirectAim {
        fn calculate_acceleration(&self, missile: &Missile, target: &Target) -> Vector3<f64> {
            let range_vec = target.state.position - missile.state.position;
            let range = range_vec.norm();
            if range < 1e-6 {
                return Vector3::zeros();
            }
            range_vec.normalize() * missile.max_acceleration
        }

        fn name(&self) -> &str {
            "DirectAim"
        }
    }

    let scenario = Scenario::builder("custom-law")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(100.0, 0.0, 0.0),
            max_acceleration: 30.0,
            navigation_constant: 4.0,
            max_closing_speed: 1000.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(1000.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            acceleration: Vector3::zeros(),
        })
        .dt(0.01)
        .total_time(20.0)
        .hit_threshold(5.0)
        .build()
        .unwrap();

    let metrics = scenario.simulate(&DirectAim);
    println!("Custom DirectAim law: {}", metrics.console_summary());
    println!();
}

/// Demonstrates step-by-step simulation for real-time/game-loop usage.
fn demo_game_loop_style() {
    println!("Game Loop Style _step-by-step_");

    let missile = Missile::new(MissileConfig {
        position: Vector3::new(0.0, 0.0, 0.0),
        velocity: Vector3::new(100.0, 50.0, 0.0),
        max_acceleration: 30.0,
        navigation_constant: 4.0,
        max_closing_speed: 1000.0,
    });

    let target = Target::new(TargetConfig {
        position: Vector3::new(500.0, 500.0, 0.0),
        velocity: Vector3::new(-20.0, 0.0, 0.0),
        acceleration: Vector3::zeros(),
    });

    let mut engine = SimulationEngine::new(missile, target, 0.01, 20.0, 5.0);
    let mut metrics = SimulationMetrics::new();

    let guidance = PureProportionalNavigation;

    println!("Frame | Time   | Missile Pos          | Target Pos           | Distance");
    println!("{}", "-".repeat(85));

    let mut frame = 0;
    let mut last_print = 0;
    loop {
        engine.step(&guidance, &mut metrics);
        frame += 1;

        let m_pos = &engine.missile.state.position;
        let t_pos = &engine.target.state.position;
        let dist = (m_pos - t_pos).norm();

        if frame <= 3 || frame - last_print >= 15 {
            last_print = frame;
            println!(
                "{:>5} | {:>6.2} | ({:>6.1},{:>6.1},{:>6.1}) | ({:>6.1},{:>6.1},{:>6.1}) | {:>8.2}",
                frame, engine.time, m_pos.x, m_pos.y, m_pos.z, t_pos.x, t_pos.y, t_pos.z, dist,
            );
        }

        if dist < engine.hit_threshold {
            println!("... HIT at frame {}!", frame);
            break;
        }

        if frame > 5000 {
            println!("... simulation ended without hit");
            break;
        }
    }

    metrics.finalize(engine.hit_threshold);
    println!("Final: {}", metrics.console_summary());
    println!();
}

/// Shows 2D usage by zeroing the Z axis.
fn demo_2d_usage() {
    println!("2D Usage (Z = 0)");

    let scenario = Scenario::builder("2d-intercept")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(100.0, 50.0, 0.0),
            max_acceleration: 30.0,
            navigation_constant: 4.0,
            max_closing_speed: 1000.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(2000.0, 500.0, 0.0),
            velocity: Vector3::new(-30.0, 0.0, 0.0),
            acceleration: Vector3::zeros(),
        })
        .dt(0.01)
        .total_time(30.0)
        .hit_threshold(5.0)
        .build()
        .unwrap();

    let metrics = scenario.simulate(&PureProportionalNavigation);
    println!("2D scenario: {}", metrics.console_summary());

    let final_missile = metrics.missile_trajectory.last().unwrap();
    let final_target = metrics.target_trajectory.last().unwrap();
    println!(
        "Final positions - Missile: ({:.1}, {:.1}), Target: ({:.1}, {:.1})",
        final_missile.x, final_missile.y, final_target.x, final_target.y
    );
    println!();
}
