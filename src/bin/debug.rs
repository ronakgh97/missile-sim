use missile_sim::prelude::*;
use raylib::ffi::rlSetClipPlanes;
use raylib::prelude::*;
use std::collections::VecDeque;

// Grid settings
const GRID_SIZE: i32 = 2500;
const GRID_SPACING: f32 = 1000.0;

// Simulation settings
const STEPS_PER_FRAME: usize = 144;
const TRAIL_RENDER_STEP: usize = 10;
// Target control constants
const TARGET_CONTROL_ACCEL: f64 = 750.0; // m/s^2 acceleration when key held
const TARGET_MAX_SPEED: f64 = 2500.0; // m/s max speed for target when player-controlled

// Camera settings
const CAM_SENSITIVITY: f32 = 0.005;
const CAM_ZOOM_SPEED: f32 = 50.0;
const CAM_PITCH_MIN: f32 = -1.5;
const CAM_PITCH_MAX: f32 = 1.5;
const CAM_DISTANCE_MIN: f32 = 100.0;
const CAM_DISTANCE_MAX: f32 = 5000.0;
const CAM_DEFAULT_DISTANCE: f32 = 500.0;
const CAM_DEFAULT_YAW: f32 = 0.0;
const CAM_DEFAULT_PITCH: f32 = 0.5;

// Visual settings
const MISSILE_SIZE: f32 = 40.0;
const TARGET_SIZE: f32 = 40.0;
const TRAIL_SPHERE_SIZE: f32 = 10.0;
const VELOCITY_VECTOR_SCALE: f32 = 1.0;

// Window settings
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 900;
const TARGET_FPS: u32 = 60;

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
    AugmentedPN,
    PurePursuit,
    DeviatedPursuit,
    LeadPursuit,
}

impl GuidanceType {
    fn next(&self) -> Self {
        match self {
            GuidanceType::PurePN => GuidanceType::TruePN,
            GuidanceType::TruePN => GuidanceType::AugmentedPN,
            GuidanceType::AugmentedPN => GuidanceType::PurePursuit,
            GuidanceType::PurePursuit => GuidanceType::DeviatedPursuit,
            GuidanceType::DeviatedPursuit => GuidanceType::LeadPursuit,
            GuidanceType::LeadPursuit => GuidanceType::PurePN,
        }
    }

    fn name(&self) -> &str {
        match self {
            GuidanceType::PurePN => "Pure PN",
            GuidanceType::TruePN => "True PN",
            GuidanceType::AugmentedPN => "Augmented PN",
            GuidanceType::PurePursuit => "Pure Pursuit",
            GuidanceType::DeviatedPursuit => "Deviated Pursuit",
            GuidanceType::LeadPursuit => "Lead Pursuit",
        }
    }

    fn as_guidance_law(&self) -> Box<dyn GuidanceLaw> {
        match self {
            GuidanceType::PurePN => Box::new(PureProportionalNavigation),
            GuidanceType::TruePN => Box::new(TrueProportionalNavigation),
            GuidanceType::AugmentedPN => Box::new(AugmentedProportionalNavigation::default()),
            GuidanceType::PurePursuit => Box::new(PurePursuit),
            GuidanceType::DeviatedPursuit => Box::new(DeviatedPursuit),
            GuidanceType::LeadPursuit => Box::new(LeadPursuit::default()),
        }
    }
}

struct AppState {
    engine: SimulationEngine,
    metrics: SimulationMetrics,
    current_scenario_index: usize,
    current_guidance: GuidanceType,
    missile_trail: VecDeque<Vector3>,
    target_trail: VecDeque<Vector3>,
    paused: bool,
    target_control_mode: bool,
    target_input_accel: nalgebra::Vector3<f64>,
}

impl AppState {
    fn new(scenario: &Scenario) -> Self {
        Self {
            engine: scenario.to_engine(),
            metrics: SimulationMetrics::new(),
            current_scenario_index: 0,
            current_guidance: GuidanceType::PurePN,
            missile_trail: VecDeque::new(),
            target_trail: VecDeque::new(),
            paused: true,
            target_control_mode: false,
            target_input_accel: nalgebra::Vector3::zeros(),
        }
    }

