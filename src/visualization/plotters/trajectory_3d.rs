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

    // Calculate bounds with padding
    let all_points: Vec<_> = metrics
        .missile_trajectory
        .iter()
        .chain(metrics.target_trajectory.iter())
        .collect();

    let min_x = all_points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min) - 500.0;
    let max_x = all_points
        .iter()
        .map(|p| p.x)
        .fold(f64::NEG_INFINITY, f64::max)
        + 500.0;
    let min_y = all_points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min) - 500.0;
    let max_y = all_points
        .iter()
        .map(|p| p.y)
        .fold(f64::NEG_INFINITY, f64::max)
        + 500.0;
    let min_z = all_points.iter().map(|p| p.z).fold(f64::INFINITY, f64::min) - 500.0;
    let max_z = all_points
        .iter()
        .map(|p| p.z)
        .fold(f64::NEG_INFINITY, f64::max)
        + 500.0;

    // Build plot
    let mut chart = ChartBuilder::on(&root)
        .caption(&format!("{}", title), ("Lucida Console", 40))
        .margin(15)
        .build_cartesian_3d(min_x..max_x, min_z..max_z, min_y..max_y)?;

    // Configure axes with labels
    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(4)
        .x_labels(5)
        .y_labels(5)
        .z_labels(5)
        .label_style(("Lucida Console", 20))
        .draw()?;

    // Missile path
    chart
        .draw_series(LineSeries::new(
            metrics.missile_trajectory.iter().map(|p| (p.x, p.z, p.y)),
            &RED.mix(0.8),
        ))?
        .label("Missile")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Missile start position
    if let Some(start) = metrics.missile_trajectory.first() {
        chart.draw_series(PointSeries::of_element(
            vec![(start.x, start.z, start.y)],
            8,
            ShapeStyle::from(&RED).filled(),
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))?;

        // Label
        chart.draw_series(std::iter::once(Text::new(
            "Initial",
            (start.x, start.z + 100.0, start.y),
            ("Lucida Console", 18).into_font().color(&RED),
        )))?;
    }

    // Missile end position
    if let Some(end) = metrics.missile_trajectory.last() {
        let size = 12.0;
        chart.draw_series(LineSeries::new(
            vec![
                (end.x - size, end.z - size, end.y),
                (end.x + size, end.z + size, end.y),
            ],
            RED.stroke_width(3),
        ))?;
        chart.draw_series(LineSeries::new(
            vec![
                (end.x - size, end.z + size, end.y),
                (end.x + size, end.z - size, end.y),
            ],
            RED.stroke_width(3),
        ))?;

        // Label
        chart.draw_series(std::iter::once(Text::new(
            "Final",
            (end.x, end.z + 150.0, end.y),
            ("Lucida Console", 18).into_font().color(&RED),
        )))?;
    }

    // Target path
    chart
        .draw_series(LineSeries::new(
            metrics.target_trajectory.iter().map(|p| (p.x, p.z, p.y)),
            &BLUE.mix(0.8),
        ))?
        .label("Target")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Target start position (blue filled circle)
    if let Some(start) = metrics.target_trajectory.first() {
        chart.draw_series(PointSeries::of_element(
            vec![(start.x, start.z, start.y)],
            8,
            ShapeStyle::from(&BLUE).filled(),
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))?;

        // Label
        chart.draw_series(std::iter::once(Text::new(
            "Initial",
            (start.x, start.z - 100.0, start.y),
            ("Lucida Console", 18).into_font().color(&BLUE),
        )))?;
    }

    // Target end position
    if let Some(end) = metrics.target_trajectory.last() {
        let size = 12.0;
        chart.draw_series(LineSeries::new(
            vec![
                (end.x - size, end.z - size, end.y),
                (end.x + size, end.z + size, end.y),
            ],
            BLUE.stroke_width(3),
        ))?;
        chart.draw_series(LineSeries::new(
            vec![
                (end.x - size, end.z + size, end.y),
                (end.x + size, end.z - size, end.y),
            ],
            BLUE.stroke_width(3),
        ))?;

        // Label
        chart.draw_series(std::iter::once(Text::new(
            "Final",
            (end.x, end.z - 150.0, end.y),
            ("Lucida Console", 18).into_font().color(&BLUE),
        )))?;
    }

    if let Some(closest_idx) = metrics
        .distance_history
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, _)| idx)
    {
        if closest_idx < metrics.missile_trajectory.len()
            && closest_idx < metrics.target_trajectory.len()
        {
            let m_pos = &metrics.missile_trajectory[closest_idx];
            let t_pos = &metrics.target_trajectory[closest_idx];

            chart.draw_series(LineSeries::new(
                vec![(m_pos.x, m_pos.z, m_pos.y), (t_pos.x, t_pos.z, t_pos.y)],
                GREEN.mix(0.6).stroke_width(2),
            ))?;

            // Miss distance label at midpoint
            let _mid_x = (m_pos.x + t_pos.x) / 2.0;
            let _mid_z = (m_pos.z + t_pos.z) / 2.0;
            let _mid_y = (m_pos.y + t_pos.y) / 2.0;
        }
    }

    // Configure legend
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font(("sans-serif", 20))
        .draw()?;

    root.present()?;
    Ok(())
}
