use crate::game::prelude::Electrified;
use crate::game::prelude::Obscured;
use bevy::prelude::*;

mod effect_event;
mod effect_types;

pub mod prelude {
    pub use super::Effect;
    pub use super::effect_event::AddEffect;
    pub use super::effect_types::electricity::Electrified;
    pub use super::effect_types::obscured::Obscured;
}

pub struct EffectPlugin;
impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        // app.add_message::<EventAddEffect>();
        app.add_plugins(effect_types::plugin);
    }
}

#[derive(Component, Debug, Clone)]
pub enum Effect {
    Obscured(Obscured),
    Electrified(Electrified),
}

impl Effect {
    fn base_bundle() -> impl Bundle {}
    pub fn spawn<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        match self {
            Effect::Obscured(obscured) => commands.spawn((obscured, Self::base_bundle())),
            Effect::Electrified(electrified) => commands.spawn((electrified, Self::base_bundle())),
        }
    }
}
// fn spawn_effect_entity(&self) -> EffectEntity;
//
// fn add_to(&self, mut commands: Commands, entity: Entity) {
//     match commands.get_entity(entity) {
//         Ok(mut ec) => ec.add_child(self.spawn_effect_entity()),
//         Err(_) => return,
//     };
// }
