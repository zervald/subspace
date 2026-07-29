use crate::game::missile::EventShootMissile;
use crate::game::prelude::*;
use crate::game::radar_camera::CameraFollowMark;
use crate::game::spaceship::*;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::*;
use bevy::{color, prelude::*};

const FULL_STOP_THRESHOLD: f32 = 10.;
const PROPULSION_SPEED: f32 = 50.0;
const ROTATION_SPEED: f32 = 7.5;
const STOP_FACTOR: f32 = 2.;
const WEAPON_FIRE_INTERVAL: f32 = 0.10; // per seconds

#[derive(Resource, Debug, DerefMut, Deref)]
pub struct WeaponTimer(Timer);

#[derive(Component, Debug)]
pub struct PlayerShip;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct CruiseSystems;

pub struct PlayershipPlugin;
impl Plugin for PlayershipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayershipAssets>();

        app.add_systems(
            Update,
            (
                spaceship_propulsion_controls,
                spaceship_rotation_controls,
                // spaceship_shield_controls,
                spaceship_stop_controls,
                spaceship_weapon_controls,
            )
                .in_set(AppSystems::RecordInput)
                .in_set(PausableSystems)
                .in_set(CruiseSystems),
        );

        app.configure_sets(Update, CruiseSystems.run_if(in_state(GameState::Cruising)));

        // .add_systems(
        //     FixedUpdate,
        //     playership_destroyed.run_if(in_state(GameState::Cruising)),
        // )
        app.insert_resource(WeaponTimer(Timer::from_seconds(
            WEAPON_FIRE_INTERVAL,
            TimerMode::Repeating,
        )));
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayershipAssets {
    #[dependency]
    mesh: Handle<Mesh>,
    #[dependency]
    material: Handle<ColorMaterial>,
}

impl FromWorld for PlayershipAssets {
    fn from_world(world: &mut World) -> Self {
        let shape = Triangle2d::new(Vec2::Y * 5.0, vec2(-2.5, -2.5), vec2(2.5, -2.5));
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh_handle = meshes.add(shape);
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let mat_handle = materials.add(Color::from(color::palettes::tailwind::BLUE_600));
        Self {
            mesh: mesh_handle,
            material: mat_handle,
        }
    }
}

pub fn playership(x: f32, y: f32, asset: &PlayershipAssets) -> impl Bundle {
    (
        spaceship(x, y, asset.mesh.clone(), asset.material.clone()),
        PlayerShip,
        Name::new("Serenity"),
        CameraFollowMark,
    )
}

fn spaceship_propulsion_controls(
    query: Single<(Forces, &Transform), With<PlayerShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut forces, transform) = query.into_inner();

    let mut accel = 0.0;
    if keyboard_input.pressed(KeyCode::KeyW) {
        accel = PROPULSION_SPEED;
    }

    let direction = transform.up();

    forces.apply_force(direction.xy() * accel);
}

fn spaceship_stop_controls(
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

fn spaceship_rotation_controls(
    mut angular: Single<&mut AngularVelocity, With<PlayerShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let delta_secs = time.delta_secs();
    let scalar = if keyboard_input.pressed(KeyCode::KeyD) {
        -ROTATION_SPEED
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        ROTATION_SPEED
    } else {
        0.
    };

    angular.0 += scalar * delta_secs;
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
