use crate::{
    common::state::AppState,
    game::{
        cloud::{Cloud, CloudBundle},
        planet::spawn_test_planet,
        playership::spawn_playership,
        radar_camera::spawn_camera,
        spaceship::spawn_test_ennemy,
    },
};
use bevy::prelude::*;

pub struct TestScenePlugin;
impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_test_planet);
        app.add_systems(OnEnter(AppState::InGame), spawn_playership);
        app.add_systems(OnEnter(AppState::InGame), spawn_camera);
        app.add_systems(OnEnter(AppState::InGame), spawn_test_ennemy);
        app.add_systems(OnEnter(AppState::InGame), spawn_test_cloud);
    }
}

fn spawn_test_cloud(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    CloudBundle::spawn(
        Cloud::Thunderstorm,
        -10.0,
        -10.0,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
