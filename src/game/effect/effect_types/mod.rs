use bevy::prelude::*;

#[derive(Component, Debug)]
pub enum EffectType {
    Obscured,
    Electricity,
}

mod effect_electricity;

pub struct EffectTypePlugin;
impl Plugin for EffectTypePlugin {
    fn build(&self, _app: &mut App) {}
}
