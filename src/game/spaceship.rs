use crate::game::detection::sensor::*;
use crate::game::gravity::GravityAffected;
use crate::game::prelude::*;
use avian2d::prelude::*;
use bevy::{color::palettes::css::*, prelude::*};
use log::info;

const COLLISION_DAMAGE_FACTOR: f32 = 0.5;
const DEFAULT_HEALTH: i32 = 100;
const MAX_ANGULAR_SPEED: f32 = 15.0;
const ROTATION_DAMPENING: f32 = 2.0;

pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Debug)]
#[require(GameEntity, Emission, RigidBody::Dynamic)]
pub struct Spaceship;

#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct ShipShield {
    pub strength: f32,
    pub active: bool,
}

pub fn spawn_spaceship(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let shape = Triangle2d::new(Vec2::Y * 5.0, vec2(-2.5, -2.5), vec2(2.5, -2.5));
    commands
        .spawn((
            Spaceship,
            AngularDamping(ROTATION_DAMPENING),
            Collider::from(shape),
            CollisionEventsEnabled,
            GravityAffected,
            Health(DEFAULT_HEALTH),
            MaxAngularSpeed(MAX_ANGULAR_SPEED),
            Mesh2d(meshes.add(shape)),
            MeshMaterial2d(materials.add(Color::from(BLUE))),
            PassiveSensor::default(),
            RigidBody::Dynamic,
        ))
        // .observe(observer_collision())
        .id()
}

pub fn spawn_test_ennemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Triangle2d::new(Vec2::Y * 5.0, vec2(-2.5, -2.5), vec2(2.5, -2.5));
    commands.spawn((
        Spaceship,
        AngularDamping(ROTATION_DAMPENING),
        Collider::from(shape),
        CollisionEventsEnabled,
        GravityAffected,
        Health(DEFAULT_HEALTH),
        MaxAngularSpeed(MAX_ANGULAR_SPEED),
        Mesh2d(meshes.add(shape)),
        MeshMaterial2d(materials.add(Color::from(RED))),
        PassiveSensor::default(),
        RigidBody::Dynamic,
        Transform::from_xyz(50., 50., RadarOrdering::ZShips.as_f32()),
    ));
}

#[allow(dead_code)]
fn observer_collision(
    trigger: Trigger<OnCollisionStart>,
    mut ship_query: Query<(&LinearVelocity, &mut Health), With<Spaceship>>,
) {
    if trigger.body.is_none() {
        return;
    }
    let ship = trigger.target();
    let other_entity = trigger.collider;
    if let Ok((velocity, mut health)) = ship_query.get_mut(ship) {
        let damage: i32 = (COLLISION_DAMAGE_FACTOR * velocity.length()).round() as i32;
        info!("SHIP COLLISION: {ship} collided with {other_entity} for {damage}");
        health.damage(damage);
    }
}
