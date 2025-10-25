use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Controller {
    LocalPlayer,
    RemotePlayer,
    AI,
}

impl Default for Controller {
    fn default() -> Self {
        Self::LocalPlayer
    }
}
