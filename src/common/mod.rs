use bevy::prelude::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            asset_loader::AssetLoaderPlugin,
            state::StatePlugin,
            despawn::DespawnPlugin,
            lifetime::LifetimePlugin,
        ));
    }
}

pub mod asset_loader;
pub mod despawn;
pub mod lifetime;
pub mod prelude;
pub mod state;
