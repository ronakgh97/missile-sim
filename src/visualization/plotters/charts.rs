use crate::simulation::SimulationMetrics;
use plotters::prelude::*;

pub fn plot_metric_chart(
    time: &[f64],
    values: &[f64],
    filename: &str,
    title: &str,
    y_label: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min) * 0.9;
    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max) * 1.1;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(time[0]..*time.last().unwrap(), min_val..max_val)?;

    chart
        .configure_mesh()
        .x_desc("Time (s)")
        .y_desc(y_label)
        .draw()?;

    chart.draw_series(LineSeries::new(
        time.iter().zip(values.iter()).map(|(&t, &v)| (t, v)),
        &BLUE,
    ))?;

    root.present()?;
    Ok(())
}

pub fn plot_all_metrics(
    metrics: &SimulationMetrics,
    base_name: &str,
    width: u32,
    height: u32,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut output_files = Vec::new();
    let time = &metrics.time_history;

    // LOS Rate
    let filename = format!("{base_name}_los_rate.png");
    plot_metric_chart(
        time,
        &metrics.los_rate_history,
        &filename,
        "Line-of-Sight Rate Magnitude",
        "LOS Rate (rad/s)",
        width,
        height,
    )?;
    output_files.push(filename);

    // Distance
    let filename = format!("{base_name}_distance.png");
    plot_metric_chart(
        time,
        &metrics.distance_history,
        &filename,
        "Missile-Target Distance",
        "Distance (m)",
        width,
        height,
    )?;
    output_files.push(filename);

    // Acceleration
    let filename = format!("{base_name}_acceleration.png");
    plot_metric_chart(
        time,
        &metrics.acceleration_history,
        &filename,
        "Missile Acceleration Magnitude",
        "Acceleration (m/s²)",
        width,
        height,
    )?;
    output_files.push(filename);

    // Closing Speed
    let filename = format!("{base_name}_closing_speed.png");
    plot_metric_chart(
        time,
        &metrics.closing_speed_history,
        &filename,
        "Closing Speed",
        "Speed (m/s)",
        width,
        height,
    )?;
    output_files.push(filename);

    Ok(output_files)
}