    fn reset(&mut self, scenario: &Scenario) {
        self.engine = scenario.to_engine();
        self.metrics = SimulationMetrics::new();
        self.missile_trail.clear();
        self.target_trail.clear();
        self.paused = true;
        self.target_control_mode = false;
        self.target_input_accel = nalgebra::Vector3::zeros();
    }

    fn update_trails(&mut self) {
        self.missile_trail
            .push_back(to_vec3(self.engine.missile.state.position));
        self.target_trail
            .push_back(to_vec3(self.engine.target.state.position));
    }
}

struct CameraState {
    camera: Camera3D,
    mode: CameraMode,
    yaw: f32,
    pitch: f32,
    distance: f32,
}

impl CameraState {
    fn new() -> Self {
        Self {
            camera: Camera3D::perspective(
                Vector3::new(100.0, 100.0, 100.0),
                Vector3::zero(),
                Vector3::up(),
                110.0,
            ),
            mode: CameraMode::Missile,
            yaw: CAM_DEFAULT_YAW,
            pitch: CAM_DEFAULT_PITCH,
            distance: CAM_DEFAULT_DISTANCE,
        }
    }

    fn update(&mut self, target_pos: nalgebra::Vector3<f64>) {
        self.camera.target = to_vec3(target_pos);

        self.camera.position.x =
            self.camera.target.x + self.distance * self.yaw.cos() * self.pitch.cos();
        self.camera.position.y = self.camera.target.y + self.distance * self.pitch.sin();
        self.camera.position.z =
            self.camera.target.z + self.distance * self.yaw.sin() * self.pitch.cos();
    }

    fn handle_mouse_input(&mut self, rl: &RaylibHandle) {
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let delta = rl.get_mouse_delta();
            self.yaw -= delta.x * CAM_SENSITIVITY;
            self.pitch += delta.y * CAM_SENSITIVITY;
            self.pitch = self.pitch.clamp(CAM_PITCH_MIN, CAM_PITCH_MAX);
        }

        self.distance -= rl.get_mouse_wheel_move() * CAM_ZOOM_SPEED;
        self.distance = self.distance.clamp(CAM_DISTANCE_MIN, CAM_DISTANCE_MAX);
    }
}

struct VisualToggles {
    show_vectors: bool,
}

impl Default for VisualToggles {
    fn default() -> Self {
        Self { show_vectors: true }
    }
}

// HELPER FUNCTIONS
fn to_vec3(v: nalgebra::Vector3<f64>) -> Vector3 {
    Vector3::new(v.x as f32, v.y as f32, v.z as f32)
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
        d3d.draw_sphere(&position, size, color);
        return;
    }

    let dir = velocity / vel_norm;
    let direction = Vector3::new(dir.x as f32, dir.y as f32, dir.z as f32);

    let shaft_length = size * 2.5;
    let shaft_radius = size * 0.2;
    let cone_length = size * 2.0;
    let cone_radius = size * 0.275;

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

    d3d.draw_cylinder_ex(&position, &shaft_end, shaft_radius, shaft_radius, 8, color);
    d3d.draw_cylinder_ex(&shaft_end, &cone_tip, cone_radius, 0.15, 8, color);
}

// INPUT HANDLING
fn handle_scenario_switching(rl: &RaylibHandle, app_state: &mut AppState, scenarios: &[Scenario]) {
    let scenario_keys = [
        KeyboardKey::KEY_ONE,
        KeyboardKey::KEY_TWO,
        KeyboardKey::KEY_THREE,
        KeyboardKey::KEY_FOUR,
        KeyboardKey::KEY_FIVE,
        KeyboardKey::KEY_SIX,
        KeyboardKey::KEY_SEVEN,
        KeyboardKey::KEY_EIGHT,
        KeyboardKey::KEY_NINE,
        KeyboardKey::KEY_ZERO,
    ];

    for (i, &key) in scenario_keys.iter().enumerate() {
        if rl.is_key_pressed(key) && i < scenarios.len() {
            app_state.current_scenario_index = i;
            app_state.reset(&scenarios[i]);
            break;
        }
    }
}

