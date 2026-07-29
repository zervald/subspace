use crate::game::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

const MAX_VEL_DOCKING: f32 = 10.;

pub struct StationPlugin;
impl Plugin for StationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(attach_observer);
    }
}

#[derive(Component, Debug)]
#[require(GameEntity, CollisionEventsEnabled, RigidBody::Static)]
pub struct Station;

#[derive(Bundle, Debug)]
pub struct StationBundle {
    station: Station,
    collider: Collider,
    mesh2d: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

fn attach_observer(trigger: On<Add, Station>, mut commands: Commands) {
    if let Ok(mut entity_command) = commands.get_entity(trigger.event_target()) {
        entity_command.observe(obs_docking_collision);
    }
}

#[allow(dead_code)]
fn new_station(
    mut commands: Commands,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    name: String,
    size: f32,
) -> Entity {
    commands
        .spawn((
            Name::new(name),
            Station,
            Collider::circle(size),
            mesh,
            material,
            RigidBody::Static,
            Transform::from_xyz(-10., 0., RadarZOrdering::Planet.z_order()),
        ))
        .id()
}

fn obs_docking_collision(
    trigger: On<CollisionStart>,
    station_query: Query<(NameOrEntity, &Station)>,
    mut ships_query: Query<(&mut LinearVelocity, &mut Health), With<Spaceship>>,
    // mut event: EventWriter<EventDocking>,
) {
    if trigger.body1.is_none() {
        return;
    }
    let station_entity = trigger.event_target();
    let other_entity = trigger.collider1;

    if let Ok((vel, _health)) = ships_query.get_mut(other_entity) {
        if let Ok((name, station)) = &station_query.get(station_entity) {
            info!("DOCKING: {other_entity} collided with station `{name}`");
            if vel.length() < MAX_VEL_DOCKING {
                //Docking
                // TODO:
                // event.write(EventDocking {
                //     ship_id: other_entity,
                //     station_id: station_entity,
                // });
            }
        }
    }
}

#[allow(dead_code)]
fn rotate_stations(
    mut stations: Query<(&mut Transform, &ChildOf), With<Station>>,
    parents: Query<&Transform, (With<Children>, Without<Station>)>,
    timer: Res<Time>,
) {
    for (mut transform, parent) in &mut stations {
        if let Ok(parent_pos) = parents.get(parent.parent()) {
            let angle = (parent_pos.translation - transform.translation)
                .angle_between(transform.translation);
            transform.rotation = Quat::from_rotation_z(angle);
            // Interpolate between the current rotation and the fully turned rotation
            // when looking a the parent, with a given turn speed to get a smooth motion.
            // With higher speed the curvature of the orbit would be smaller.
            let incremental_turn_weight = 1. * timer.delta_secs();
            let old_rotation = transform.rotation;
            transform.rotation = old_rotation.lerp(transform.rotation, incremental_turn_weight);
        }
    }
}
