use bevy::prelude::*;

/// Local player input (from keyboard/gamepad)
#[derive(Component, Debug, Clone, Default)]
pub struct LocalInput {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub throttle: f32,
    pub fire_missile: bool,
    pub deploy_flare: bool,
    pub deploy_chaff: bool,
}

/// Remote player input (from network)
#[derive(Component, Debug, Clone, Default)]
pub struct RemoteInput {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub throttle: f32,
    pub fire_missile: bool,
    pub deploy_flare: bool,
    pub deploy_chaff: bool,
    pub sequence_number: u32, // For network ordering
    pub timestamp: f64,
}
