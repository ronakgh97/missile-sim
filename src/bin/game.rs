use bevy::prelude::*;
use bevy::window::CursorOptions;
use missile_sim::game_prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ace Strike".into(),
                resolution: (800.0, 600.0).into(),
                cursor_options: CursorOptions {
                    visible: false,
                    grab_mode: bevy::window::CursorGrabMode::Locked,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(MouseControl::default())
        .add_systems(Startup, (setup_world_system, setup_player_system))
        .add_systems(
            Update,
            (
                handle_input_system,
                toggle_cursor_system,
                apply_movement_system,
                camera_follow_system,
                debug_info_system,
            )
                .chain(),
        )
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct MouseControl {
    base_sensitivity: f32,
    max_sensitivity_multiplier: f32,
    invert_y: bool,
}

impl Default for MouseControl {
    fn default() -> Self {
        Self {
            base_sensitivity: 0.0005,
            max_sensitivity_multiplier: 5.0,
            invert_y: false,
        }
    }
}

/// Set up world environment
fn setup_world_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // CAMERA
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 100.0, 200.0).looking_at(Vec3::new(0.0, 50.0, 0.0), Vec3::Y),
        MainCamera,
    ));

    // SUN (Directional Light)
    commands.spawn((
        DirectionalLight {
            illuminance: 25000.0,
            shadows_enabled: true,
            color: Color::srgb(1.0, 0.95, 0.9),
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::PI / 4.0,
            std::f32::consts::PI / 4.0,
            0.0,
        )),
    ));

    // AMBIENT LIGHT
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.7, 0.8, 1.0),
        brightness: 500.0,
        affects_lightmapped_meshes: true,
    });

    // MASSIVE GRID GROUND
    let grid_size = 2500;
    let tile_size = 150.0;
    let total_size = grid_size as f32 * tile_size;

    // Ground material with grid lines
    let ground_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.3, 0.2),
        perceptual_roughness: 0.75,
        metallic: 0.0,
        ..default()
    });

    // Main ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(total_size / 2.0)))),
        MeshMaterial3d(ground_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // GRID LINES
    let grid_material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Vertical grid lines (along Z-axis)
    for i in 0..=grid_size {
        let x = (i as f32 - grid_size as f32 / 2.0) * tile_size;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, total_size))),
            MeshMaterial3d(grid_material.clone()),
            Transform::from_xyz(x, 0.5, 0.0),
        ));
    }

    // Horizontal grid lines (along X-axis)
    for i in 0..=grid_size {
        let z = (i as f32 - grid_size as f32 / 2.0) * tile_size;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(total_size, 1.0, 2.0))),
            MeshMaterial3d(grid_material.clone()),
            Transform::from_xyz(0.0, 0.5, z),
        ));
    }

    println!(
        "Grid size: {}km x {}km",
        total_size / 1000.0,
        total_size / 1000.0
    );
}

/// Setup player aircraft
fn setup_player_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player aircraft mesh
    let aircraft_mesh = meshes.add(Cuboid::new(35.0, 5.0, 50.0));
    let aircraft_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.1, 0.1), // Bright red
        metallic: 1.0,
        perceptual_roughness: 0.75,
        ..default()
    });

    commands.spawn((
        LocalPlayerBundle::default(),
        Mesh3d(aircraft_mesh),
        MeshMaterial3d(aircraft_material),
    ));
}

