use crate::game::prelude::*;
use avian2d::prelude::*;
use bevy::{color::palettes::css::*, prelude::*};

const MISSILE_SIZE: f32 = 1.0;
const MISSILE_DAMAGE: i32 = 50;
const PROPULSION_SPEED: f32 = 100.;

#[derive(Component, Debug)]
#[require(GameEntity)]
pub struct Missile {
    pub damage: i32,
}

#[derive(Event)]
pub struct EventShootMissile {
    pub pos: Transform,
    pub vel: LinearVelocity,
}

pub struct MissilePlugin;

impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventShootMissile>();
        app.add_systems(
            FixedUpdate,
            ev_spawn_missile.run_if(in_state(GameState::Cruising)),
        );
    }
}

fn ev_spawn_missile(
    mut event_reader: EventReader<EventShootMissile>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in event_reader.read() {
        let dir = event.pos.up();
        let mut linear_velocity = event.vel;

        linear_velocity.0 += PROPULSION_SPEED * dir.xy();

        commands
            .spawn((
                Missile {
                    damage: MISSILE_DAMAGE,
                },
                RigidBody::Dynamic,
                Collider::rectangle(MISSILE_SIZE, MISSILE_SIZE),
                Mesh2d(meshes.add(Rectangle::new(MISSILE_SIZE, MISSILE_SIZE))),
                MeshMaterial2d(materials.add(Color::from(YELLOW))),
                event.pos,
                linear_velocity,
                CollisionEventsEnabled,
            ))
            .observe(obs_missile_collision);
    }
}

fn obs_missile_collision(
    trigger: Trigger<OnCollisionStart>,
    missile_query: Query<&Missile>,
    mut query: Query<&mut Health>,
    mut commands: Commands,
) {
    if trigger.body.is_none() {
        return;
    }
    let missile_entity = trigger.target();
    let other_entity = trigger.collider;

    if let Ok(mut health) = query.get_mut(other_entity) {
        let damage: i32 = missile_query.get(missile_entity).unwrap().damage;
        info!("MISSILE COLLISION: {missile_entity} collided with {other_entity} for {damage}");
        health.damage(damage);
    }
    commands.entity(missile_entity).try_despawn();
}
