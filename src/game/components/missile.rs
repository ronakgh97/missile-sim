use bevy::prelude::*;

/// Missile data
#[derive(Component, Debug, Clone)]
pub struct MissileData {
    pub guidance: GuidanceType,
    pub max_speed: f32,
    pub max_acceleration: f32,
    pub fuel_remaining: f32,
    pub navigation_constant: f32,
    pub target: Option<Entity>,
    pub launched_by: Entity,
}

impl Default for MissileData {
    fn default() -> Self {
        Self {
            guidance: GuidanceType::TPN,
            max_speed: 800.0,
            max_acceleration: 500.0,
            fuel_remaining: 10.0,
            navigation_constant: 4.0,
            target: None,
            launched_by: Entity::PLACEHOLDER,
        }
    }
}

/// Guidance algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidanceType {
    PurePursuit,
    LeadPursuit,
    PPN,
    TPN,
}
