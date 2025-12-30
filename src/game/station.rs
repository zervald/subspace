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
pub struct Station {
    pub name: String,
}

#[derive(Bundle, Debug)]
pub struct StationBundle {
    station: Station,
    collider: Collider,
    mesh2d: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

fn attach_observer(trigger: Trigger<OnAdd, Station>, mut commands: Commands) {
    if let Ok(mut entity_command) = commands.get_entity(trigger.target()) {
        entity_command.observe(obs_station_collision);
    }
}

#[allow(dead_code)]
fn new_station(
    mut commands: Commands,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    name: &str,
    size: f32,
) -> Entity {
    commands
        .spawn((
            Station { name: name.into() },
            Collider::circle(size),
            mesh,
            material,
            RigidBody::Static,
            Transform::from_xyz(-10., 0., RadarOrdering::ZPlanet.as_f32()),
        ))
        .id()
}

fn obs_station_collision(
    trigger: Trigger<OnCollisionStart>,
    station_query: Query<&Station>,
    mut ships_query: Query<(&mut LinearVelocity, &mut Health), With<Spaceship>>,
    // mut event: EventWriter<EventDocking>,
) {
    if trigger.body.is_none() {
        return;
    }
    let station_entity = trigger.target();
    let other_entity = trigger.collider;

    if let Ok((vel, mut health)) = ships_query.get_mut(other_entity) {
        let name = &station_query.get(station_entity).unwrap().name;
        info!("COLLISION: {other_entity} collided with station: {name}");
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

// NOTE: Now done planet rotation
// BUG: Circle becomes bigger over time
// mayne need rotation
#[allow(dead_code)]
fn move_stations(
    mut stations: Query<&mut Transform, (With<Station>, With<ChildOf>)>,
    timer: Res<Time>,
) {
    for mut transform in &mut stations {
        // TODO: system/fn to determine orbital velocity,
        // store vel in station ?
        let forward = transform.up();
        transform.translation += forward * 10. * timer.delta_secs();
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
