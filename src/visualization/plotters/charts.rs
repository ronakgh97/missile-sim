use crate::simulation::SimulationMetrics;
use anyhow::{Context, Result};
use plotters::prelude::*;

pub fn plot_metric_chart(
    time: &[f64],
    values: &[f64],
    filename: &str,
    title: &str,
    y_label: &str,
    width: u32,
    height: u32,
    color: &RGBColor,
) -> Result<()> {
    let root = BitMapBackend::new(filename, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min) * 0.95;
    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max) * 1.05;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(70)
        .build_cartesian_2d(
            time[0]..*time.last().context("Time history is empty")?,
            min_val..max_val,
        )?;

    chart
        .configure_mesh()
        .x_desc("Time (s)")
        .y_desc(y_label)
        .label_style(("sans-serif", 20))
        .draw()?;

    chart.draw_series(LineSeries::new(
        time.iter().zip(values.iter()).map(|(&t, &v)| (t, v)),
        color.stroke_width(2),
    ))?;

    root.present()?;
    Ok(())
}

/// Plot multiple metrics
pub fn plot_comparison_chart(
    time: &[f64],
    datasets: Vec<(&[f64], &str, &RGBColor)>,
    filename: &str,
    title: &str,
    y_label: &str,
    width: u32,
    height: u32,
) -> Result<()> {
    let root = BitMapBackend::new(filename, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    // Find global min/max
    let mut global_min = f64::INFINITY;
    let mut global_max = f64::NEG_INFINITY;

    for (values, _, _) in &datasets {
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        global_min = global_min.min(min);
        global_max = global_max.max(max);
    }

    global_min *= 0.95;
    global_max *= 1.05;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(70)
        .build_cartesian_2d(
            time[0]..*time.last().context("Time history is empty")?,
            global_min..global_max,
        )?;

    chart
        .configure_mesh()
        .x_desc("Time (s)")
        .y_desc(y_label)
        .label_style(("sans-serif", 20))
        .draw()?;

    // Draw each series
    for (values, label, color) in datasets {
        chart
            .draw_series(LineSeries::new(
                time.iter().zip(values.iter()).map(|(&t, &v)| (t, v)),
                color.stroke_width(2),
            ))?
            .label(label)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .label_font(("sans-serif", 20))
        .draw()?;

    root.present()?;
    Ok(())
}

pub fn plot_all_metrics(
    metrics: &SimulationMetrics,
    base_name: &str,
    width: u32,
    height: u32,
) -> Result<Vec<String>> {
    let mut output_files = Vec::new();
    let time = &metrics.time_history;

    // Separation Distance
    let filename = format!("{base_name}_separation_distance.png");
    plot_metric_chart(
        time,
        &metrics.distance_history,
        &filename,
        "Separation Distance",
        "Distance (m)",
        width,
        height,
        &RED,
    )?;
    output_files.push(filename);

    // Closing Velocity
    let filename = format!("{base_name}_closing_velocity.png");
    plot_metric_chart(
        time,
        &metrics.closing_speed_history,
        &filename,
        "Closing Velocity",
        "Velocity (m/s)",
        width,
        height,
        &BLUE,
    )?;
    output_files.push(filename);

    // Lateral Acceleration
    let filename = format!("{base_name}_lateral_acceleration.png");
    plot_metric_chart(
        time,
        &metrics.acceleration_history,
        &filename,
        "Lateral Acceleration (Guidance Command)",
        "Acceleration (m/s²)",
        width,
        height,
        &GREEN,
    )?;
    output_files.push(filename);

    // Line-of-Sight Angular Rate
    let filename = format!("{base_name}_los_angular_rate.png");
    plot_metric_chart(
        time,
        &metrics.los_rate_history,
        &filename,
        "Line-of-Sight Angular Rate",
        "Angular Rate (rad/s)",
        width,
        height,
        &MAGENTA,
    )?;
    output_files.push(filename);

    // Speed Comparison
    let missile_speeds = calculate_speed_history(&metrics.missile_trajectory, time);
    let target_speeds = calculate_speed_history(&metrics.target_trajectory, time);

    let filename = format!("{base_name}_speed_comparison.png");
    plot_comparison_chart(
        &time[1..],
        vec![
            (&missile_speeds, "Missile", &RED),
            (&target_speeds, "Target", &BLUE),
        ],
        &filename,
        "Speed Comparison",
        "Speed (m/s)",
        width,
        height,
    )?;
    output_files.push(filename);

    Ok(output_files)
}

// HELPER FUNCTIONS

/// Calculate speed magnitude from position history
fn calculate_speed_history(positions: &[nalgebra::Vector3<f64>], time: &[f64]) -> Vec<f64> {
    let mut speeds = Vec::new();

    for i in 1..positions.len() {
        let dt = time[i] - time[i - 1];
        if dt > 0.0 {
            let velocity = (positions[i] - positions[i - 1]) / dt;
            speeds.push(velocity.norm());
        }
    }

    speeds
}
