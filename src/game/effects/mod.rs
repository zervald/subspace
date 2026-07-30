use bevy::prelude::*;

mod electricity;
mod obscured;

#[allow(unused)]
pub mod prelude {
    // pub use super::electricity::Electrified;
    // pub use super::obscured::Obscured;
}

pub struct EffectsPlugin;
impl Plugin for EffectsPlugin {
    fn build(&self, _app: &mut App) {}
}
