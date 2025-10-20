use missile_sim::prelude::*;
use raylib::ffi::rlSetClipPlanes;
use raylib::prelude::*;
use std::collections::VecDeque;

const GRID_SIZE: i32 = 2500;
const GRID_SPACING: f32 = 1000.0;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CameraMode {
    Missile,
    Target,
}

impl CameraMode {
    fn next(&self) -> Self {
        match self {
            CameraMode::Missile => CameraMode::Target,
            CameraMode::Target => CameraMode::Missile,
        }
    }

    fn name(&self) -> &str {
        match self {
            CameraMode::Missile => "MISSILE",
            CameraMode::Target => "TARGET",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GuidanceType {
    PurePN,
    TruePN,
}

impl GuidanceType {
    fn next(&self) -> Self {
        match self {
            GuidanceType::PurePN => GuidanceType::TruePN,
            GuidanceType::TruePN => GuidanceType::PurePN,
        }
    }

    fn name(&self) -> &str {
        match self {
            GuidanceType::PurePN => "Pure PN",
            GuidanceType::TruePN => "True PN",
        }
    }
}

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
        .vsync()
        .undecorated()
        .build();

    rl.set_target_fps(60);

    // Load preset scenarios
    let scenarios = load_preset_scenarios();
    let mut current_scenario_index = 0;

    // Create engine
    let mut engine = (&scenarios[current_scenario_index]).to_engine();
    let mut current_guidance = GuidanceType::PurePN;
    let mut metrics = SimulationMetrics::new();

    // Camera setup
    let mut camera = Camera3D::perspective(
        Vector3::new(100.0, 100.0, 100.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        72.0,
    );

    let mut camera_mode = CameraMode::Missile;

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
            KeyboardKey::KEY_NINE,
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

        // Camera mode switching (C key)
        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            camera_mode = camera_mode.next();
        }

        // Guidance law switching (L key)
        if rl.is_key_pressed(KeyboardKey::KEY_L) {
            current_guidance = current_guidance.next();
            // Reset simulation with new guidance law
            engine = (&scenarios[current_scenario_index]).to_engine();
            metrics = SimulationMetrics::new();
            missile_trail.clear();
            target_trail.clear();
            paused = true;
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
            // Use the appropriate guidance law
            let guidance: &dyn GuidanceLaw = match current_guidance {
                GuidanceType::PurePN => &PureProportionalNavigation,
                GuidanceType::TruePN => &TrueProportionalNavigation,
            };

            for _ in 0..steps_per_frame {
                engine.step(guidance, &mut metrics);
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
        let t_pos = engine.target.state.position;

        // Calculate camera target based on mode
        let camera_target_pos = match camera_mode {
            CameraMode::Missile => m_pos,
            CameraMode::Target => t_pos,
        };

        camera.target = Vector3::new(
            camera_target_pos.x as f32,
            camera_target_pos.y as f32,
            camera_target_pos.z as f32,
        );

        // Mouse camera control
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let delta = rl.get_mouse_delta();
            camera_yaw -= delta.x * 0.005;
            camera_pitch += delta.y * 0.005;
            camera_pitch = camera_pitch.clamp(-1.5, 1.5);
        }

        camera_distance -= rl.get_mouse_wheel_move() * 50.0;
        camera_distance = camera_distance.clamp(100.0, 5000.0);

        let cam_x =
            camera_target_pos.x as f32 + camera_distance * camera_yaw.cos() * camera_pitch.cos();
        let cam_y = camera_target_pos.y as f32 + camera_distance * camera_pitch.sin();
        let cam_z =
            camera_target_pos.z as f32 + camera_distance * camera_yaw.sin() * camera_pitch.cos();

        camera.position = Vector3::new(cam_x, cam_y, cam_z);

        // RENDERING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTSKYBLUE);

        {
            let mut d3d = d.begin_mode3D(camera);

            // Set clipping planes
            unsafe {
                rlSetClipPlanes(0.1, 100000.0);
            }

            // Draw the thin reference grid
            d3d.draw_grid(GRID_SIZE * 2, GRID_SPACING);

            d3d.draw_plane(
                Vector3::new(0.0, -100.0, 0.0),
                Vector2::new(100000.0, 100000.0),
                Color::FORESTGREEN,
            );

            // Draw thick X/Z grid lines
            for i in -GRID_SIZE..=GRID_SIZE {
                let pos = i as f32 * GRID_SPACING;

                // X lines along Z
                d3d.draw_cube(
                    &Vector3::new(pos, 0.0, 0.0),
                    1.0,
                    1.0,
                    (GRID_SIZE * 2) as f32 * GRID_SPACING,
                    Color::GHOSTWHITE,
                );

                // Z lines along X
                d3d.draw_cube(
                    &Vector3::new(0.0, 0.0, pos),
                    (GRID_SIZE * 2) as f32 * GRID_SPACING,
                    1.0,
                    1.0,
                    Color::GHOSTWHITE,
                );
            }

            // X axis
            d3d.draw_cube(&Vector3::new(0.0, 0.0, 0.0), 5000.0, 5.0, 5.0, Color::RED);

            // Y axis
            d3d.draw_cube(&Vector3::new(0.0, 0.0, 0.0), 5.0, 5000.0, 5.0, Color::GREEN);

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
                    (m_pos.x + m_vel.x * 1.0) as f32,
                    (m_pos.y + m_vel.y * 1.0) as f32,
                    (m_pos.z + m_vel.z * 1.0) as f32,
                );
                let t_vel_end = Vector3::new(
                    (t_pos.x + t_vel.x * 1.0) as f32,
                    (t_pos.y + t_vel.y * 1.0) as f32,
                    (t_pos.z + t_vel.z * 1.0) as f32,
                );

                d3d.draw_capsule_wires(m_vec, m_vel_end, 2.0, 24, 32, Color::BLACK);
                d3d.draw_capsule_wires(t_vec, t_vel_end, 2.0, 24, 32, Color::BLACK);
            }

            // Draw missile
            //let m_pos = engine.missile.state.position;
            //d3d.draw_sphere(
            //    &Vector3::new(m_pos.x as f32, m_pos.y as f32, m_pos.z as f32),
            //    15.0,
            //    Color::RED,
            //);

            // Draw target
            //let t_pos = engine.target.state.position;
            //d3d.draw_sphere(
            //    &Vector3::new(t_pos.x as f32, t_pos.y as f32, t_pos.z as f32),
            //    15.0,
            //    Color::BLUE,
            //);

            // Draw missile as oriented arrow
            draw_arrow(
                &mut d3d,
                m_vec,
                engine.missile.state.velocity,
                40.0,
                Color::RED,
            );

            // Draw target as oriented arrow
            draw_arrow(
                &mut d3d,
                t_vec,
                engine.target.state.velocity,
                40.0,
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
            camera_mode,
            current_guidance,
        );

        d.draw_text(
            "RIGHT MOUSE: Rotate | WHEEL: Zoom | C: Camera Mode",
            10,
            d.get_screen_height() - 90,
            16,
            Color::DARKBLUE,
        );
        d.draw_text(
            "T: Trails | V: Vectors | G: Grid",
            10,
            d.get_screen_height() - 70,
            16,
            Color::DARKBLUE,
        );
        d.draw_text(
            "1-8: Scenario | SPACE: Pause | R: Reset",
            10,
            d.get_screen_height() - 50,
            16,
            Color::DARKBLUE,
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
    camera_mode: CameraMode,
    guidance_type: GuidanceType,
) {
    let panel_x = d.get_screen_width() - 400;
    let panel_y = 10;
    let panel_w = 390;
    let panel_h = 425;

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

    // Closing velocity
    let closing_speed = metrics.closing_speed_history.last().copied().unwrap_or(0.0);
    d.draw_text(
        &format!("Closing V:   {:.1} m/s", closing_speed),
        x,
        y,
        18,
        Color::CYAN,
    );
    y += line_h;

    // Time to impact
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

    // Acceleration
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

    // LOS rate
    let los_rate = metrics.los_rate_history.last().copied().unwrap_or(0.0);
    d.draw_text(
        &format!("LOS Rate:    {:.4} rad/s", los_rate),
        x,
        y,
        18,
        Color::MAGENTA,
    );
    y += line_h + 10;

    // Guidance law
    d.draw_text(
        &format!("Guidance: {}", guidance_type.name()),
        x,
        y,
        16,
        Color::ORANGE,
    );
    y += line_h;

    // Camera mode
    d.draw_text(
        &format!("Cam Mode: {}", camera_mode.name()),
        x,
        y,
        16,
        Color::GOLD,
    );
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

fn draw_arrow(
    d3d: &mut RaylibMode3D<RaylibDrawHandle>,
    position: Vector3,
    velocity: nalgebra::Vector3<f64>,
    size: f32,
    color: Color,
) {
    let vel_norm = velocity.norm();

    if vel_norm < 0.1 {
        d3d.draw_sphere(position, size, color);
        return;
    }

    let dir = velocity / vel_norm;
    let direction = Vector3::new(dir.x as f32, dir.y as f32, dir.z as f32);

    let shaft_length = size * 2.5;
    let shaft_radius = size * 0.2;
    let cone_length = size * 2.0;
    let cone_radius = size * 0.25;

    let shaft_end = Vector3::new(
        position.x + direction.x * shaft_length,
        position.y + direction.y * shaft_length,
        position.z + direction.z * shaft_length,
    );

    let cone_tip = Vector3::new(
        shaft_end.x + direction.x * cone_length,
        shaft_end.y + direction.y * cone_length,
        shaft_end.z + direction.z * cone_length,
    );

    d3d.draw_cylinder_ex(position, shaft_end, shaft_radius, shaft_radius, 8, color);
    d3d.draw_cylinder_ex(shaft_end, cone_tip, cone_radius, 0.1, 8, color);
}
