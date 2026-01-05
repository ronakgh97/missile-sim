use crate::simulation::SimulationMetrics;
use anyhow::Result;
use nalgebra::Vector3;
use plotters::coord::cartesian::Cartesian3d;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use std::fs;

/// Default plot dimensions
pub const DEFAULT_WIDTH: u32 = 1024;
pub const DEFAULT_HEIGHT: u32 = 768;

/// Render a 3D trajectory plot and save to file
/// Creates directory if needed
pub fn render_trajectory_3d(
    metrics: &SimulationMetrics,
    output_dir: &str,
    scenario_name: &str,
    guidance_name: &str,
) -> Result<String> {
    fs::create_dir_all(output_dir)?;
    
    let filename = format!("{output_dir}/{guidance_name}_trajectory.png");
    let title = format!("{scenario_name} - {guidance_name}");
    
    plot_3d_trajectory(metrics, &filename, &title, DEFAULT_WIDTH, DEFAULT_HEIGHT)?;
    
    Ok(filename)
}

pub fn plot_3d_trajectory(
    metrics: &SimulationMetrics,
    filename: &str,
    title: &str,
    width: u32,
    height: u32,
) -> Result<()> {
    let root = BitMapBackend::new(filename, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

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

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("0xProto Nerd Font", 40))
        .margin(15)
        .build_cartesian_3d(min_x..max_x, min_y..max_y, min_z..max_z)?;

    chart
        .configure_axes()
        .light_grid_style(BLACK)
        .max_light_lines(4)
        .x_labels(4)
        .y_labels(4)
        .z_labels(4)
        .label_style(("0xProto Nerd Font", 20))
        .draw()?;

    // Calculate metrics for legend
    let m_start = metrics.missile_trajectory.first();
    let m_end = metrics.missile_trajectory.last();
    let t_start = metrics.target_trajectory.first();
    let t_end = metrics.target_trajectory.last();

    let initial_range = if let (Some(ms), Some(ts)) = (m_start, t_start) {
        ((ms.x - ts.x).powi(2) + (ms.y - ts.y).powi(2) + (ms.z - ts.z).powi(2)).sqrt()
    } else {
        0.0
    };

    let miss_distance = metrics
        .distance_history
        .iter()
        .fold(f64::INFINITY, |acc, &d| acc.min(d));

    let time_taken = metrics.time_history.last().copied().unwrap_or(0.0);

    // Draw missile trajectory (no in-graph labels)
    chart
        .draw_series(LineSeries::new(
            metrics.missile_trajectory.iter().map(|p| (p.x, p.y, p.z)),
            RED.stroke_width(2),
        ))?
        .label("Missile Trajectory")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.stroke_width(2)));

    if let Some(start) = m_start {
        draw_marker(&mut chart, (start.x, start.y, start.z), 10, &RED)?;
    }

    if let Some(end) = m_end {
        draw_marker(&mut chart, (end.x, end.y, end.z), 10, &RED)?;
    }

    // Draw target trajectory (no in-graph labels)
    chart
        .draw_series(LineSeries::new(
            metrics.target_trajectory.iter().map(|p| (p.x, p.y, p.z)),
            BLUE.stroke_width(2),
        ))?
        .label("Target Trajectory")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.stroke_width(2)));

    if let Some(start) = t_start {
        draw_marker(&mut chart, (start.x, start.y, start.z), 8, &BLUE)?;
    }

    if let Some(end) = t_end {
        draw_marker(&mut chart, (end.x, end.y, end.z), 8, &BLUE)?;
    }

    // Draw miss distance line (no label)
    if let Some(closest_idx) = metrics
        .distance_history
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, _)| idx)
        && closest_idx < metrics.missile_trajectory.len()
        && closest_idx < metrics.target_trajectory.len()
    {
        let m_pos = &metrics.missile_trajectory[closest_idx];
        let t_pos = &metrics.target_trajectory[closest_idx];

        chart
            .draw_series(LineSeries::new(
                vec![(m_pos.x, m_pos.y, m_pos.z), (t_pos.x, t_pos.y, t_pos.z)],
                GREEN.stroke_width(3),
            ))?
            .label("Miss Distance")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN.stroke_width(3)));
    }

    // Draw initial range line (no label)
    if let (Some(ms), Some(ts)) = (m_start, t_start) {
        chart
            .draw_series(LineSeries::new(
                vec![(ms.x, ms.y, ms.z), (ts.x, ts.y, ts.z)],
                BLACK.stroke_width(1),
            ))?
            .label("Initial Range")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK.stroke_width(1)));
    }

    // Add legend entries for positions
    chart
        .draw_series(std::iter::empty::<PathElement<_>>())?
        .label(format!(
            "Missile Start: ({:.0}, {:.0}, {:.0}) m",
            m_start.map_or(0.0, |p| p.x),
            m_start.map_or(0.0, |p| p.y),
            m_start.map_or(0.0, |p| p.z)
        ))
        .legend(|(x, y)| Circle::new((x + 10, y), 5, RED.filled()));

    chart
        .draw_series(std::iter::empty::<PathElement<_>>())?
        .label(format!(
            "Missile End: ({:.0}, {:.0}, {:.0}) m",
            m_end.map_or(0.0, |p| p.x),
            m_end.map_or(0.0, |p| p.y),
            m_end.map_or(0.0, |p| p.z)
        ))
        .legend(|(x, y)| Circle::new((x + 10, y), 5, RED.filled()));

    chart
        .draw_series(std::iter::empty::<PathElement<_>>())?
        .label(format!(
            "Target Start: ({:.0}, {:.0}, {:.0}) m",
            t_start.map_or(0.0, |p| p.x),
            t_start.map_or(0.0, |p| p.y),
            t_start.map_or(0.0, |p| p.z)
        ))
        .legend(|(x, y)| Circle::new((x + 10, y), 5, BLUE.filled()));

    chart
        .draw_series(std::iter::empty::<PathElement<_>>())?
        .label(format!(
            "Target End: ({:.0}, {:.0}, {:.0}) m",
            t_end.map_or(0.0, |p| p.x),
            t_end.map_or(0.0, |p| p.y),
            t_end.map_or(0.0, |p| p.z)
        ))
        .legend(|(x, y)| Circle::new((x + 10, y), 5, BLUE.filled()));

    // Add metrics to legend
    chart
        .draw_series(std::iter::empty::<PathElement<_>>())?
        .label(format!("Initial Range: {:.2} m", initial_range))
        .legend(|(x, y)| EmptyElement::at((x, y)));

    chart
        .draw_series(std::iter::empty::<PathElement<_>>())?
        .label(format!("Miss Distance: {:.2} m", miss_distance))
        .legend(|(x, y)| EmptyElement::at((x, y)));

    chart
        .draw_series(std::iter::empty::<PathElement<_>>())?
        .label(format!("Time: {:.3} s", time_taken))
        .legend(|(x, y)| EmptyElement::at((x, y)));

    // Configure and draw legend
    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.95))
        .border_style(BLACK)
        .label_font(("0xProto Nerd Font", 16))
        .margin(10)
        .draw()?;

    root.present()?;
    Ok(())
}

