use crate::game::detection::sensor::*;
use crate::game::gravity::AffectedByGravity;
use crate::game::prelude::*;
use avian2d::prelude::*;
use bevy::color::palettes::css::*;

const COLLISION_DAMAGE_FACTOR: f32 = 0.5;
const DEFAULT_HEALTH: i32 = 100;
const MAX_ANGULAR_SPEED: f32 = 20.0;
const ROTATION_DAMPENING: f32 = 2.0;

pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
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

pub fn spawn_test_ennemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Triangle2d::new(Vec2::Y * 5.0, vec2(-2.5, -2.5), vec2(2.5, -2.5));
    let mesh = meshes.add(shape);
    let material = materials.add(Color::from(RED));
    commands.spawn(spaceship(50.0, 50., mesh, material));
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
