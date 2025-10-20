use missile_sim::prelude::*;
use raylib::ffi::rlSetClipPlanes;
use raylib::prelude::*;
use std::collections::VecDeque;

const GRID_SIZE: i32 = 5000;
const GRID_SPACING: f32 = 100.0;

fn main() {
    let steps_per_frame = 120;

    let mut camera_yaw = 0.0;
    let mut camera_pitch = 0.5;
    let mut camera_distance = 500.0;

    // Window setup
    let (mut rl, thread) = init()
        .size(1280, 900)
        .title("DEBUG WINDOW - Missile Guidance Simulation")
        .msaa_4x()
        .build();

    rl.set_target_fps(60);

    // Load preset scenarios
    let scenarios = load_preset_scenarios();
    let mut current_scenario_index = 0;

    // Create engine
    let mut engine = (&scenarios[current_scenario_index]).to_engine();
    let guidance = PureProportionalNavigation;
    let mut metrics = SimulationMetrics::new();

    // Camera setup
    let mut camera = Camera3D::perspective(
        Vector3::new(100.0, 100.0, 100.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        72.0,
    );

    // Trails
    let mut missile_trail: VecDeque<Vector3> = VecDeque::new();
    let mut target_trail: VecDeque<Vector3> = VecDeque::new();

    // Start paused
    let mut paused = true;
    let mut show_trails = true;
    let mut show_vectors = true;
    let mut show_grid = true;

    //Main loop
    while !rl.window_should_close() {
        // INPUT HANDLING

        // Scenario switching
        for (i, key) in [
            KeyboardKey::KEY_ONE,
            KeyboardKey::KEY_TWO,
            KeyboardKey::KEY_THREE,
            KeyboardKey::KEY_FOUR,
            KeyboardKey::KEY_FIVE,
            KeyboardKey::KEY_SIX,
            KeyboardKey::KEY_SEVEN,
            KeyboardKey::KEY_EIGHT,
        ]
        .iter()
        .enumerate()
        {
            if rl.is_key_pressed(*key) {
                current_scenario_index = i.min(scenarios.len() - 1);
                engine = (&scenarios[current_scenario_index]).to_engine();
                metrics = SimulationMetrics::new();
                missile_trail.clear();
                target_trail.clear();
                paused = true;
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            engine = (&scenarios[current_scenario_index]).to_engine();
            metrics = SimulationMetrics::new();
            missile_trail.clear();
            target_trail.clear();
            paused = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_T) {
            show_trails = !show_trails;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_V) {
            show_vectors = !show_vectors;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            show_grid = !show_grid;
        }

        // SIMULATION

        if !paused {
            for _ in 0..steps_per_frame {
                engine.step(&guidance, &mut metrics);
            }

            // Update trails
            let m_pos = engine.missile.state.position;
            let t_pos = engine.target.state.position;

            // Collect trail points
            missile_trail.push_back(Vector3::new(m_pos.x as f32, m_pos.y as f32, m_pos.z as f32));
            target_trail.push_back(Vector3::new(t_pos.x as f32, t_pos.y as f32, t_pos.z as f32));
        }

        // CAMERA

        let m_pos = engine.missile.state.position;
        camera.target = Vector3::new(m_pos.x as f32, m_pos.y as f32, m_pos.z as f32);

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let delta = rl.get_mouse_delta();
            camera_yaw -= delta.x * 0.005;
            camera_pitch += delta.y * 0.005;
            camera_pitch = camera_pitch.clamp(-1.5, 1.5);
        }

        camera_distance -= rl.get_mouse_wheel_move() * 50.0;
        camera_distance = camera_distance.clamp(100.0, 5000.0);

        let cam_x = m_pos.x as f32 + camera_distance * camera_yaw.cos() * camera_pitch.cos();
        let cam_y = m_pos.y as f32 + camera_distance * camera_pitch.sin();
        let cam_z = m_pos.z as f32 + camera_distance * camera_yaw.sin() * camera_pitch.cos();
        camera.position = Vector3::new(cam_x, cam_y, cam_z);

        // RENDERING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::SKYBLUE);

        {
            let mut d3d = d.begin_mode3D(camera);

            // Set clipping planes
            unsafe {
                rlSetClipPlanes(0.1, 100000.0);
            }

            // Draw the thin reference grid
            d3d.draw_grid(GRID_SIZE * 2, GRID_SPACING);

            d3d.draw_plane(Vector3::new(0.0, -100.0, 0.0), Vector2::new(100000.0, 100000.0), Color::LIGHTGRAY);

            // Draw thick X/Z grid lines
            for i in -GRID_SIZE..=GRID_SIZE {
                let pos = i as f32 * GRID_SPACING;

                // X lines along Z
                d3d.draw_cube(
                    &Vector3::new(pos, 0.0, 0.0),
                    1.0,
                    0.1,
                    (GRID_SIZE * 2) as f32 * GRID_SPACING,
                    Color::BLACK,
                );

                // Z lines along X
                d3d.draw_cube(
                    &Vector3::new(0.0, 0.0, pos),
                    (GRID_SIZE * 2) as f32 * GRID_SPACING,
                    0.1,
                    1.0,
                    Color::BLACK,
                );
            }

            // X axis
            d3d.draw_cube(&Vector3::new(0.0, 0.0, 0.0), 5000.0, 5.0, 5.0, Color::GREEN);

            // Y axis
            d3d.draw_cube(&Vector3::new(0.0, 0.0, 0.0), 5.0, 5000.0, 5.0, Color::RED);

            // Z axis
            d3d.draw_cube(&Vector3::new(0.0, 0.0, 0.0), 5.0, 5.0, 5000.0, Color::BLUE);

            let t_pos = engine.target.state.position;
            let m_pos = engine.missile.state.position;
            let t_vec = Vector3::new(t_pos.x as f32, t_pos.y as f32, t_pos.z as f32);
            let m_vec = Vector3::new(m_pos.x as f32, m_pos.y as f32, m_pos.z as f32);

            // Trails
            if show_trails {
                // Draw every 10 units
                for i in (0..missile_trail.len()).step_by(10) {
                    d3d.draw_sphere(missile_trail[i], 5.0, Color::DARKRED);
                }

                // Draw every 10 units
                for i in (0..target_trail.len()).step_by(10) {
                    d3d.draw_sphere(target_trail[i], 5.0, Color::DARKBLUE);
                }
            }

            // Connection line
            d3d.draw_capsule(m_vec, t_vec, 1.0, 32, 64, Color::GHOSTWHITE);

            // Velocity vectors
            if show_vectors {
                let m_vel = engine.missile.state.velocity;
                let t_vel = engine.target.state.velocity;

                let m_vel_end = Vector3::new(
                    (m_pos.x + m_vel.x * 0.2) as f32,
                    (m_pos.y + m_vel.y * 0.2) as f32,
                    (m_pos.z + m_vel.z * 0.2) as f32,
                );
                let t_vel_end = Vector3::new(
                    (t_pos.x + t_vel.x * 0.2) as f32,
                    (t_pos.y + t_vel.y * 0.2) as f32,
                    (t_pos.z + t_vel.z * 0.2) as f32,
                );

                d3d.draw_line_3D(m_vec, m_vel_end, Color::ORANGE);
                d3d.draw_line_3D(t_vec, t_vel_end, Color::CYAN);
            }

            // Draw missile
            let m_pos = engine.missile.state.position;
            d3d.draw_sphere(
                &Vector3::new(m_pos.x as f32, m_pos.y as f32, m_pos.z as f32),
                15.0,
                Color::RED,
            );

            // Draw target
            let t_pos = engine.target.state.position;
            d3d.draw_sphere(
                &Vector3::new(t_pos.x as f32, t_pos.y as f32, t_pos.z as f32),
                25.0,
                Color::BLUE,
            );
        }

        // UI OVERLAY
        draw_ui_panel(
            &mut d,
            &engine,
            &metrics,
            &scenarios[current_scenario_index],
            paused,
            camera_distance,
        );
    }
}

fn draw_ui_panel(
    d: &mut RaylibDrawHandle,
    engine: &SimulationEngine,
    metrics: &SimulationMetrics,
    scenario: &Scenario,
    paused: bool,
    cam_distance: f32,
) {
    let panel_x = d.get_screen_width() - 400;
    let panel_y = 10;
    let panel_w = 390;
    let panel_h = 400;

    // Semi-transparent panel
    d.draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::new(0, 0, 0, 200));
    d.draw_rectangle_lines(
        panel_x,
        panel_y,
        panel_w,
        panel_h,
        Color::new(0, 255, 0, 255),
    );

    let mut y = panel_y + 15;
    let line_h = 24;
    let x = panel_x + 15;

    // Title
    d.draw_text("=== METRICS ===", x, y, 20, Color::GREEN);
    y += line_h + 8;

    // Scenario
    d.draw_text(
        &format!("Scenario: {}", scenario.name),
        x,
        y,
        18,
        Color::WHITE,
    );
    y += line_h;

    // Time
    let time_color = if paused {
        Color::YELLOW
    } else {
        Color::LIGHTGRAY
    };
    d.draw_text(&format!("Time: {:.2} s", engine.time), x, y, 18, time_color);
    if paused {
        d.draw_text("[PAUSED]", x + 180, y, 18, Color::YELLOW);
    }
    y += line_h + 5;

    // Separation
    let distance = (engine.missile.state.position - engine.target.state.position).norm();
    d.draw_text(
        &format!("Separation:  {:.1} m", distance),
        x,
        y,
        18,
        Color::ORANGE,
    );
    y += line_h;

    // Closing velocity - always show, use 0.0 if no data
    let closing_speed = metrics.closing_speed_history.last().copied().unwrap_or(0.0);
    d.draw_text(
        &format!("Closing V:   {:.1} m/s", closing_speed),
        x,
        y,
        18,
        Color::CYAN,
    );
    y += line_h;

    // Time to impact - always show
    let tti = if closing_speed > 0.0 && distance > 0.0 {
        distance / closing_speed
    } else {
        0.0
    };
    d.draw_text(&format!("Impact Est:  {:.2} s", tti), x, y, 18, Color::LIME);
    y += line_h + 5;

    // Missile speed
    let m_speed = engine.missile.state.velocity.norm();
    d.draw_text(
        &format!("Missile Spd: {:.0} m/s", m_speed),
        x,
        y,
        18,
        Color::RED,
    );
    y += line_h;

    // Target speed
    let t_speed = engine.target.state.velocity.norm();
    d.draw_text(
        &format!("Target Spd:  {:.0} m/s", t_speed),
        x,
        y,
        18,
        Color::SKYBLUE,
    );
    y += line_h + 5;

    // Acceleration - always show
    let accel = metrics.acceleration_history.last().copied().unwrap_or(0.0);
    let g_force = accel / 9.81;
    d.draw_text(
        &format!("Accel:       {:.1} G", g_force),
        x,
        y,
        18,
        Color::YELLOW,
    );
    y += line_h;

    // LOS rate - always show
    let los_rate = metrics.los_rate_history.last().copied().unwrap_or(0.0);
    d.draw_text(
        &format!("LOS Rate:    {:.4} rad/s", los_rate),
        x,
        y,
        18,
        Color::MAGENTA,
    );
    y += line_h + 10;

    // Separator
    d.draw_text("=== INFO ===", x, y, 18, Color::GREEN);
    y += line_h;

    // Camera distance
    d.draw_text(
        &format!("Cam Dist: {:.0} m", cam_distance),
        x,
        y,
        16,
        Color::DARKGRAY,
    );
    y += line_h;

    // FPS
    d.draw_fps(x, y);
}
