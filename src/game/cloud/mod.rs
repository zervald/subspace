use crate::game::{
    effect::{EffectSource, effect_event::AddEffectEvent},
    prelude::*,
};
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Copy, Clone, Default, Debug)]
#[require(GameEntity, CollisionEventsEnabled)]
pub enum Cloud {
    #[default]
    Nebula,
    Astroid,
    HeavyNebula,
    Thunderstorm,
}

impl Cloud {
    fn effects(&self) -> Vec<EffectType> {
        match self {
            Cloud::Nebula => vec![EffectType::Obscured],
            Cloud::Astroid => vec![EffectType::Obscured],
            Cloud::HeavyNebula => vec![EffectType::Obscured],
            Cloud::Thunderstorm => vec![EffectType::Obscured, EffectType::Electricity],
        }
    }
}

#[derive(Bundle, Debug)]
pub struct CloudBundle {
    cloud: Cloud,
    collider: Collider, // compound of childrens?
    transform: Transform,
}

impl CloudBundle {
    pub fn new(cloud: Cloud, x: f32, y: f32) -> Self {
        Self {
            cloud,
            collider: Collider::default(),
            transform: Transform::from_xyz(x, y, RadarOrdering::ZClouds.as_f32()),
        }
    }
}
// TODO: use Collider::compound or children entity to create a single collider

#[derive(Bundle, Debug)]
pub struct CloudParts {
    collider: Collider,
    mesh2d: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

fn cloud_enter(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    cloud_query: Query<(Entity, &Cloud)>,
    ships_query: Query<Entity, With<Spaceship>>,
) {
    if let Ok((cloud, cloud_type)) = cloud_query.get(trigger.target()) {
        if let Ok(target) = ships_query.get(trigger.collider) {
            commands.trigger(AddEffectEvent::new(
                cloud,
                target,
                None,
                cloud_type.effects(),
            ))
        }
    }
}

fn cloud_exit(
    trigger: Trigger<OnCollisionEnd>,
    mut commands: Commands,
    cloud_query: Query<&EffectSource, With<Cloud>>,
) {
    let effect_entity = match cloud_query.get(trigger.target()) {
        Ok(e) => **e,
        Err(_) => return,
    };

    if let Ok(mut ec) = commands.get_entity(effect_entity) {
        ec.try_despawn();
    }
}

pub struct CloudPlugin;
impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(cloud_enter);
        app.add_observer(cloud_exit);
    }
}
