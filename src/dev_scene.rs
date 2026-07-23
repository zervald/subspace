use crate::{
    game::playership::{PlayershipAssets, playership},
    screens::Screen,
};
use bevy::{color::palettes::css::*, prelude::*};

pub fn spawn_test_level(
    mut commands: Commands,
    playership_asset: Res<PlayershipAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Rectangle::new(5., 5.);
    let ship = playership(&playership_asset);

    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            // Cloud test
            (ship),
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
            )
        ],
    ));
}
