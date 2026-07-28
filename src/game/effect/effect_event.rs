use std::sync::Arc;

use bevy::prelude::*;

use crate::{common::prelude::Lifetime, game::effect::effect_types::EffectType};

#[derive(Message, Event)]
pub struct EventAddEffect {
    source: Entity,
    target: Entity,
    lifetime: Option<Lifetime>,
    effects: Arc<[EffectType]>,
}

impl EventAddEffect {
    pub fn new(
        source: Entity,
        target: Entity,
        lifetime: Option<Lifetime>,
        effects: Arc<[EffectType]>,
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
