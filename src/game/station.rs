use crate::game::prelude::*;
use avian2d::prelude::*;

pub struct StationPlugin;
impl Plugin for StationPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Debug, Default, Clone)]
#[require(GameEntity)]
pub struct Station;

pub fn station(
    x: f32,
    y: f32,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    size: f32,
) -> impl Bundle {
    (
        Station,
        Collider::circle(size),
        mesh,
        material,
        RigidBody::Static,
        CollisionEventsEnabled,
        Transform::from_xyz(x, y, RadarZOrdering::Planet.z_order()),
    )
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
