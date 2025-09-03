use crate::{common::state::AppState, game::GameEntity};
use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 2000.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), despawn_all_game_entities)
            .add_systems(
                Update,
                despawn_far_away_entities.run_if(in_state(AppState::InGame)),
            );
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        // Entity is far away from the camera's viewport.
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_all_game_entities(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
    query.iter().for_each(|entity| {
        commands.entity(entity).try_despawn();
    });
}
