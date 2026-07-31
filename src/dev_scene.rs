use crate::game::docking::Dockable;
use crate::game::gravity::GravitySource;
use crate::game::planet::obs_planet_collision;
use crate::game::station::{Station, station};
use crate::{
    game::{
        planet::Planet,
        playership::{PlayershipAssets, playership},
        z_ordering::RadarZOrdering,
    },
    screens::Screen,
};
use avian2d::collision::collider::Collider;
use avian2d::collision::collision_events::CollisionEventsEnabled;
use avian2d::dynamics::rigid_body::RigidBody;
use bevy::color::palettes::tailwind::BLUE_400;
use bevy::ui_widgets::observe;
use bevy::{color::palettes::css::*, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_test_level);
}

fn dev_dock() -> impl Scene {
    bsn! {
        #dev_dock
        Name::new("Pebble dock")
        Dockable
        Collider::circle(2. + 5.)
    }
}

fn dev_station() -> impl Scene {
    bsn! {
        #dev_station
        Station
        Collider::circle(2.)
        Mesh2d(asset_value(Circle::new(2.)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::from(GREEN)))
        template_value(RigidBody::Static)
        CollisionEventsEnabled
        Transform::from_xyz(-50., 0., RadarZOrdering::Planet.z_order())
        Children [dev_dock()]
    }
}

fn spawn_test_level(
    mut commands: Commands,
    playership_asset: Res<PlayershipAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Rectangle::new(5., 5.);

    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::Visible,
        DespawnOnExit(Screen::Gameplay),
        children![
            // Cloud test
            (playership(0., 0., &playership_asset)),
            (
                Name::new("Lifetime state"),
                Mesh2d(meshes.add(shape)),
                MeshMaterial2d(materials.add(Color::from(YELLOW))),
                Transform::from_xyz(6., -10., 0.),
                // Lifetime::from(GameState::Cruising),
            ),
            (
                Name::new("Lifetime timer"),
                Mesh2d(meshes.add(shape)),
                MeshMaterial2d(materials.add(Color::from(GREEN))),
                Transform::from_xyz(12., -10., 0.),
                // Lifetime::from_secs(5.)
            ),
            test_planet(meshes, materials),
        ],
    ));
}

fn test_planet(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    let station_mesh = Mesh2d(meshes.add(Circle::new(2.)));
    let station_material = MeshMaterial2d(materials.add(Color::from(GREEN)));
    const STATION_RADIUS: f32 = 10.;

    let dock = (
        Name::new("Pebble dock"),
        Dockable,
        Collider::circle(STATION_RADIUS + 5.),
    );

    let station = (
        Name::new("Pebble"),
        station(-50.0, 0., station_mesh, station_material, 1.),
        children![dock],
    );

    (
        Name::new("Rock"),
        Planet,
        Collider::circle(STATION_RADIUS),
        Mesh2d(meshes.add(Circle::new(STATION_RADIUS))),
        MeshMaterial2d(materials.add(Color::from(BLUE_400))),
        GravitySource::default(),
        RigidBody::Static,
        Transform::from_xyz(100., 100., RadarZOrdering::Planet.z_order()),
        observe(obs_planet_collision),
        children![station,],
    )
}
