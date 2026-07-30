use crate::attach_observer_on_add;
use crate::game::detection::sensor::*;
use crate::game::gravity::AffectedByGravity;
use crate::game::prelude::*;
use avian2d::prelude::*;

const COLLISION_DAMAGE_FACTOR: f32 = 0.5;
const DEFAULT_HEALTH: i32 = 100;
const MAX_ANGULAR_SPEED: f32 = 20.0;
const ROTATION_DAMPENING: f32 = 2.0;

pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
    fn build(&self, _app: &mut App) {
        _app.add_observer(attach_observer_on_add!(Spaceship, obs_collision));
    }
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

fn obs_collision(
    trigger: On<CollisionStart>,
    mut ship_query: Query<(NameOrEntity, &LinearVelocity, &mut Health), With<Spaceship>>,
) {
    if trigger.body1.is_none() {
        return;
    }
    let ship = trigger.collider1;
    let other_entity = trigger.collider2;
    if let Ok((name, velocity, mut health)) = ship_query.get_mut(ship) {
        let damage: i32 = (COLLISION_DAMAGE_FACTOR * velocity.length()).round() as i32;
        info!("SHIP COLLISION: {name} collided with {other_entity} for {damage} damage");
        health.damage(damage);
    }
}
