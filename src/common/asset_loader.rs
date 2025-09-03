use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct AssetLibrary {
    #[allow(dead_code)]
    pub space_background: Handle<Image>,
    // pub asteroid: Handle<Scene>,
    // pub spaceship: Handle<Scene>,
    // pub missiles: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLibrary>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut asset_library: ResMut<AssetLibrary>, asset_server: Res<AssetServer>) {
    *asset_library = AssetLibrary {
        space_background: asset_server
            .load("Free_Space_Background_By_Digitalmoons/Space_Background1.png"),
        // asteroid: asset_server.load("Asteroid.glb#Scene0"),
        // spaceship: asset_server.load("Spaceship.glb#Scene0"),
        // missiles: asset_server.load("Missiles.glb#Scene0"),
    }
}
