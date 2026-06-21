//! This bench runs a large number of simulations with different guidance laws and random scenarios,
//! and then save the results to a CSV file for further analysis.
//! It uses parallel processing for speed up

use colored::Colorize;
use missile_sim::prelude::*;
use rand::prelude::*;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::collections::HashMap;
use std::f64::consts::TAU;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, LazyLock, Mutex};
use std::time::Instant;

struct Record {
    scenario_name: Arc<String>,
    guidance_law: &'static str,
    hit: i8,
    miss_distance: f64,
    time_to_impact: f64,
}

static GLOBAL_RECORD: LazyLock<Arc<Mutex<Vec<Record>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

static COUNTER: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));

fn main() -> anyhow::Result<()> {
    let run_count = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or_else(|| {
            println!("Usage: bench <run_count>");
            std::process::exit(1);
        });

    let start_time = Instant::now();
    let laws: Vec<(&str, Box<dyn GuidanceLaw>)> = vec![
        ("PPN", Box::new(PureProportionalNavigation)),
        ("TPN", Box::new(TrueProportionalNavigation)),
        ("APN", Box::new(AugmentedProportionalNavigation::new(1.256))),
        ("PP", Box::new(PurePursuit)),
        ("LP", Box::new(LeadPursuit::new(1.256))),
    ];

    let random_scene: Vec<Scenario> = (0..run_count)
        .map(|i| {
            let mut rng = StdRng::seed_from_u64(i);
            generate_random_scenario(i, &mut rng).expect("Failed to generate scenario")
        })
        .collect();

    random_scene.into_par_iter().for_each(|scenario| {
        let shared_scenario_name = Arc::new(scenario.name.clone());
        for (law_name, law) in &laws {
            let metrics = scenario.simulate(law.as_ref());
            let record = Record {
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
                    run_count * laws.len() as u64,
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

    let file_name = format!("Metrics_{}.csv", run_count);
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

    println!(
        "CSV saved to {}, Time elapsed: {:?}",
        file_path.display(),
        start_time.elapsed()
    );

    let mut hit_summary: HashMap<&str, usize> = HashMap::with_capacity(laws.len());
    let mut miss_dist_summary: HashMap<&str, f64> = HashMap::with_capacity(laws.len());
    let mut impact_time_summary: HashMap<&str, f64> = HashMap::with_capacity(laws.len());

    // aggregate results
    for record in all_records.iter() {
        if record.hit == 1 {
            *hit_summary.entry(record.guidance_law).or_insert(0) += 1;
        }

        *miss_dist_summary.entry(record.guidance_law).or_insert(0.0) += record.miss_distance;
        *impact_time_summary
            .entry(record.guidance_law)
            .or_insert(0.0) += record.time_to_impact;
    }

    // average them
    for law in ["PPN", "TPN", "APN", "PP", "LP"] {
        if let Some(v) = miss_dist_summary.get_mut(law) {
            *v /= run_count as f64;
        }

        if let Some(v) = impact_time_summary.get_mut(law) {
            *v /= run_count as f64;
        }
    }

    println!();
    for (law, hits) in &hit_summary {
        let hit_rate = (*hits as f64 / run_count as f64) * 100.0;

        println!("- {}: {:.1}% hit rate ({} hits)", law, hit_rate, hits);
    }
    println!();
    for (law, avg_miss) in &miss_dist_summary {
        println!("- {}: {:.2} average miss distance", law, avg_miss);
    }
    println!();
    for (law, toi) in &impact_time_summary {
        println!("- {}: {:.2} impact time", law, toi);
    }

    plot_all_metrics(
        &hit_summary,
        &miss_dist_summary,
        &impact_time_summary,
        run_count as usize,
        &PathBuf::from(format!("./assets/Summary_{}.png", run_count)),
    )?;

    Ok(())
}

fn plot_all_metrics(
    hit_summary: &HashMap<&str, usize>,
    miss_summary: &HashMap<&str, f64>,
    impact_summary: &HashMap<&str, f64>,
    each_run: usize,
    file_path: &PathBuf,
) -> anyhow::Result<()> {
    use plotters::prelude::*;

    let root = BitMapBackend::new(file_path, (1800, 2160)).into_drawing_area();
    root.fill(&RGBColor(30, 34, 42))?;

    let laws = ["PPN", "TPN", "APN", "PP", "LP"];
    let areas = root.split_evenly((3, 1));

    // HIT GRAPH
    let max_hits = hit_summary.values().copied().max().unwrap_or(100);

    let mut chart = ChartBuilder::on(&areas[0])
        .caption(
            "Guidance Law Hit Comparison",
            ("0xProto Nerd Font", 32).into_font().color(&WHITE),
        )
        .margin(40)
        .x_label_area_size(60)
        .y_label_area_size(80)
        .build_cartesian_2d(-0.5..(laws.len() as f64 - 0.5), 0..(max_hits + 100))?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(RGBColor(60, 66, 78))
        .light_line_style(RGBColor(45, 50, 60))
        .label_style(
            ("0xProto Nerd Font", 18)
                .into_font()
                .color(&RGBColor(180, 190, 200)),
        )
        .axis_desc_style(("0xProto Nerd Font", 22).into_font().color(&WHITE))
        .x_desc("Guidance Law")
        .y_desc("Successful Hits")
        .x_labels(laws.len())
        .x_label_formatter(&|x| {
            let idx = x.round() as usize;
            if idx < laws.len() {
                laws[idx].to_string()
            } else {
                "".to_string()
            }
        })
        .draw()?;

    let bar_color = RGBColor(0, 180, 216);

    chart.draw_series(laws.iter().enumerate().map(|(idx, law)| {
        let hits = *hit_summary.get(law).unwrap_or(&0);
        let x = idx as f64;
        Rectangle::new([(x - 0.3, 0), (x + 0.3, hits)], bar_color.filled())
    }))?;

    chart.draw_series(laws.iter().enumerate().map(|(idx, law)| {
        let hits = *hit_summary.get(law).unwrap_or(&0);
        let pct = hits as f64 / each_run as f64 * 100.0;
        let x = idx as f64;

        Text::new(
            format!("{:.1}%", pct),
            (x - 0.14, hits + 25),
            ("0xProto Nerd Font", 16).into_font().color(&WHITE),
        )
    }))?;

    // MISS DISTANCE GRAPH
    let max_miss = miss_summary.values().copied().fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&areas[1])
        .caption(
            "Average Miss Distance",
            ("0xProto Nerd Font", 32).into_font().color(&WHITE),
        )
        .margin(40)
        .x_label_area_size(60)
        .y_label_area_size(80)
        .build_cartesian_2d(-0.5..(laws.len() as f64 - 0.5), 0.0..(max_miss + 100.0))?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(RGBColor(60, 66, 78))
        .light_line_style(RGBColor(45, 50, 60))
        .label_style(
            ("0xProto Nerd Font", 18)
                .into_font()
                .color(&RGBColor(180, 190, 200)),
        )
        .axis_desc_style(("0xProto Nerd Font", 22).into_font().color(&WHITE))
        .x_desc("Guidance Law")
        .y_desc("Miss Distance")
        .x_labels(laws.len())
        .x_label_formatter(&|x| {
            let idx = x.round() as usize;
            if idx < laws.len() {
                laws[idx].to_string()
            } else {
                "".to_string()
            }
        })
        .draw()?;

    chart.draw_series(laws.iter().enumerate().map(|(idx, law)| {
        let val = *miss_summary.get(law).unwrap_or(&0.0);
        let x = idx as f64;
        Rectangle::new([(x - 0.3, 0.0), (x + 0.3, val)], bar_color.filled())
    }))?;

    // TIME TO IMPACT GRAPH
    let max_time = impact_summary.values().copied().fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&areas[2])
        .caption(
            "Average Time To Impact",
            ("0xProto Nerd Font", 32).into_font().color(&WHITE),
        )
        .margin(40)
        .x_label_area_size(60)
        .y_label_area_size(80)
        .build_cartesian_2d(-0.5..(laws.len() as f64 - 0.5), 0.0..(max_time + 10.0))?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(RGBColor(60, 66, 78))
        .light_line_style(RGBColor(45, 50, 60))
        .label_style(
            ("0xProto Nerd Font", 18)
                .into_font()
                .color(&RGBColor(180, 190, 200)),
        )
        .axis_desc_style(("0xProto Nerd Font", 22).into_font().color(&WHITE))
        .x_desc("Guidance Law")
        .y_desc("Time To Impact")
        .x_labels(laws.len())
        .x_label_formatter(&|x| {
            let idx = x.round() as usize;
            if idx < laws.len() {
                laws[idx].to_string()
            } else {
                "".to_string()
            }
        })
        .draw()?;

    chart.draw_series(laws.iter().enumerate().map(|(idx, law)| {
        let val = *impact_summary.get(law).unwrap_or(&0.0);
        let x = idx as f64;
        Rectangle::new([(x - 0.3, 0.0), (x + 0.3, val)], bar_color.filled())
    }))?;

    root.present()?;
    Ok(())
}

#[inline(always)]
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
        rng.random_range(-10.0..15.0),
        rng.random_range(-10.0..15.0),
        rng.random_range(-10.0..15.0),
    );

    let nav_const = rng.random_range(3.0..8.0);

    ScenarioBuilder::new(&format!("random_{}", seed))
        .missile(Missile {
            state: State3D {
                position: m_pos,
                velocity: m_vel,
            },
            max_acceleration: m_acc,
            navigation_constant: nav_const,
            max_closing_speed: 8000.0,
        })
        .target(Target {
            state: State3D {
                position: t_pos,
                velocity: t_vel,
            },
            acceleration: t_acc,
        })
        .dt(0.0001)
        .total_time(60.0)
        .hit_threshold(10.0)
        .build()
}
