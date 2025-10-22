use crate::entity::{MissileConfig, TargetConfig};
use crate::prelude::Scenario;
use crate::simulation::ScenarioBuilder;
use nalgebra::Vector3;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::f64::consts::TAU;

pub fn load_train_data() -> Vec<Scenario> {
    let mut scenarios = Vec::new();

    for seed in 0..100 {
        scenarios.push(generate_random_scenario(seed));
    }

    scenarios
}

fn generate_random_scenario(seed: u64) -> Scenario {
    let mut rng = StdRng::seed_from_u64(seed);

    // Random position in 3D space
    let m_pos = Vector3::new(
        rng.random_range(-2000.0..2000.0),
        rng.random_range(500.0..3000.0),
        rng.random_range(-2000.0..2000.0),
    );

    // Random velocity (speed + direction)
    let m_speed = rng.random_range(800.0..2500.0);
    let m_azimuth = rng.random_range(0.0..TAU);
    let m_elevation: f64 = rng.random_range(-0.5..0.5);

    let m_vel = Vector3::new(
        m_speed * m_azimuth.cos() * m_elevation.cos(),
        m_speed * m_elevation.sin(),
        m_speed * m_azimuth.sin() * m_elevation.cos(),
    );

    // Random position
    let separation = rng.random_range(3000.0..10000.0);
    let t_direction = rng.random_range(0.0..TAU);

    let t_pos = Vector3::new(
        m_pos.x + separation * t_direction.cos(),
        rng.random_range(500.0..3000.0),
        m_pos.z + separation * t_direction.sin(),
    );

    // Random velocity
    let t_speed = rng.random_range(300.0..1500.0);
    let t_azimuth = rng.random_range(0.0..TAU);
    let t_elevation: f64 = rng.random_range(-0.3..0.3);

    let t_vel = Vector3::new(
        t_speed * t_azimuth.cos() * t_elevation.cos(),
        t_speed * t_elevation.sin(),
        t_speed * t_azimuth.sin() * t_elevation.cos(),
    );

    //  Random capabilities
    let max_accel = rng.random_range(1000.0..3000.0);
    let nav_const = rng.random_range(3.0..7.0);

    ScenarioBuilder::new(&format!("train_{}", seed))
        .missile_config(MissileConfig {
            position: m_pos,
            velocity: m_vel,
            max_acceleration: max_accel,
            navigation_constant: nav_const,
            max_closing_speed: 8000.0,
        })
        .target_config(TargetConfig {
            position: t_pos,
            velocity: t_vel,
        })
        .dt(0.0001) // 10kHz
        .total_time(30.0)
        .hit_threshold(10.0)
        .build()
}
