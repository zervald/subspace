use crate::{
    game::{playership::playership, prelude::*},
    screens::Screen,
};
use bevy::{color::palettes::css::*, prelude::*};

pub fn spawn_test_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Triangle2d::new(Vec2::Y * 5.0, vec2(-2.5, -2.5), vec2(2.5, -2.5));

    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            // Cloud test
            (playership(&mut meshes, &mut materials)),
            (
                Name::new("Lifetime state"),
                Mesh2d(meshes.add(shape)),
                MeshMaterial2d(materials.add(Color::from(BLUE))),
                Transform::from_xyz(1., -5., RadarZOrdering::Ships.as_f32()),
                // Lifetime::from(GameState::Cruising),
            ),
            (
                Name::new("Lifetime timer"),
                Mesh2d(meshes.add(shape)),
                MeshMaterial2d(materials.add(Color::from(GREEN))),
                Transform::from_xyz(2., -5., 0.),
                // Lifetime::from_secs(5.)
            )
        ],
    ));
}