// HELPER FUNCTIONS

/// Draw a simple filled circle marker
fn draw_marker(
    chart: &mut ChartContext<
        '_,
        BitMapBackend,
        Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>,
    >,
    position: (f64, f64, f64),
    radius: i32,
    color: &RGBColor,
) -> Result<()> {
    chart.draw_series(PointSeries::of_element(
        vec![position],
        radius,
        ShapeStyle::from(color).filled(),
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;

    Ok(())
}

/// Draw velocity vectors along a 3D trajectory
#[allow(dead_code)]
fn draw_velocity_vectors(
    chart: &mut ChartContext<
        '_,
        BitMapBackend,
        Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>,
    >,
    trajectory: &[Vector3<f64>],
    color: &RGBColor,
    scale: f64,
) -> Result<()> {
    for i in 0..trajectory.len().saturating_sub(1) {
        let p1 = trajectory[i];
        let p2 = trajectory[i + 1];

        // Compute velocity vector
        let v = (p2 - p1) * scale;

        // Draw velocity arrow
        chart.draw_series(LineSeries::new(
            vec![(p1.x, p1.y, p1.z), (p1.x + v.x, p1.y + v.y, p1.z + v.z)],
            ShapeStyle::from(color).stroke_width(3),
        ))?;

        // Arrowhead marker
        chart.draw_series(std::iter::once(TriangleMarker::new(
            (p1.x + v.x, p1.y + v.y, p1.z + v.z),
            8,
            ShapeStyle::from(color).filled(),
        )))?;
    }

    Ok(())
}
