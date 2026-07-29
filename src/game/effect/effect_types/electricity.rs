use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Electrified {
    pub magnitude: f32,
    pub duration: Timer,
}
