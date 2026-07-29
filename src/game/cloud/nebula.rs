use std::time::Duration;

use bevy::prelude::*;

use crate::game::prelude::*;

pub struct Nebula;

impl super::CloudBundle for Nebula {
    fn default_effects() -> Vec<Effect> {
        // TODO:
        vec![
            Effect::Electrified(Electrified {
                magnitude: 1.,
                duration: Timer::new(Duration::new(2, 0), TimerMode::Once),
            }),
            Effect::Obscured(Obscured {
                magnitude: 1.,
                duration: Timer::new(Duration::new(2, 0), TimerMode::Once),
            }),
        ]
    }
}

// CloudType::HeavyNebula => vec![EffectType::Obscured],
// CloudType::Astroid => vec![EffectType::Obscured],
// CloudType::Thunderstorm => vec![EffectType::Obscured, EffectType::Electricity],
