use crate::game::prelude::*;
use bevy::prelude::*;

pub struct ContactPlugin;
impl Plugin for ContactPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     Update,
        //     ().run_if(in_state(GameState::Cruising)),
        // );
    }
}

#[allow(dead_code)]
fn visibility_manager(mut query: Query<&mut Visibility, (With<Emission>, Without<PlayerShip>)>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

#[allow(dead_code)]
fn spawn_ghost() {
    todo!()
}