fn handle_target_control(rl: &RaylibHandle, app_state: &mut AppState) {
    // Frame-rate independent.
    let mut dir = nalgebra::Vector3::zeros();

    if rl.is_key_down(KeyboardKey::KEY_W) {
        dir.x += 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        dir.x -= 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        dir.z += 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        dir.z -= 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_E) {
        dir.y += 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_Q) {
        dir.y -= 1.0;
    }

    if dir.norm() > 0.0 {
        app_state.target_input_accel = dir.normalize() * TARGET_CONTROL_ACCEL;
    } else {
        app_state.target_input_accel = nalgebra::Vector3::zeros();
    }
}

fn handle_input(
    rl: &RaylibHandle,
    app_state: &mut AppState,
    camera_state: &mut CameraState,
    toggles: &mut VisualToggles,
    scenarios: &[Scenario],
) {
    // Scenario switching
    handle_scenario_switching(rl, app_state, scenarios);

    // Camera mode
    if rl.is_key_pressed(KeyboardKey::KEY_C) {
        camera_state.mode = camera_state.mode.next();
    }

    // Guidance switching
    if rl.is_key_pressed(KeyboardKey::KEY_L) {
        app_state.current_guidance = app_state.current_guidance.next();
        app_state.reset(&scenarios[app_state.current_scenario_index]);
    }

    // Target control toggle
    if rl.is_key_pressed(KeyboardKey::KEY_T) {
        app_state.target_control_mode = !app_state.target_control_mode;
    }

    // Simulation control
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        app_state.paused = !app_state.paused;
    }

    if rl.is_key_pressed(KeyboardKey::KEY_R) {
        app_state.reset(&scenarios[app_state.current_scenario_index]);
    }

    // Visual toggles
    if rl.is_key_pressed(KeyboardKey::KEY_V) {
        toggles.show_vectors = !toggles.show_vectors;
    }

    // Target control input
    if app_state.target_control_mode && !app_state.paused {
        handle_target_control(rl, app_state);
    }

    // Camera control
    camera_state.handle_mouse_input(rl);
}

// RENDERING
fn render_grid(d3d: &mut RaylibMode3D<RaylibDrawHandle>) {
    // Base grid
    //d3d.draw_grid(GRID_SIZE * 2, GRID_SPACING);

    // Ground plane
    d3d.draw_plane(
        &Vector3::new(0.0, -100.0, 0.0),
        &Vector2::new(100000.0, 100000.0),
        Color::DARKSEAGREEN,
    );

    // Grid lines

    for i in -GRID_SIZE..=GRID_SIZE {
        let pos = i as f32 * GRID_SPACING;

        d3d.draw_cube(
            &Vector3::new(pos, 100.0, 0.0),
            10.0,
            10.0,
            (GRID_SIZE * 2) as f32 * GRID_SPACING,
            Color::GHOSTWHITE,
        );

        d3d.draw_cube(
            &Vector3::new(0.0, 100.0, pos),
            (GRID_SIZE * 2) as f32 * GRID_SPACING,
            10.0,
            10.0,
            Color::GHOSTWHITE,
        );
    }

    // Coordinate axes
    let axis_len = 50000.0;
    d3d.draw_cube(&Vector3::zero(), axis_len, 25.0, 25.0, Color::DARKRED); // X
    d3d.draw_cube(&Vector3::zero(), 25.0, axis_len, 25.0, Color::DARKGREEN); // Y
    d3d.draw_cube(&Vector3::zero(), 25.0, 25.0, axis_len, Color::DARKBLUE); // Z
}

fn render_trails(
    d3d: &mut RaylibMode3D<RaylibDrawHandle>,
    missile_trail: &VecDeque<Vector3>,
    target_trail: &VecDeque<Vector3>,
) {
    for i in (0..missile_trail.len()).step_by(TRAIL_RENDER_STEP) {
        d3d.draw_sphere(&missile_trail[i], TRAIL_SPHERE_SIZE, Color::DARKRED);
    }

    for i in (0..target_trail.len()).step_by(TRAIL_RENDER_STEP) {
        d3d.draw_sphere(&target_trail[i], TRAIL_SPHERE_SIZE, Color::DARKBLUE);
    }
}

