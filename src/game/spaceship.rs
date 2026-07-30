use crate::game::detection::sensor::*;
use crate::game::gravity::AffectedByGravity;
use crate::game::prelude::*;
use avian2d::prelude::*;

const DEFAULT_HEALTH: i32 = 100;
const MAX_ANGULAR_SPEED: f32 = 20.0;
const ROTATION_DAMPENING: f32 = 2.0;

pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Debug)]
#[require(
    GameEntity,
    // ship gameplay
    Health(DEFAULT_HEALTH),
    Emission,
    PassiveSensor,
    // AffectedBy
    AffectedByCloud,
    AffectedByGravity,
    // physics
    AngularDamping(ROTATION_DAMPENING),
    CollisionEventsEnabled,
    Mass(2.0),
    MaxAngularSpeed(MAX_ANGULAR_SPEED),
    NoAutoMass,
    RigidBody::Dynamic,
)]
pub struct Spaceship;

#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct ShipShield {
    pub strength: f32,
    pub active: bool,
}

pub fn spaceship(
    x: f32,
    y: f32,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> impl Bundle {
    let shape = Triangle2d::new(Vec2::Y * 5.0, vec2(-2.5, -2.5), vec2(2.5, -2.5));
    (
        Spaceship,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(x, y, RadarZOrdering::Ships.z_order()),
        Collider::from(shape),
    )
}
