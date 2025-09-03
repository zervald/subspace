use bevy::prelude::*;

use crate::common::state::AppState;

#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct Health(pub i32);

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            despawn_dead_entities.run_if(in_state(AppState::InGame)),
        );
    }
}

impl From<i32> for Health {
    fn from(amount: i32) -> Self {
        Self(amount)
    }
}

impl Health {
    pub fn new(value: i32) -> Health {
        Self(value)
    }

    pub fn damage(&mut self, amount: i32) {
        self.0 -= amount;
    }
}

fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        // Entity doesn't have any health.
        if health.0 <= 0 {
            commands.entity(entity).try_despawn();
        }
    }
}
