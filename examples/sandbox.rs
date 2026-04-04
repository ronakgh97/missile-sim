use colored::Colorize;
use missile_sim::prelude::*;
use rand::prelude::*;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::f64::consts::TAU;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, LazyLock, Mutex};

struct SummaryRecord {
    scenario_name: Arc<String>,
    guidance_law: &'static str,
    hit: i8,
    miss_distance: f64,
    time_to_impact: f64,
}

static GLOBAL_RECORD: LazyLock<Arc<Mutex<Vec<SummaryRecord>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

static COUNTER: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));

fn main() -> anyhow::Result<()> {
    let each_run = 1500;

    let laws: Vec<(&str, Box<dyn GuidanceLaw>)> = vec![
        ("PPN", Box::new(PureProportionalNavigation)),
        ("TPN", Box::new(TrueProportionalNavigation)),
        ("APN", Box::new(AugmentedProportionalNavigation::new(1.225))),
        ("PP", Box::new(PurePursuit)),
        ("DP", Box::new(DeviatedPursuit)),
        ("LP", Box::new(LeadPursuit::new(1.25))),
    ];

    let random_scene: Vec<Scenario> = (0..each_run)
        .map(|i| {
            let mut rng = StdRng::seed_from_u64(i);
            generate_random_scenario(i, &mut rng).expect("Failed to generate scenario")
        })
        .collect();

    random_scene.into_par_iter().for_each(|scenario| {
        let shared_scenario_name = Arc::new(scenario.name.clone());
        for (law_name, law) in &laws {
            let metrics = scenario.simulate(law.as_ref());
            let record = SummaryRecord {
                scenario_name: Arc::clone(&shared_scenario_name),
                guidance_law: law_name,
                hit: if metrics.hit { 1 } else { 0 },
                miss_distance: metrics.miss_distance,
                time_to_impact: *metrics.time_history.last().unwrap_or(&0.0),
            };

            {
                GLOBAL_RECORD
                    .lock()
                    .expect("Failed to acquire lock on global record")
                    .push(record);

                COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                println!(
                    "[{} / {}] Laws: {} | Hit: {}",
                    COUNTER.load(std::sync::atomic::Ordering::SeqCst),
                    each_run * laws.len() as u64,
                    law_name,
                    if metrics.hit {
                        "YES".green()
                    } else {
                        "NO".red()
                    },
                );
            }
        }
    });

    let file_name = format!("Summary_{}.csv", each_run);
    let file_path = PathBuf::from(file_name);

    let mut file = File::create(&file_path)?;
    writeln!(
        file,
        "scenario_name,guidance_law,hit,miss_distance,time_to_impact"
    )?;

    let all_records = GLOBAL_RECORD.lock().expect("Failed to acquire lock");
    for record in all_records.iter() {
        writeln!(
            file,
            "{},{},{},{:.2},{:.2}",
            record.scenario_name,
            record.guidance_law,
            record.hit,
            record.miss_distance,
            record.time_to_impact
        )?;
    }

    println!("Results saved to {}", file_path.display());
    Ok(())
}

#[inline]
fn generate_random_scenario(seed: u64, rng: &mut StdRng) -> anyhow::Result<Scenario> {
    let m_pos = Vector3::new(
        rng.random_range(-2000.0..2000.0),
        rng.random_range(-2000.0..2000.0),
        rng.random_range(-2000.0..2000.0),
    );

    let separation = rng.random_range(2000.0..10000.0);
    let t_direction = rng.random_range(0.0..TAU);

    let t_pos = Vector3::new(
        m_pos.x + separation * t_direction.cos(),
        rng.random_range(500.0..2000.0),
        m_pos.z + separation * t_direction.sin(),
    );

    let m_speed = rng.random_range(800.0..2500.0);
    let m_azimuth = t_direction + rng.random_range(-0.5..0.5);
    let m_elevation = rng.random_range(-0.5..0.5);

    let m_vel = Vector3::new(
        m_speed * m_azimuth.cos() * m_elevation.cos(),
        m_speed * m_elevation.sin(),
        m_speed * m_azimuth.sin() * m_elevation.cos(),
    );
    let m_acc = rng.random_range(500.0..1000.0);

    let t_speed = rng.random_range(300.0..1500.0);
    let t_azimuth = rng.random_range(0.0..TAU);
    let t_elevation: f64 = rng.random_range(-0.3..0.3);

    let t_vel = Vector3::new(
        t_speed * t_azimuth.cos() * t_elevation.cos(),
        t_speed * t_elevation.sin(),
        t_speed * t_azimuth.sin() * t_elevation.cos(),
    );
    let t_acc = Vector3::new(
        rng.random_range(-5.0..5.0),
        rng.random_range(-5.0..5.0),
        rng.random_range(-5.0..5.0),
    );

    let nav_const = rng.random_range(3.0..8.0);

    ScenarioBuilder::new(&format!("random_{}", seed))
        .missile_config(MissileConfig {
            position: m_pos,
            velocity: m_vel,
            max_acceleration: m_acc,
            navigation_constant: nav_const,
            max_closing_speed: 8000.0,
        })
        .target_config(TargetConfig {
            position: t_pos,
            velocity: t_vel,
            acceleration: t_acc,
        })
        .dt(0.0001)
        .total_time(60.0)
        .hit_threshold(10.0)
        .build()
}
