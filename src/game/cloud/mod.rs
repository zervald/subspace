use super::z_ordering;
use crate::game::{
    effect::{EffectSource, effect_event::EventAddEffect},
    prelude::*,
};
use avian2d::prelude::*;
use bevy::prelude::*;

const DEFAULT_CIRCLE_RADIUS: f32 = 10.0;
const DEFAULT_COLOR: Color = Color::srgba(0.0, 0.2, 0.3, 0.5);

#[derive(Component, Copy, Clone, Default, Debug)]
#[require(GameEntity)]
pub enum Cloud {
    #[default]
    Nebula,
    HeavyNebula,
    Astroid,
    Thunderstorm,
}

impl Cloud {
    fn effects(&self) -> Vec<EffectType> {
        match self {
            Cloud::Nebula => vec![EffectType::Obscured],
            Cloud::HeavyNebula => vec![EffectType::Obscured],
            Cloud::Astroid => vec![EffectType::Obscured],
            Cloud::Thunderstorm => vec![EffectType::Obscured, EffectType::Electricity],
        }
    }
}

#[derive(Bundle, Debug)]
pub struct CloudBundle {
    cloud: Cloud,
    transform: Transform,
}

impl CloudBundle {
    pub fn spawn(
        cloud: Cloud,
        x: f32,
        y: f32,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn((
                Self {
                    cloud,
                    transform: Transform::from_xyz(x, y, RadarOrdering::ZClouds.as_f32()),
                },
                Visibility::Visible,
            ))
            .with_children(|children| {
                children.spawn(CloudColliderChild::new(
                    x - x / 2.,
                    y - y / 2.,
                    meshes,
                    materials,
                ));
                children.spawn(CloudColliderChild::new(
                    x + x / 2.,
                    y + y / 2.,
                    meshes,
                    materials,
                ));
                children.spawn(CloudColliderChild::new(
                    x - x / 2.,
                    y + y / 2.,
                    meshes,
                    materials,
                ));
            });
    }
}
// TODO: use Collider::compound or children entity to create a single collider

#[derive(Component, Debug, Default)]
#[require(CollisionEventsEnabled)]
pub struct CloudPart;

#[derive(Bundle, Debug)]
struct CloudColliderChild {
    cloud_part: CloudPart,
    collider: Collider,
    mesh2d: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

impl CloudColliderChild {
    fn new(
        x: f32,
        y: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            cloud_part: CloudPart,
            collider: Collider::circle(DEFAULT_CIRCLE_RADIUS),
            mesh2d: Mesh2d(meshes.add(Circle::new(DEFAULT_CIRCLE_RADIUS))),
            mesh_material: MeshMaterial2d(materials.add(ColorMaterial::from_color(DEFAULT_COLOR))),
            transform: Transform::from_xyz(x, y, z_ordering::RadarOrdering::ZClouds.as_f32()),
        }
    }
}

fn cloud_enter(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    collider_query: Query<&ChildOf, With<CloudPart>>,
    cloud_query: Query<&Cloud>,
    ships_query: Query<NameOrEntity, With<Spaceship>>,
) {
    let cloud: Entity = match collider_query.get(trigger.target()) {
        Ok(c) if c.0 != Entity::PLACEHOLDER => c.0,
        _ => return,
    };
    let Ok(cloud_type) = cloud_query.get(cloud) else {
        return;
    };
    let Ok(target) = ships_query.get(trigger.collider) else {
        return;
    };

    info!("Cloud {cloud} collided with {target}");
    commands.trigger(EventAddEffect::new(
        cloud,
        target.entity,
        None,
        cloud_type.effects(),
    ));
}

// TODO: find solution with lifetime,
// maybe lifetime::event(T: Event) ?
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
        // app.add_observer(cloud_exit);
    }
}
