use crate::simulation::SimulationMetrics;
use nalgebra::Vector3;
use plotters::coord::cartesian::Cartesian3d;
use plotters::coord::types::RangedCoordf64;
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
        .caption(title, ("Lucida Console", 40))
        .margin(15)
        .build_cartesian_3d(min_x..max_x, min_y..max_y, min_z..max_z)?;

    chart
        .configure_axes()
        .light_grid_style(BLACK)
        .max_light_lines(4)
        .x_labels(4)
        .y_labels(4)
        .z_labels(4)
        .label_style(("Lucida Console", 20))
        .draw()?;

    // Draw missile trajectory
    chart
        .draw_series(LineSeries::new(
            metrics.missile_trajectory.iter().map(|p| (p.x, p.y, p.z)),
            &RED,
        ))?
        .label("Missile")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    if let Some(start) = metrics.missile_trajectory.first() {
        draw_start_marker(&mut chart, (start.x, start.y, start.z), 10, &RED)?;
    }

    if let Some(end) = metrics.missile_trajectory.last() {
        draw_end_marker(&mut chart, (end.x, end.y, end.z), 25.0, &GREEN)?;
    }

    // Draw target trajectory
    chart
        .draw_series(LineSeries::new(
            metrics.target_trajectory.iter().map(|p| (p.x, p.y, p.z)),
            &BLUE,
        ))?
        .label("Target")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    if let Some(start) = metrics.target_trajectory.first() {
        draw_start_marker(&mut chart, (start.x, start.y, start.z), 8, &BLUE)?;
    }

    if let Some(end) = metrics.target_trajectory.last() {
        draw_end_marker(&mut chart, (end.x, end.y, end.z), 25.0, &GREEN)?;
    }

    // Draw miss distance line
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
                vec![(m_pos.x, m_pos.y, m_pos.z), (t_pos.x, t_pos.y, t_pos.z)],
                GREEN.stroke_width(10),
            ))?;

            let mid = (
                (m_pos.x + t_pos.x) / 2.0,
                (m_pos.y + t_pos.y) / 2.0,
                (m_pos.z + t_pos.z) / 2.0,
            );
            draw_labeled_dot(&mut chart, mid, &BLACK)?;
        }
    }

    // Draw initial line (from missile start to target start)
    if let (Some(m_start), Some(t_start)) = (
        metrics.missile_trajectory.first(),
        metrics.target_trajectory.first(),
    ) {
        chart.draw_series(LineSeries::new(
            vec![
                (m_start.x, m_start.y, m_start.z),
                (t_start.x, t_start.y, t_start.z),
            ],
            ShapeStyle::from(&BLACK).stroke_width(1),
        ))?;

        // Compute midpoint for label placement
        let mid = (
            (m_start.x + t_start.x) / 2.0,
            (m_start.y + t_start.y) / 2.0,
            (m_start.z + t_start.z) / 2.0,
        );

        // Compute distance
        let range = ((m_start.x - t_start.x).powi(2)
            + (m_start.y - t_start.y).powi(2)
            + (m_start.z - t_start.z).powi(2))
        .sqrt();

        // Draw "Range" label
        chart.draw_series(std::iter::once(Text::new(
            format!("Range: {range:.2} m"),
            (mid.0 + 200.0, mid.1 + 200.0, mid.2 + 1000.0),
            ("Lucida Console", 25).into_font().color(&BLACK),
        )))?;
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .label_font(("Lucida Console", 20))
        .draw()?;

    root.present()?;
    Ok(())
}

// HELPER FUNCTIONS

/// Draw a filled circle marker at start position
fn draw_start_marker(
    chart: &mut ChartContext<
        '_,
        BitMapBackend,
        Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>,
    >,
    position: (f64, f64, f64),
    radius: i32,
    color: &RGBColor,
) -> Result<(), Box<dyn std::error::Error>> {
    chart.draw_series(PointSeries::of_element(
        vec![position],
        radius,
        ShapeStyle::from(color).filled(),
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;
    draw_labeled_dot(chart, position, color)?;
    Ok(())
}

/// Draw an X marker at end position
fn draw_end_marker(
    chart: &mut ChartContext<
        '_,
        BitMapBackend,
        Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>,
    >,
    position: (f64, f64, f64),
    size: f64,
    color: &RGBColor,
) -> Result<(), Box<dyn std::error::Error>> {
    // First diagonal
    chart.draw_series(LineSeries::new(
        vec![
            (position.0 - size, position.1 - size, position.2),
            (position.0 + size, position.1 + size, position.2),
        ],
        color.stroke_width(10),
    ))?;

    // Second diagonal
    chart.draw_series(LineSeries::new(
        vec![
            (position.0 - size, position.1 + size, position.2),
            (position.0 + size, position.1 - size, position.2),
        ],
        color.stroke_width(10),
    ))?;

    Ok(())
}

/// Draw a dot with coordinate label
pub fn draw_labeled_dot(
    chart: &mut ChartContext<
        '_,
        BitMapBackend,
        Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>,
    >,
    position: (f64, f64, f64),
    color: &RGBColor,
) -> Result<(), Box<dyn std::error::Error>> {
    chart.draw_series(std::iter::once(Circle::new(position, 2, color.filled())))?;

    chart.draw_series(std::iter::once(Text::new(
        format!("({:.2}, {:.2}, {:.2})", position.0, position.1, position.2),
        (position.0 + 100.0, position.1 + 100.0, position.2 + 1000.0),
        ("Lucida Console", 25).into_font().color(color),
    )))?;

    Ok(())
}

/// Draw velocity vectors along a 3D trajectory
fn draw_velocity_vectors(
    chart: &mut ChartContext<
        '_,
        BitMapBackend,
        Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>,
    >,
    trajectory: &Vec<Vector3<f64>>,
    color: &RGBColor,
    scale: f64,
) -> Result<(), Box<dyn std::error::Error>> {
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