fn render_velocity_vectors(
    d3d: &mut RaylibMode3D<RaylibDrawHandle>,
    m_pos: Vector3,
    t_pos: Vector3,
    m_vel: nalgebra::Vector3<f64>,
    t_vel: nalgebra::Vector3<f64>,
) {
    let m_vel_end = Vector3::new(
        m_pos.x + (m_vel.x as f32 * VELOCITY_VECTOR_SCALE),
        m_pos.y + (m_vel.y as f32 * VELOCITY_VECTOR_SCALE),
        m_pos.z + (m_vel.z as f32 * VELOCITY_VECTOR_SCALE),
    );

    let t_vel_end = Vector3::new(
        t_pos.x + (t_vel.x as f32 * VELOCITY_VECTOR_SCALE),
        t_pos.y + (t_vel.y as f32 * VELOCITY_VECTOR_SCALE),
        t_pos.z + (t_vel.z as f32 * VELOCITY_VECTOR_SCALE),
    );

    d3d.draw_capsule_wires(&m_pos, &m_vel_end, 2.0, 24, 32, Color::BLACK);
    d3d.draw_capsule_wires(&t_pos, &t_vel_end, 2.0, 24, 32, Color::BLACK);
}

fn render_3d_scene(
    d3d: &mut RaylibMode3D<RaylibDrawHandle>,
    app_state: &AppState,
    toggles: &VisualToggles,
) {
    unsafe {
        rlSetClipPlanes(0.1, 100000.0);
    }

    let m_vec = to_vec3(app_state.engine.missile.state.position);
    let t_vec = to_vec3(app_state.engine.target.state.position);

    // Trails
    render_trails(d3d, &app_state.missile_trail, &app_state.target_trail);

    // Line of sight
    d3d.draw_capsule(&m_vec, &t_vec, 5.0, 32, 64, Color::DARKBROWN);

    // Velocity vectors
    if toggles.show_vectors {
        render_velocity_vectors(
            d3d,
            m_vec,
            t_vec,
            app_state.engine.missile.state.velocity,
            app_state.engine.target.state.velocity,
        );
    }

    // Entities
    draw_arrow(
        d3d,
        m_vec,
        app_state.engine.missile.state.velocity,
        MISSILE_SIZE,
        Color::RED,
    );
    draw_arrow(
        d3d,
        t_vec,
        app_state.engine.target.state.velocity,
        TARGET_SIZE,
        Color::BLUE,
    );
}

