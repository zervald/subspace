use crate::common::state::AppState;
use avian2d::prelude::ExternalForce;
use bevy::prelude::*;

const DEFAULT_REACH: f32 = 1000.;
const GRAVITY_ENABLED: bool = false;

#[derive(Component, Debug)]
pub struct GravitySource {
    pub force: f32,
    pub reach: f32,
}

impl Default for GravitySource {
    fn default() -> Self {
        Self {
            force: 10000.,
            reach: DEFAULT_REACH,
        }
    }
}

#[derive(Component, Debug)]
pub struct GravityAffected;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            apply_gravity.run_if(in_state(AppState::InGame).and(|| GRAVITY_ENABLED)),
        );
    }
}

fn apply_gravity(
    mut commands: Commands,
    sources: Query<(&Transform, &GravitySource)>,
    objects: Query<(Entity, &Transform), With<GravityAffected>>,
) {
    if sources.is_empty() || objects.is_empty() {
        return;
    }
    for (source_transform, gravity) in sources {
        for (object, object_transform) in objects {
            let distance = source_transform
                .translation
                .distance_squared(object_transform.translation);
            let reach = gravity.reach * gravity.reach;
            if distance < reach {
                let dir = source_transform.translation - object_transform.translation;
                let vec_force = gravity.force * dir.xy() * (1. / distance);
                info!(?vec_force);
                commands
                    .entity(object)
                    .try_insert(ExternalForce::new(vec_force).with_persistence(false));
            }
        }
    }
}
