use crate::game::prelude::Effect;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct AddEffect {
    effect_entity: Entity,
    parent: Entity,
    effect: Effect,
}

impl AddEffect {
    pub fn new(effect_entity: Entity, parent: Entity, effect: Effect) -> Self {
        Self {
            effect_entity,
            parent,
            effect,
        }
    }
}
