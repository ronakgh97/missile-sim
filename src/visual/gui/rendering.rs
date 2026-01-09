use anyhow::Result;
use macroquad::prelude::*;

///
/// Hey its my birthday today...
///
/// I became 19 today, kiddo!!
///
/// I dont know how to celebrate birthdays anymore, SO I JUST GONNA CODE, LISTEN TO SOME JAZZ.
///
/// From 'N' year from now:
/// - I hope you're still coding while you can...maybe
/// - I hope you're kinder to yourself
/// - I hope your family crap is finally over

pub async fn render_master() -> Result<()> {
    sections_lines().await?;

    render_grid(15).await?;

    Ok(())
}

/// Split the screen into 4 sections with 2 lines
pub async fn sections_lines() -> Result<()> {
    let screen_width = screen_width();
    let screen_height = screen_height();

    draw_line(
        screen_width / 2.0,
        0.0,
        screen_width / 2.0,
        screen_height,
        5.0,
        BLUE,
    );
    draw_line(
        0.0,
        screen_height / 2.0,
        screen_width,
        screen_height / 2.0,
        5.0,
        BLUE,
    );

    Ok(())
}

/// Render a grid in each section (top-left, top-right, bottom-left) (x,y,z)
pub async fn render_grid(line_count: usize) -> Result<()> {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Draw for first section (top-left)
    for i in 1..line_count {
        let x = (screen_width / 2.0) / (line_count as f32) * (i as f32);
        let y = (screen_height / 2.0) / (line_count as f32) * (i as f32);

        // Vertical lines
        draw_line(x, 0.0, x, screen_height / 2.0, 1.0, RED);
        // Horizontal lines
        draw_line(0.0, y, screen_width / 2.0, y, 1.0, RED);
    }

    // Draw for second section (top-right)
    for i in 1..line_count {
        let x = (screen_width / 2.0) + (screen_width / 2.0) / (line_count as f32) * (i as f32);
        let y = (screen_height / 2.0) / (line_count as f32) * (i as f32);
        // Vertical lines
        draw_line(x, 0.0, x, screen_height / 2.0, 1.0, RED);
        // Horizontal lines
        draw_line(screen_width / 2.0, y, screen_width, y, 1.0, RED);
    }

    // Draw for third section (bottom-left)
    for i in 1..line_count {
        let x = (screen_width / 2.0) / (line_count as f32) * (i as f32);
        let y = (screen_height / 2.0) + (screen_height / 2.0) / (line_count as f32) * (i as f32);
        // Vertical lines
        draw_line(x, screen_height / 2.0, x, screen_height, 1.0, RED);
        // Horizontal lines
        draw_line(0.0, y, screen_width / 2.0, y, 1.0, RED);
    }

    Ok(())
}

#[allow(unused)]
pub async fn render_missile(x: f64, y: f64, z: f64) -> Result<()> {
    Ok(())
}

// /// Create a 2D cameras for x, y, z top-down views
// pub async fn create_cameras() -> Result<Vec<Camera2D>> {
//     let screen_width = screen_width();
//     let screen_height = screen_height();
//     let half_width = screen_width / 2.0;
//     let half_height = screen_height / 2.0;
//
//     let camera_x = Camera2D {
//         target: vec2(0.0, 0.0),
//         zoom: vec2(1.0 / half_width, -1.0 / half_height),
//         ..Default::default()
//     };
//
//     let camera_y = Camera2D {
//         target: vec2(0.0, 0.0),
//         zoom: vec2(1.0 / half_width, -1.0 / half_height),
//         ..Default::default()
//     };
//
//     let camera_z = Camera2D {
//         target: vec2(0.0, 0.0),
//         zoom: vec2(1.0 / half_width, -1.0 / half_height),
//         ..Default::default()
//     };
//
//     Ok(vec![camera_x, camera_y, camera_z])
// }
