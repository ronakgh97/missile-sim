use bevy::prelude::*;

/// Player ID for identifying different players (multiplayer)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u32);

impl PlayerId {
    pub const LOCAL: PlayerId = PlayerId(0);

    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn is_local(&self) -> bool {
        self.0 == 0
    }
}

/// Player information
#[derive(Component, Debug, Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub score: u32,
    pub kills: u32,
    pub deaths: u32,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            name: "Player 1".to_string(),
            score: 0,
            kills: 0,
            deaths: 0,
        }
    }
}
