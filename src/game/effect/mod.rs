use bevy::prelude::*;
use effect_event::AddEffectEvent;
use effect_types::EffectTypePlugin;

use crate::{common::prelude::Lifetime, game::effect::effect_types::EffectType};

/// Marker component for an "Effect Entity", determining when it should get removed
///
/// This effect entity will be despawn when [`Effect::source`] is removed
/// or when [`Effect::lifetime`] runs out.
///
///
#[derive(Component)]
pub struct Effect {
    source: Entity,
    lifetime: Option<Lifetime>,
    effects: Vec<EffectType>,
}

pub type EffectEntity = Entity;

// TODO: form AddEffectEvent -> EffectEntity
impl Effect {}

#[derive(Component, Deref, DerefMut)]
pub struct EffectSource {
    effect_entity: Entity,
}

impl From<Entity> for EffectSource {
    fn from(value: Entity) -> Self {
        Self {
            effect_entity: value,
        }
    }
}

pub mod effect_event;
pub mod effect_types;

pub struct EffectPlugin;
impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddEffectEvent>();
        app.add_plugins(EffectTypePlugin);
    }
}
