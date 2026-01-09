use anyhow::Result;
use macroquad::prelude::*;

///
/// Hey its my birthday today...
///
/// I became 19 today, kiddo!!
///
/// I dont know how to celebrate birthdays anymore, SO I JUST GONNA CODE, LISTEN TO SOME JAZZ.
///
/// 'N' year from now:
/// - I hope you're still coding while you can...maybe
/// - I hope you're kinder to yourself
/// - I hope your family crap is finally over

pub async fn render_master() -> Result<()> {
    render_grid(5).await?;
    sections_lines().await?;
    render_plane_label().await?;
    Ok(())
}

/// Split the screen into 6 sections with 3 lines
pub async fn sections_lines() -> Result<()> {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Vertical line
    draw_line(
        screen_width * 0.334,
        0.0,
        screen_width * 0.334,
        screen_height,
        5.0,
        BLUE,
    );

    // Vertical line
    draw_line(
        screen_width * 0.334 * 2.0,
        0.0,
        screen_width * 0.334 * 2.0,
        screen_height,
        5.0,
        BLUE,
    );

    // Horizontal line
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

async fn render_plane_label() -> Result<()> {
    // Missile Labels

    // X-Y Plane
    draw_text("X-Y Plane", 10.0, 20.0, 20.0, WHITE);

    // X-Z Plane
    draw_text(
        "X-Z Plane",
        screen_width() * 0.334 + 10.0,
        20.0,
        20.0,
        WHITE,
    );

    // Z-Y Plane
    draw_text(
        "Z-Y Plane",
        screen_width() * 0.334 * 2.0 + 10.0,
        20.0,
        20.0,
        WHITE,
    );

    // Target Labels

    // X-Y Plane
    draw_text("X-Y Plane", 10.0, screen_height() / 2.0 + 20.0, 20.0, WHITE);

    // X-Z Plane
    draw_text(
        "X-Z Plane",
        screen_width() * 0.334 + 10.0,
        screen_height() / 2.0 + 20.0,
        20.0,
        WHITE,
    );

    // Z-Y Plane
    draw_text(
        "Z-Y Plane",
        screen_width() * 0.334 * 2.0 + 10.0,
        screen_height() / 2.0 + 20.0,
        20.0,
        WHITE,
    );

    Ok(())
}

/// Render a global grid aligned across all 6 sections
/// Missile: Top-left (XY Plane), Top-between (XZ Plane), Top-right (ZY Plane)
/// Target: Bottom-left, Bottom-between, Bottom-right (same planes as above)
pub async fn render_grid(line_count: usize) -> Result<()> {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Render grid for top left section (missile x-top)
    for i in 0..=line_count {
        let x = i as f32 * (screen_width * 0.334) / line_count as f32;
        let y = i as f32 * (screen_height / 2.0) / line_count as f32;

        // Vertical lines
        draw_line(x, 0.0, x, screen_height / 2.0, 1.0, RED);

        // Horizontal lines
        draw_line(0.0, y, screen_width * 0.334, y, 1.0, RED);
    }

    // Render grid for top middle section (missile y-top)
    for i in 0..=line_count {
        let x = i as f32 * (screen_width * 0.334) / line_count as f32 + screen_width * 0.334;
        let y = i as f32 * (screen_height / 2.0) / line_count as f32;

        // Vertical lines
        draw_line(x, 0.0, x, screen_height / 2.0, 1.0, RED);

        // Horizontal lines
        draw_line(
            screen_width * 0.334,
            y,
            screen_width * 0.334 * 2.0,
            y,
            1.0,
            RED,
        );
    }

    // Render grid for top right section (missile z-top)
    for i in 0..=line_count {
        let x = i as f32 * (screen_width * 0.334) / line_count as f32 + screen_width * 0.334 * 2.0;
        let y = i as f32 * (screen_height / 2.0) / line_count as f32;

        // Vertical lines
        draw_line(x, 0.0, x, screen_height / 2.0, 1.0, RED);
        // Horizontal lines
        draw_line(screen_width * 0.334 * 2.0, y, screen_width, y, 1.0, RED);
    }

    // Render grid for bottom left section (target x-top)
    for i in 0..=line_count {
        let x = i as f32 * (screen_width * 0.334) / line_count as f32;
        let y = i as f32 * (screen_height / 2.0) / line_count as f32 + screen_height / 2.0;
        // Vertical lines
        draw_line(x, screen_height / 2.0, x, screen_height, 1.0, RED);
        // Horizontal lines
        draw_line(0.0, y, screen_width * 0.334, y, 1.0, RED);
    }

    // Render grid for bottom middle section (target y-top)
    for i in 0..=line_count {
        let x = i as f32 * (screen_width * 0.334) / line_count as f32 + screen_width * 0.334;
        let y = i as f32 * (screen_height / 2.0) / line_count as f32 + screen_height / 2.0;
        // Vertical lines
        draw_line(x, screen_height / 2.0, x, screen_height, 1.0, RED);
        // Horizontal lines
        draw_line(
            screen_width * 0.334,
            y,
            screen_width * 0.334 * 2.0,
            y,
            1.0,
            RED,
        );
    }

    // Render grid for bottom right section (target z-top)
    for i in 0..=line_count {
        let x = i as f32 * (screen_width * 0.334) / line_count as f32 + screen_width * 0.334 * 2.0;
        let y = i as f32 * (screen_height / 2.0) / line_count as f32 + screen_height / 2.0;
        // Vertical lines
        draw_line(x, screen_height / 2.0, x, screen_height, 1.0, RED);
        // Horizontal lines
        draw_line(screen_width * 0.334 * 2.0, y, screen_width, y, 1.0, RED);
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
