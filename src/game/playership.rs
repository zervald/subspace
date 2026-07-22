use crate::game::missile::EventShootMissile;
use crate::game::prelude::*;
use crate::game::radar_camera::FlagCameraFollow;
use crate::game::spaceship::*;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::*;
use bevy::prelude::*;

const FULL_STOP_THRESHOLD: f32 = 10.;
const PROPULSION_SPEED: f32 = 50.0;
const ROTATION_SPEED: f32 = 7.5;
const STOP_FACTOR: f32 = 2.;
const WEAPON_FIRE_INTERVAL: f32 = 0.10; // per seconds

#[derive(Resource, Debug, DerefMut, Deref)]
pub struct WeaponTimer(Timer);

#[derive(Component, Debug)]
pub struct PlayerShip {
    pub name: String,
}

pub struct PlayershipPlugin;
impl Plugin for PlayershipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spaceship_propulsion_control,
                spaceship_rotation_control,
                // spaceship_shield_controls,
                spaceship_stop,
                spaceship_weapon_controls,
            )
                .chain()
                .run_if(in_state(GameState::Cruising))
                .in_set(AppSystems::RecordInput)
                .in_set(PausableSystems),
        )
        // .add_systems(
        //     FixedUpdate,
        //     playership_destroyed.run_if(in_state(GameState::Cruising)),
        // )
        .insert_resource(WeaponTimer(Timer::from_seconds(
            WEAPON_FIRE_INTERVAL,
            TimerMode::Repeating,
        )));
    }
}

pub fn playership(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    let z = RadarZOrdering::Ships.z_order();
    (
        spaceship(meshes, materials),
        PlayerShip {
            name: "Serenity".into(),
        },
        Name::new("Playership"),
        FlagCameraFollow,
        Transform::from_xyz(0., 0., z),
    )
}

fn spaceship_propulsion_control(
    query: Single<(&mut LinearVelocity, &Transform), With<PlayerShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut linear_v, transform) = query.into_inner();

    let mut accel = 0.0;
    if keyboard_input.pressed(KeyCode::KeyW) {
        accel = PROPULSION_SPEED;
    }

    let direction = transform.up();

    linear_v.0 += direction.xy() * accel * time.delta_secs();
}

fn spaceship_stop(
    query: Single<(&mut LinearVelocity, &mut Transform), With<PlayerShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut vel, mut transform) = query.into_inner();

    if keyboard_input.pressed(KeyCode::ControlLeft) {
        if vel.length() < FULL_STOP_THRESHOLD {
            vel.0 = Vec2::ZERO;
        }
        let reverse_vec = vel.0 * -1.;
        vel.0 += reverse_vec * STOP_FACTOR * time.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::Enter) {
        transform.translation = Vec3::ZERO;
        vel.x = 0.0;
        vel.y = 0.0;
    }
}

fn spaceship_rotation_control(
    mut angular: Single<&mut AngularVelocity, With<PlayerShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let delta_secs = time.delta_secs();
    let mut rotation_direction = 0.0;
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation_direction = -1.;
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        rotation_direction = 1.;
    }

    angular.0 += ROTATION_SPEED * rotation_direction * delta_secs;
}

fn spaceship_weapon_controls(
    query: Single<(&Transform, &LinearVelocity), With<PlayerShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event: MessageWriter<EventShootMissile>,
    mut weapon_timer: ResMut<WeaponTimer>,
    time: Res<Time>,
) {
    weapon_timer.tick(time.delta());
    if !weapon_timer.is_finished() {
        return;
    }

    let (ship_transform, ship_velocity) = query.into_inner();
    let dir = ship_transform.up();

    let mut new_trans = *ship_transform;
    new_trans.translation.x += 10. * dir.x;
    new_trans.translation.y += 10. * dir.y;

    if keyboard_input.pressed(KeyCode::Space) {
        event.write(EventShootMissile {
            pos: new_trans,
            vel: *ship_velocity,
        });
    }
}

#[allow(dead_code)]
fn spaceship_shield_controls(
    mut commands: Commands,
    playership: Single<Entity, With<PlayerShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(*playership).try_insert_if_new(ShipShield {
            strength: 100.,
            active: true,
        });
    }
}