fn render_ui(
    d: &mut RaylibDrawHandle,
    app_state: &AppState,
    camera_state: &CameraState,
    scenario: &Scenario,
) {
    draw_ui_panel(
        d,
        &app_state.engine,
        &app_state.metrics,
        scenario,
        app_state.paused,
        camera_state.distance,
        camera_state.mode,
        app_state.current_guidance,
        app_state.target_control_mode,
    );

    // Control hints
    let h = d.get_screen_height();
    d.draw_text(
        "RMB: Rotate | WHEEL: Zoom | C: Camera | L: Guidance | T: Target Ctrl",
        10,
        h - 110,
        16,
        Color::DARKBLUE,
    );
    d.draw_text(
        "WASD: Move Target | Q/E: Alt Target | V: Vectors",
        10,
        h - 90,
        16,
        Color::DARKBLUE,
    );
    d.draw_text(
        "1-9: Scenario | SPACE: Pause | R: Reset",
        10,
        h - 70,
        16,
        Color::DARKBLUE,
    );
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
    target_control_mode: bool,
) {
    let panel_x = d.get_screen_width() - 400;
    let panel_y = 10;
    let panel_w = 390;
    let panel_h = 425;

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

    d.draw_text("METRICS", x, y, 20, Color::GREEN);
    y += line_h + 8;

    d.draw_text(
        &format!("Scenario: {}", scenario.name),
        x,
        y,
        18,
        Color::WHITE,
    );
    y += line_h;

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

    let distance = (engine.missile.state.position - engine.target.state.position).norm();
    d.draw_text(
        &format!("Separation:  {distance:.1} m"),
        x,
        y,
        18,
        Color::ORANGE,
    );
    y += line_h;

    let closing_speed = metrics.closing_speed_history.last().copied().unwrap_or(0.0);
    d.draw_text(
        &format!("Closing V:   {closing_speed:.1} m/s"),
        x,
        y,
        18,
        Color::CYAN,
    );
    y += line_h;

    let time = if closing_speed > 0.0 && distance > 0.0 {
        distance / closing_speed
    } else {
        0.0
    };
    d.draw_text(&format!("Impact Est:  {time:.2} s"), x, y, 18, Color::LIME);
    y += line_h + 5;

    let m_speed = engine.missile.state.velocity.norm();
    d.draw_text(
        &format!("Missile Spd: {m_speed:.0} m/s"),
        x,
        y,
        18,
        Color::RED,
    );
    y += line_h;

    let t_speed = engine.target.state.velocity.norm();
    d.draw_text(
        &format!("Target Spd:  {t_speed:.0} m/s"),
        x,
        y,
        18,
        Color::SKYBLUE,
    );
    y += line_h + 5;

    let accel = metrics.acceleration_history.last().copied().unwrap_or(0.0);
    let g_force = accel / 9.81;
    d.draw_text(
        &format!("Accel:       {g_force:.1} G"),
        x,
        y,
        18,
        Color::YELLOW,
    );
    y += line_h;

    let los_rate = metrics.los_rate_history.last().copied().unwrap_or(0.0);
    d.draw_text(
        &format!("LOS Rate:    {los_rate:.4} rad/s"),
        x,
        y,
        18,
        Color::MAGENTA,
    );
    y += line_h + 10;

    d.draw_text(
        &format!("Guidance: {}", guidance_type.name()),
        x,
        y,
        16,
        Color::ORANGE,
    );
    y += line_h;

    d.draw_text(
        &format!("Cam Mode: {}", camera_mode.name()),
        x,
        y,
        16,
        Color::GOLD,
    );
    y += line_h;

    d.draw_text(
        &format!("Cam Dist: {cam_distance:.0} m"),
        x,
        y,
        16,
        Color::DARKGRAY,
    );
    y += line_h;

    let target_control_color = if target_control_mode {
        Color::GREEN
    } else {
        Color::GRAY
    };
    d.draw_text(
        &format!(
            "Target Ctrl: {}",
            if target_control_mode { "ON" } else { "OFF" }
        ),
        x,
        y,
        16,
        target_control_color,
    );
    y += line_h;

    d.draw_fps(x, y);
}

// MAIN
fn main() {
    let (mut rl, thread) = init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Missile Guidance Simulation")
        .msaa_4x()
        .vsync()
        .undecorated()
        .build();

    rl.set_target_fps(TARGET_FPS);

    let scenarios = load_preset_scenarios();
    let mut app_state = AppState::new(&scenarios[0]);
    let mut camera_state = CameraState::new();
    let mut toggles = VisualToggles::default();

    while !rl.window_should_close() {
        // Input
        handle_input(
            &rl,
            &mut app_state,
            &mut camera_state,
            &mut toggles,
            &scenarios,
        );

        // Simulation
        if !app_state.paused {
            let guidance = app_state.current_guidance.as_guidance_law();
            for _ in 0..STEPS_PER_FRAME {
                // If player is controlling the target, apply per-step acceleration
                if app_state.target_control_mode {
                    // Apply accel = a * dt
                    let a = app_state.target_input_accel;
                    app_state.engine.target.state.velocity += a * app_state.engine.dt;

                    // clamp speed
                    let speed = app_state.engine.target.state.velocity.norm();
                    if speed > TARGET_MAX_SPEED {
                        app_state.engine.target.state.velocity =
                            app_state.engine.target.state.velocity.normalize() * TARGET_MAX_SPEED;
                    }
                }

                app_state
                    .engine
                    .step(guidance.as_ref(), &mut app_state.metrics);
            }
            app_state.update_trails();
        }

        // Camera
        let camera_target = match camera_state.mode {
            CameraMode::Missile => app_state.engine.missile.state.position,
            CameraMode::Target => app_state.engine.target.state.position,
        };
        camera_state.update(camera_target);

        // Rendering
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTSKYBLUE);

        {
            let mut d3d = d.begin_mode3D(camera_state.camera);
            render_3d_scene(&mut d3d, &app_state, &toggles);
            render_grid(&mut d3d);
        }

        render_ui(
            &mut d,
            &app_state,
            &camera_state,
            &scenarios[app_state.current_scenario_index],
        );
    }
}
