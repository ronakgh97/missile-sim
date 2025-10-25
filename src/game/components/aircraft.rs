use bevy::prelude::*;

/// Complete aircraft physics & stats (everything in one place!)
#[derive(Component, Debug, Clone)]
pub struct AircraftData {
    pub aircraft_type: AircraftType,

    // GAMEPLAY STATS
    pub max_speed: f32,
    pub turn_rate: f32,
    pub acceleration: f32,

    // PHYSICS
    pub mass: f32,
    pub moment_of_inertia: Vec3,
    pub angular_velocity: Vec3,

    //AERODYNAMICS
    pub wing_area: f32,
    pub lift_coefficient: f32,
    pub drag_coefficient: f32,

    // ENGINE
    pub max_thrust: f32,
    pub current_thrust: f32,

    pub angle_of_attack: f32,
    pub sideslip_angle: f32,
}

impl AircraftData {
    pub fn f16() -> Self {
        Self {
            aircraft_type: AircraftType::F16,
            max_speed: 680.0,
            turn_rate: 2.0,
            acceleration: 50.0,
            mass: 15000.0,
            moment_of_inertia: Vec3::new(50000.0, 120000.0, 150000.0),
            angular_velocity: Vec3::ZERO,
            wing_area: 27.87,
            lift_coefficient: 0.8,
            drag_coefficient: 0.025,
            max_thrust: 127000.0,
            current_thrust: 0.0,
            angle_of_attack: 0.0,
            sideslip_angle: 0.0,
        }
    }

    pub fn f22() -> Self {
        Self {
            aircraft_type: AircraftType::F22,
            max_speed: 720.0,
            turn_rate: 2.5,
            acceleration: 60.0,
            mass: 19000.0,
            moment_of_inertia: Vec3::new(60000.0, 140000.0, 180000.0),
            angular_velocity: Vec3::ZERO,
            wing_area: 78.0,
            lift_coefficient: 0.9,
            drag_coefficient: 0.02,
            max_thrust: 156000.0,
            current_thrust: 0.0,
            angle_of_attack: 0.0,
            sideslip_angle: 0.0,
        }
    }

    pub fn su27() -> Self {
        Self {
            aircraft_type: AircraftType::Su27,
            max_speed: 700.0,
            turn_rate: 2.2,
            acceleration: 55.0,
            mass: 16000.0,
            moment_of_inertia: Vec3::new(55000.0, 130000.0, 160000.0),
            angular_velocity: Vec3::ZERO,
            wing_area: 62.0,
            lift_coefficient: 0.85,
            drag_coefficient: 0.022,
            max_thrust: 137000.0,
            current_thrust: 0.0,
            angle_of_attack: 0.0,
            sideslip_angle: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftType {
    F16,
    F22,
    Su27,
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
