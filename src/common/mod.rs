use bevy::prelude::*;

mod despawn;
pub mod lifetime;
pub mod prelude;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((despawn::DespawnPlugin, lifetime::LifetimePlugin));
    }
}
