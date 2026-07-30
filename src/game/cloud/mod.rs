use std::sync::Arc;

use crate::game::prelude::*;
use avian2d::collision::collision_events::CollisionEventsEnabled;
use bevy::prelude::*;
use bevy_alchemy::Effect;

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

#[derive(Component)]
#[require()]
pub struct Cloud {
    // pub effects: Vec<Effect<B>>,
}
