use crate::simulation::SimulationMetrics;

use plotters::prelude::*;

pub fn plot_3d_trajectory(
    metrics: &SimulationMetrics,
    filename: &str,
    title: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    // Calculate bounds
    let all_points: Vec<_> = metrics
        .missile_trajectory
        .iter()
        .chain(metrics.target_trajectory.iter())
        .collect();

    let min_x = all_points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min) - 200.0;
    let max_x = all_points
        .iter()
        .map(|p| p.x)
        .fold(f64::NEG_INFINITY, f64::max)
        + 200.0;
    let min_y = all_points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min) - 200.0;
    let max_y = all_points
        .iter()
        .map(|p| p.y)
        .fold(f64::NEG_INFINITY, f64::max)
        + 200.0;
    let min_z = all_points.iter().map(|p| p.z).fold(f64::INFINITY, f64::min) - 200.0;
    let max_z = all_points
        .iter()
        .map(|p| p.z)
        .fold(f64::NEG_INFINITY, f64::max)
        + 200.0;

    let mut chart = ChartBuilder::on(&root)
        .caption(&format!("{} ", title), ("sans-serif", 40))
        .margin(20)
        .build_cartesian_3d(min_x..max_x, min_z..max_z, min_y..max_y)?;

    chart.configure_axes().draw()?;

    // Draw missile trajectory
    chart.draw_series(LineSeries::new(
        metrics.missile_trajectory.iter().map(|p| (p.x, p.z, p.y)),
        &RED,
    ))?;

    // Draw target trajectory
    chart.draw_series(LineSeries::new(
        metrics.target_trajectory.iter().map(|p| (p.x, p.z, p.y)),
        &BLUE,
    ))?;

    // Mark start/end points
    if let Some(start) = metrics.missile_trajectory.first() {
        chart.draw_series(PointSeries::of_element(
            vec![(start.x, start.z, start.y)],
            5,
            &RED,
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))?;
    }

    if let Some(end) = metrics.missile_trajectory.last() {
        chart.draw_series(PointSeries::of_element(
            vec![(end.x, end.z, end.y)],
            8,
            &GREEN,
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))?;
    }

    root.present()?;
    Ok(())
}
