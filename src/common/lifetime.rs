use bevy::prelude::*;

use crate::game::prelude::GameState;

#[derive(Component)]
pub enum Lifetime {
    Timer(Timer),
    State(GameState),
    Entity(Entity),
}

impl Lifetime {
    pub fn from_duration(duration: f32) -> Self {
        Lifetime::Timer(Timer::from_seconds(duration, TimerMode::Once))
    }

    pub fn finished(&self) {}
}
