use bevy::prelude::*;

/// Aircraft data (shared by all jets)
#[derive(Component, Debug, Clone)]
pub struct AircraftData {
    pub aircraft_type: AircraftType,
    pub max_speed: f32,
    pub acceleration: f32,
    pub turn_rate: f32,
}

impl Default for AircraftData {
    fn default() -> Self {
        Self::f16()
    }
}

impl AircraftData {
    pub fn f16() -> Self {
        Self {
            aircraft_type: AircraftType::F16,
            max_speed: 400.0,
            acceleration: 50.0,
            turn_rate: 2.0,
        }
    }

    pub fn f22() -> Self {
        Self {
            aircraft_type: AircraftType::F22,
            max_speed: 450.0,
            acceleration: 60.0,
            turn_rate: 2.5,
        }
    }

    pub fn su27() -> Self {
        Self {
            aircraft_type: AircraftType::Su27,
            max_speed: 420.0,
            acceleration: 55.0,
            turn_rate: 2.2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftType {
    F16,
    F22,
    Su27,
    MiG29,
}

#[derive(Component, Debug, Clone)]
pub struct Weapons {
    pub missiles: u32,
    pub flares: u32,
}

impl Default for Weapons {
    fn default() -> Self {
        Self {
            missiles: 6,
            flares: 20,
        }
    }
}
