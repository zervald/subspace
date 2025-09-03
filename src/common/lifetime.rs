use bevy::prelude::*;

#[derive(Component)]
pub struct Lifetime {
    timer: Timer,
}

impl Lifetime {
    pub fn from_duration(duration: f32) -> Self {
        Lifetime {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}
