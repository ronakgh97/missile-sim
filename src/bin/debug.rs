use raylib::prelude::*;

fn main() {
    let grid_size = 5000;
    let spacing = 50.0;
    let line_thickness = 1.0;

    let axis_length = 5000.0;
    let axis_thickness = 1.0;

    let (mut rl, thread) = init().size(1280, 900).title("DEBUG").build();

    rl.set_target_fps(30);

    let mut camera = Camera3D::perspective(
        Vector3::new(50.0, 50.0, 50.0), // position
        Vector3::new(0.0, 0.0, 0.0),    // target
        Vector3::new(0.0, 1.0, 0.0),    // up
        45.0,
    );

    while !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_ORBITAL);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTGRAY);

        {
            d.draw_fps(10, 10);

            let mut d3d = d.begin_mode3D(camera);

            // Draw the thin reference grid
            d3d.draw_grid(grid_size * 2, spacing);

            // Draw thick X/Z grid lines using thin cubes
            for i in -grid_size..=grid_size {
                let pos = i as f32 * spacing;

                // X lines along Z
                d3d.draw_cube(
                    &Vector3::new(pos, 0.0, 0.0),
                    line_thickness,
                    0.1,
                    (grid_size * 2) as f32 * spacing,
                    Color::BLACK,
                );

                // Z lines along X
                d3d.draw_cube(
                    &Vector3::new(0.0, 0.0, pos),
                    (grid_size * 2) as f32 * spacing,
                    0.1,
                    line_thickness,
                    Color::BLACK,
                );
            }

            // Y axis (up)
            d3d.draw_cube(
                &Vector3::new(0.0, 0.0, 0.0),
                axis_thickness,
                axis_length,
                axis_thickness,
                Color::RED,
            );

            // X axis
            d3d.draw_cube(
                &Vector3::new(0.0, 0.0, 0.0),
                axis_length,
                axis_thickness,
                axis_thickness,
                Color::GREEN,
            );

            // Z axis
            d3d.draw_cube(
                &Vector3::new(0.0, 0.0, 0.0),
                axis_thickness,
                axis_thickness,
                axis_length,
                Color::BLUE,
            );
        }
    }
}