fn camera_follow_system(
    player_query: Query<&Transform, (With<Aircraft>, With<Player>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Aircraft>)>,
    time: Res<Time>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) =
        (player_query.single(), camera_query.single_mut())
    {
        let player_pos = player_transform.translation;
        let player_forward = player_transform.forward();

        let offset = *-player_forward * 50.0 + Vec3::Y * 30.0;
        let target_position = player_pos + offset;

        let lerp_factor = 5.0 * time.delta_secs();
        camera_transform.translation = camera_transform
            .translation
            .lerp(target_position, lerp_factor);
        camera_transform.look_at(player_pos, Vec3::Y);
    }
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_control: Res<MouseControl>,
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut query: Query<(&mut LocalInput, &Velocity, &AircraftData)>,
) {
    for (mut input, velocity, aircraft) in query.iter_mut() {
        input.pitch = 0.0;
        input.roll = 0.0;
        input.yaw = 0.0;

        // Calculate velocity-scaled sensitivity
        let speed = velocity.0.length();
        let speed_ratio = (speed / aircraft.max_speed).clamp(0.0, 1.0);
        let sensitivity_scale =
            1.0 + (speed_ratio * (mouse_control.max_sensitivity_multiplier - 1.0));
        let current_sensitivity = mouse_control.base_sensitivity * sensitivity_scale;

        // Mouse input
        let mut mouse_delta = Vec2::ZERO;
        for event in mouse_motion.read() {
            mouse_delta += event.delta;
        }

        input.roll =
            mouse_delta.x * current_sensitivity * if mouse_control.invert_y { 1.0 } else { -1.0 };
        input.pitch =
            mouse_delta.y * current_sensitivity * if mouse_control.invert_y { -1.0 } else { 1.0 };

        // Keyboard yaw
        if keys.pressed(KeyCode::KeyQ) {
            input.yaw = 100.0 * current_sensitivity;
        }
        if keys.pressed(KeyCode::KeyE) {
            input.yaw = -100.0 * current_sensitivity;
        }

        // Throttle
        if keys.pressed(KeyCode::KeyW) {
            input.throttle = (input.throttle + 0.01).min(1.0);
        }
        if keys.pressed(KeyCode::KeyS) {
            input.throttle = (input.throttle - 0.01).max(0.0);
        }
    }
}

/// Toggle cursor lock
fn toggle_cursor_system(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut windows: Query<&mut Window>,
) {
    if let Ok(mut window) = windows.single_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            window.cursor_options.visible = true;
            window.cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
        }

        if mouse_buttons.just_pressed(MouseButton::Left) {
            window.cursor_options.visible = false;
            window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
        }
    }
}

fn apply_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &LocalInput, &AircraftData), With<Aircraft>>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut velocity, input, aircraft) in query.iter_mut() {
        let speed = velocity.0.length();
        let speed_ratio = (speed / aircraft.max_speed).clamp(0.0, 1.0);

        // Speed-based turn rate reduction
        let turn_scale = 1.0 + (speed_ratio * 0.5);

        // Rotation
        transform.rotate_local_x(input.pitch * aircraft.turn_rate * turn_scale * dt);
        transform.rotate_local_y(input.yaw * aircraft.turn_rate * turn_scale * dt);
        transform.rotate_local_z(input.roll * aircraft.turn_rate * 2.0 * turn_scale * dt);

        // Thrust
        let forward = transform.forward().as_vec3();
        let target_speed = aircraft.max_speed * input.throttle;
        let current_speed = speed;
        let speed_change = (target_speed - current_speed) * aircraft.acceleration * dt / 100.0;

        // Apply speed change in forward direction
        velocity.0 = forward * (current_speed + speed_change);

        // Update position
        transform.translation += velocity.0 * dt;

        // Ground collision
        if transform.translation.y < 10.0 {
            transform.translation.y = 10.0;
            velocity.0.y = 0.0;
        }
    }
}

fn debug_info_system(
    query: Query<(&Transform, &Velocity, &LocalInput), With<Aircraft>>,
    time: Res<Time>,
) {
    if time.elapsed_secs() % 1.0 < 0.016 {
        for (transform, velocity, input) in query.iter() {
            let pos = transform.translation;
            let speed = velocity.0.length();
            let altitude = pos.y;

            println!(
                "[{:.0}, {:.0}, {:.0}] | Alt: {:.0}m | Speed: {:.1} m/s | Throttle:{:.1}%",
                pos.x,
                pos.y,
                pos.z,
                altitude,
                speed,
                input.throttle * 100.0
            );
        }
    }
}
