use bevy::prelude::*;

use crate::{common::prelude::Lifetime, game::effect::effect_types::EffectType};

#[derive(Event)]
pub struct AddEffectEvent {
    source: Entity,
    target: Entity,
    lifetime: Option<Lifetime>,
    effects: Vec<EffectType>,
}

impl AddEffectEvent {
    pub fn new(
        source: Entity,
        target: Entity,
        lifetime: Option<Lifetime>,
        effects: Vec<EffectType>,
    ) -> Self {
        Self {
            source,
            target,
            lifetime,
            effects,
        }
    }
}

//TODO: AddEffectEvent system / observers
