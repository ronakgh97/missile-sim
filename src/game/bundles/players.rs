use crate::game::components::aircraft::{AircraftData, Weapons};
use crate::game::components::controller::Controller;
use crate::game::components::health::Health;
use crate::game::components::input::{LocalInput, RemoteInput};
use crate::game::components::marker::{Aircraft, Player};
use crate::game::components::movement::{Acceleration, Velocity};
use crate::game::components::player::{PlayerId, PlayerInfo};
use bevy::prelude::*;

/// Bundle for spawning a LOCAL PLAYER
#[derive(Bundle)]
pub struct LocalPlayerBundle {
    // Markers
    pub aircraft: Aircraft,
    pub player: Player,

    // Player data
    pub player_id: PlayerId,
    pub player_info: PlayerInfo,
    pub controller: Controller,

    // Aircraft data
    pub aircraft_data: AircraftData,
    pub weapons: Weapons,

    // Movement
    pub velocity: Velocity,
    pub acceleration: Acceleration,

    // Combat
    pub health: Health,

    // Input
    pub local_input: LocalInput,

    // Bevy built-ins
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Default for LocalPlayerBundle {
    fn default() -> Self {
        Self {
            aircraft: Aircraft,
            player: Player,
            player_id: PlayerId::LOCAL,
            player_info: PlayerInfo::default(),
            controller: Controller::LocalPlayer,
            aircraft_data: AircraftData::f16(),
            weapons: Weapons::default(),
            velocity: Velocity(Vec3::new(100.0, 0.0, 0.0)),
            acceleration: Acceleration(Vec3::ZERO),

            health: Health::new(100.0),
            local_input: LocalInput::default(),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

impl LocalPlayerBundle {
    /// Create with custom aircraft type
    pub fn with_aircraft(aircraft_type: AircraftData) -> Self {
        Self {
            aircraft_data: aircraft_type,
            ..default()
        }
    }

    /// Create at specific position
    pub fn at_position(position: Vec3) -> Self {
        Self {
            transform: Transform::from_translation(position),
            ..default()
        }
    }
}

/// Bundle for spawning a REMOTE PLAYER (from network)
#[derive(Bundle)]
pub struct RemotePlayerBundle {
    // Markers
    pub aircraft: Aircraft,
    pub player: Player,

    // Player data
    pub player_id: PlayerId,
    pub player_info: PlayerInfo,
    pub controller: Controller,

    // Aircraft data
    pub aircraft_data: AircraftData,
    pub weapons: Weapons,

    // Movement
    pub velocity: Velocity,
    pub acceleration: Acceleration,

    // Combat
    pub health: Health,

    // Input (from network)
    pub remote_input: RemoteInput,

    // Bevy built-ins
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl RemotePlayerBundle {
    pub fn new(id: u32, name: String, position: Vec3) -> Self {
        Self {
            aircraft: Aircraft,
            player: Player,
            player_id: PlayerId::new(id),
            player_info: PlayerInfo { name, ..default() },
            controller: Controller::RemotePlayer,
            aircraft_data: AircraftData::f16(),
            weapons: Weapons::default(),
            velocity: Velocity(Vec3::ZERO),
            acceleration: Acceleration(Vec3::ZERO),
            health: Health::new(100.0),
            remote_input: RemoteInput::default(),
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
