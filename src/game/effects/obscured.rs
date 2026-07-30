use bevy::prelude::*;
use bevy_alchemy::{Delay, EffectTimer, Lifetime};

#[derive(Component, Debug, Clone)]
pub struct Obscured {
    magnitude: i32,
}

pub fn obscure_bundle() -> impl Bundle {
    (
        Obscured { magnitude: 1 },   // The amount of damage to apply per tick.
        Lifetime::from_seconds(3.0), // The duration of the effect.
        Delay::from_seconds(1.0) // The time between damage ticks.
            .trigger_immediately(), // Make damage tick immediately when the effect is applied.
    )
}
