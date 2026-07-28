use crate::game::prelude::EffectType;
use bevy::prelude::*;

pub struct Nebula;

impl super::CloudBundle for Nebula {
    fn default_effects() -> Vec<EffectType> {
        vec![EffectType::Obscured]
    }
}

// CloudType::HeavyNebula => vec![EffectType::Obscured],
// CloudType::Astroid => vec![EffectType::Obscured],
// CloudType::Thunderstorm => vec![EffectType::Obscured, EffectType::Electricity],
