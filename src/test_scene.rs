use crate::{
    common::{prelude::Lifetime, state::AppState},
    game::{
        cloud::{Cloud, CloudBundle},
        planet::spawn_test_planet,
        playership::spawn_playership,
        prelude::*,
        radar_camera::spawn_camera,
        spaceship::spawn_test_ennemy,
    },
};
use bevy::{color::palettes::css::*, prelude::*};

pub struct TestScenePlugin;
impl Plugin for TestScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_test_planet);
        app.add_systems(OnEnter(AppState::InGame), spawn_playership);
        app.add_systems(OnEnter(AppState::InGame), spawn_camera);
        app.add_systems(OnEnter(AppState::InGame), spawn_test_ennemy);
        app.add_systems(OnEnter(AppState::InGame), spawn_test_cloud);
        app.add_systems(OnEnter(AppState::InGame), spawn_test_lifetime);
    }
}

fn spawn_test_cloud(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    CloudBundle::spawn(
        Cloud::Thunderstorm,
        -10.0,
        -10.0,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}

fn spawn_test_lifetime(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Triangle2d::new(Vec2::Y * 5.0, vec2(-2.5, -2.5), vec2(2.5, -2.5));
    let id = commands
        .spawn((
            Name::new("Lifetime timer"),
            Mesh2d(meshes.add(shape)),
            MeshMaterial2d(materials.add(Color::from(RED))),
            Lifetime::from_secs(5.),
            Transform::from_xyz(0., -50., RadarOrdering::ZShips.as_f32()),
        ))
        .id();

    commands.spawn((
        Name::new("Lifetime state"),
        Mesh2d(meshes.add(shape)),
        MeshMaterial2d(materials.add(Color::from(BLUE))),
        Lifetime::from(GameState::Cruising),
        Transform::from_xyz(10., -50., RadarOrdering::ZShips.as_f32()),
    ));

    commands.spawn((
        Name::new("Lifetime entité"),
        Mesh2d(meshes.add(shape)),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        Lifetime::from(id),
        Transform::from_xyz(20., -50., RadarOrdering::ZShips.as_f32()),
    ));
}
