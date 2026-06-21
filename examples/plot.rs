use anyhow::Result;
use colored::Colorize;
use missile_sim::prelude::*;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let bvr_intercept = Scenario::builder("BVR Fighter Intercept")
        .missile(Missile {
            state: State3D {
                position: Vector3::new(0.0, 0.0, 10000.0),
                velocity: Vector3::new(900.0, 0.0, 0.0), // Mach ~2.6
            },
            max_acceleration: 350.0, // ~35g
            navigation_constant: 4.0,
            max_closing_speed: 1800.0,
        })
        .target(Target {
            state: State3D {
                position: Vector3::new(25000.0, 3000.0, 10500.0),
                velocity: Vector3::new(-320.0, 20.0, 0.0), // fighter cruise
            },
            acceleration: Vector3::new(0.0, 25.0, 0.0),
        })
        .dt(0.0001)
        .total_time(35.0)
        .hit_threshold(10.0)
        .build()?;

    let sea_skimmer = Scenario::builder("Sea Skimming Cruise Missile Defense")
        .missile(Missile {
            state: State3D {
                position: Vector3::new(0.0, 0.0, 15.0),
                velocity: Vector3::new(1000.0, 0.0, 0.0),
            },
            max_acceleration: 400.0,
            navigation_constant: 5.0,
            max_closing_speed: 2000.0,
        })
        .target(Target {
            state: State3D {
                position: Vector3::new(18000.0, 100.0, 8.0),
                velocity: Vector3::new(-270.0, 0.0, 0.0),
            },
            acceleration: Vector3::zeros(),
        })
        .dt(0.0001)
        .total_time(20.0)
        .hit_threshold(3.0)
        .build()?;

    let sam_defense = Scenario::builder("SAM Against Diving Aircraft")
        .missile(Missile {
            state: State3D {
                position: Vector3::new(0.0, 0.0, 0.0),
                velocity: Vector3::new(1400.0, 0.0, 300.0),
            },
            max_acceleration: 300.0,
            navigation_constant: 4.5,
            max_closing_speed: 2200.0,
        })
        .target(Target {
            state: State3D {
                position: Vector3::new(35000.0, 5000.0, 12000.0),
                velocity: Vector3::new(-250.0, 0.0, -150.0),
            },
            acceleration: Vector3::new(0.0, 10.0, -5.0),
        })
        .dt(0.0001)
        .total_time(45.0)
        .hit_threshold(8.0)
        .build()?;

    let terminal_defense = Scenario::builder("Ballistic Terminal Intercept")
        .missile(Missile {
            state: State3D {
                position: Vector3::new(0.0, 0.0, 0.0),
                velocity: Vector3::new(1600.0, 0.0, 1800.0),
            },
            max_acceleration: 500.0,
            navigation_constant: 6.0,
            max_closing_speed: 3500.0,
        })
        .target(Target {
            state: State3D {
                position: Vector3::new(40000.0, 0.0, 35000.0),
                velocity: Vector3::new(-400.0, 0.0, -2200.0),
            },
            acceleration: Vector3::zeros(),
        })
        .dt(0.0001)
        .total_time(30.0)
        .hit_threshold(5.0)
        .build()?;

    let drone_intercept = Scenario::builder("Rotorcraft Anti-Drone Engagement")
        .missile(Missile {
            state: State3D {
                position: Vector3::new(0.0, 0.0, 300.0),
                velocity: Vector3::new(450.0, 0.0, 0.0),
            },
            max_acceleration: 80.0,
            navigation_constant: 4.0,
            max_closing_speed: 700.0,
        })
        .target(Target {
            state: State3D {
                position: Vector3::new(6000.0, 800.0, 500.0),
                velocity: Vector3::new(-60.0, 15.0, 0.0),
            },
            acceleration: Vector3::new(0.0, 2.0, 0.0),
        })
        .dt(0.0001)
        .total_time(18.0)
        .hit_threshold(2.0)
        .build()?;

    let hypersonic = Scenario::builder("Hypersonic Glide Vehicle Intercept")
        .missile(Missile {
            state: State3D {
                position: Vector3::new(0.0, 0.0, 15000.0),
                velocity: Vector3::new(2200.0, 0.0, 400.0),
            },
            max_acceleration: 450.0,
            navigation_constant: 5.0,
            max_closing_speed: 4500.0,
        })
        .target(Target {
            state: State3D {
                position: Vector3::new(60000.0, 7000.0, 30000.0),
                velocity: Vector3::new(-1800.0, -150.0, -250.0),
            },
            acceleration: Vector3::new(0.0, 30.0, 10.0),
        })
        .dt(0.0001)
        .total_time(40.0)
        .hit_threshold(15.0)
        .build()?;

    let scene = [
        bvr_intercept,
        sea_skimmer,
        sam_defense,
        terminal_defense,
        drone_intercept,
        hypersonic,
    ];

    let mut title = vec![];
    let mut metrics = vec![];

    let results: Vec<_> = scene
        .par_iter()
        .map(|s| {
            let m = s.simulate(&PurePursuit);
            println!(
                "Scene: {} Hit: {}",
                s.name,
                if m.hit { "YES".green() } else { "NO".red() }
            );
            (s.name.as_str(), m)
        })
        .collect();

    for (name, m) in results {
        title.push(name);
        metrics.push(m);
    }

    plot_projection(&metrics, PathBuf::from("./assets/Plot_PP.png"), &title)?;

    title.clear();
    metrics.clear();

    let results: Vec<_> = scene
        .par_iter()
        .map(|s| {
            let m = s.simulate(&PureProportionalNavigation);
            println!(
                "Scene: {} Hit: {}",
                s.name,
                if m.hit { "YES".green() } else { "NO".red() }
            );
            (s.name.as_str(), m)
        })
        .collect();

    for (name, m) in results {
        title.push(name);
        metrics.push(m);
    }

    plot_projection(&metrics, PathBuf::from("./assets/Plot_PPN.png"), &title)?;

    title.clear();
    metrics.clear();

    let results: Vec<_> = scene
        .par_iter()
        .map(|s| {
            let m = s.simulate(&TrueProportionalNavigation);
            println!(
                "Scene: {} Hit: {}",
                s.name,
                if m.hit { "YES".green() } else { "NO".red() }
            );
            (s.name.as_str(), m)
        })
        .collect();

    for (name, m) in results {
        title.push(name);
        metrics.push(m);
    }

    plot_projection(&metrics, PathBuf::from("./assets/Plot_TPN.png"), &title)?;

    Ok(())
}

