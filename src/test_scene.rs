use crate::{
    common::state::AppState,
    game::{
        planet::spawn_test_planet, playership::spawn_playership, radar_camera::spawn_camera,
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
    }
}
