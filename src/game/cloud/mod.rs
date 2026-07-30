use std::sync::Arc;

use crate::game::prelude::*;
use avian2d::collision::collision_events::CollisionEventsEnabled;
use bevy::prelude::*;

mod cloud_part;
mod cloud_systems;
mod nebula;

pub mod prelude {
    pub use super::cloud_systems::AffectedByCloud;
}

pub struct CloudPlugin;
impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(cloud_systems::plugin);
    }
}

const DEFAULT_CIRCLE_RADIUS: f32 = 10.0;
const DEFAULT_COLOR: Color = Color::srgba(0.0, 0.2, 0.3, 0.5);

#[derive(Component, Debug, Clone)]
pub struct Cloud {
    pub effects: Arc<[Effect]>,
}

pub trait CloudBundle {
    fn default_effects() -> Vec<Effect>;
    fn new(x: f32, y: f32) -> impl Bundle {
        (
            Cloud {
                effects: Arc::from(Self::default_effects()),
            },
            Transform::from_xyz(x, y, RadarZOrdering::Clouds.z_order()),
            CollisionEventsEnabled,
        )
    }
}
