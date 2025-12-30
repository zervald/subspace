use crate::game::gravity::GravityAffected;
use crate::game::prelude::*;
use avian2d::prelude::*;
use bevy::{color::palettes::css::*, prelude::*};
use rand::prelude::*;
use std::ops::Range;

const HEALTH: i32 = 50;
const ROTATE_SPEED: f32 = 2.5;
const SIZE_RADIUS: f32 = 2.5;
const SPAWN_LIMIT: usize = 100;
const SPAWN_RANGE_X: Range<f32> = -800.0..800.0;
const SPAWN_RANGE_Y: Range<f32> = -450.0..450.0;
const VELOCITY_SCALAR: f32 = 5.0;
#[allow(dead_code)]
const SPAWN_TIME_SECONDS: f32 = 0.;

#[derive(Component, Debug)]
#[require(GameEntity)]
pub struct Asteroid;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (rotate_asteroids).run_if(in_state(GameState::Cruising)),
        );
    }
}

fn spawn_batch_asteroid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let random_unit_vector = || -> Vec2 {
    //     let mut rng = rand::rng();
    //     Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0)).normalize_or_zero()
    // };
    // let velocity = || random_unit_vector() * VELOCITY_SCALAR;

    let pos = || -> Vec3 {
        let mut rng = rand::rng();
        Vec3::new(
            rng.random_range(SPAWN_RANGE_X),
            rng.random_range(SPAWN_RANGE_Y),
            0.,
        )
    };

    for _ in 0..SPAWN_LIMIT {
        commands.spawn((
            Asteroid,
            Health(HEALTH),
            Mesh2d(meshes.add(Circle::new(SIZE_RADIUS))),
            MeshMaterial2d(materials.add(Color::from(GREY))),
            // LinearVelocity(velocity()),
            Transform::from_translation(pos()),
            RigidBody::Static,
            Collider::circle(SIZE_RADIUS),
            children![(
                Mesh2d(meshes.add(Circle::new(SIZE_RADIUS / 3.))),
                MeshMaterial2d(materials.add(Color::from(DARK_SLATE_GREY))),
                Transform::default().with_translation(Vec3::new(1.1, 1.1, 1.)),
            )],
        ));
    }
}

#[allow(dead_code)]
fn spawn_asteroid(
    mut commands: Commands,
    // mut spawn_timer: ResMut<SpawnTimer>,
    // time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Asteroid>,
    // scene_assets: Res<AssetLibrary>,
) {
    // spawn_timer.timer.tick(time.delta());
    // if !spawn_timer.timer.just_finished() {
    //     return;
    // }

    if query.iter().len() >= SPAWN_LIMIT {
        return;
    }

    let mut rng = rand::rng();

    let pos = Vec3::new(
        rng.random_range(SPAWN_RANGE_X),
        rng.random_range(SPAWN_RANGE_Y),
        0.,
    );

    let transform = Transform::from_translation(pos);

    let mut random_unit_vector =
        || Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    // let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands
        .spawn((
            Asteroid,
            Health(HEALTH),
            Mesh2d(meshes.add(Circle::new(SIZE_RADIUS))),
            MeshMaterial2d(materials.add(Color::from(GREY))),
            LinearVelocity(velocity),
            transform,
            RigidBody::Kinematic,
            Collider::circle(SIZE_RADIUS),
            GravityAffected,
            children![(
                Mesh2d(meshes.add(Circle::new(SIZE_RADIUS / 3.))),
                MeshMaterial2d(materials.add(Color::from(DARK_SLATE_GREY))),
                Transform::default().with_translation(Vec3::new(1.1, 1.1, 1.)),
            )],
        ))
        .observe(obs_astroid_collision);
}

// TODO: Change to angular velocity
fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_secs());
    }
}

fn obs_astroid_collision(trigger: Trigger<OnCollisionStart>, mut query: Query<&mut Health>) {
    if trigger.body.is_none() {
        return;
    }
    let astroid_entity = trigger.target();
    let other_entity = trigger.collider;

    if let Ok(mut health) = query.get_mut(other_entity) {
        debug!("ASTEROID COLLISION: {other_entity} collided with {astroid_entity}");
        **health = 0;
    }
}