pub fn plot_projection<P: AsRef<Path>>(
    metrics: &[SimulationMetrics],
    output: P,
    titles: &[&str],
) -> Result<()> {
    use plotters::prelude::*;

    let root = BitMapBackend::new(output.as_ref(), (3840, 2160)).into_drawing_area();

    root.fill(&BLACK)?;

    // 2 rows x 3 columns = 6 scene
    let chart_areas = root.split_evenly((2, 3));

    for (idx, area) in chart_areas.into_iter().enumerate() {
        let Some(metric) = metrics.get(idx) else {
            continue;
        };

        let missile: Vec<(f64, f64)> = metric
            .missile_trajectory
            .iter()
            .map(|p| (p.x / 1000.0, p.z / 1000.0)) // km
            .collect();

        let target: Vec<(f64, f64)> = metric
            .target_trajectory
            .iter()
            .map(|p| (p.x / 1000.0, p.z / 1000.0))
            .collect();

        let all_points: Vec<_> = missile.iter().chain(target.iter()).copied().collect();

        let x_min = all_points
            .iter()
            .map(|(x, _)| *x)
            .fold(f64::INFINITY, f64::min);

        let x_max = all_points
            .iter()
            .map(|(x, _)| *x)
            .fold(f64::NEG_INFINITY, f64::max);

        let y_min = all_points
            .iter()
            .map(|(_, y)| *y)
            .fold(f64::INFINITY, f64::min);

        let y_max = all_points
            .iter()
            .map(|(_, y)| *y)
            .fold(f64::NEG_INFINITY, f64::max);

        let x_pad = (x_max - x_min) * 0.05;
        let y_pad = (y_max - y_min) * 0.05;

        let miss_distance = metric
            .distance_records
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);

        let time_taken = metric.time_history.last().copied().unwrap_or(0.0);

        let initial_range = if let (Some(m), Some(t)) = (
            metric.missile_trajectory.first(),
            metric.target_trajectory.first(),
        ) {
            ((m.x - t.x).powi(2) + (m.y - t.y).powi(2) + (m.z - t.z).powi(2)).sqrt() / 1000.0
        } else {
            0.0
        };

        let title = format!(
            "{} | R₀ {:.1}km | MISS {:.1}m | {:.1}s",
            titles[idx], initial_range, miss_distance, time_taken
        );

        let mut chart = ChartBuilder::on(&area)
            .caption(title, ("0xProto Nerd Font", 18).into_font().color(&WHITE))
            .margin(15)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(
                (x_min - x_pad)..(x_max + x_pad),
                (y_min - y_pad)..(y_max + y_pad),
            )?;

        chart
            .configure_mesh()
            .label_style(("0xProto Nerd Font", 12).into_font().color(&WHITE))
            .axis_desc_style(("0xProto Nerd Font", 14).into_font().color(&WHITE))
            .x_desc("X (km) Horizontal Position")
            .y_desc("Z (km) Vertical Position")
            .bold_line_style(WHITE.mix(0.5).stroke_width(1))
            .light_line_style(WHITE.mix(0.25).stroke_width(1))
            .draw()?;

        // Missile trajectory glow
        chart.draw_series(LineSeries::new(
            missile.iter().copied(),
            RED.mix(0.25).stroke_width(8),
        ))?;

        chart.draw_series(LineSeries::new(
            missile.iter().copied(),
            RED.stroke_width(3),
        ))?;

        // Target trajectory glow
        chart.draw_series(LineSeries::new(
            target.iter().copied(),
            CYAN.mix(0.25).stroke_width(8),
        ))?;

        chart.draw_series(LineSeries::new(
            target.iter().copied(),
            CYAN.stroke_width(3),
        ))?;

        // Start markers
        if let Some(start) = missile.first() {
            chart.draw_series(std::iter::once(Circle::new(*start, 6, RED.filled())))?;
        }
        if let Some(start) = target.first() {
            chart.draw_series(std::iter::once(TriangleMarker::new(
                *start,
                8,
                CYAN.filled(),
            )))?;
        }

        // End markers
        if let Some(end) = missile.last() {
            chart.draw_series(std::iter::once(Cross::new(*end, 10, WHITE.stroke_width(2))))?;
        }
        if let Some(end) = target.last() {
            chart.draw_series(std::iter::once(Cross::new(*end, 10, WHITE.stroke_width(2))))?;
        }

        // Initial LOS line
        if let (Some(ms), Some(ts)) = (missile.first(), target.first()) {
            chart.draw_series(LineSeries::new(
                vec![*ms, *ts],
                WHITE.mix(1.6).stroke_width(1),
            ))?;
        }
    }

    root.present()?;
    Ok(())
}
