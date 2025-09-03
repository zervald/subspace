use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
// use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

pub mod common;
pub mod game;
pub mod ui;

// plugins
use common::CommonPlugin;
use game::GamePlugin;
use ui::UIPlugin;

mod test_scene;
use crate::test_scene::TestScenePlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::srgb(0.0, 0.05, 0.05)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1.0,
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sub-Space".to_string(),
                resolution: WindowResolution::new(1920., 1080.),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        // egui
        // .add_plugins(EguiPlugin::default())
        // mu plugins
        .add_plugins((
            PhysicsPlugins::default(),
            CommonPlugin,
            GamePlugin,
            UIPlugin,
            TestScenePlugin,
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .run();
}

// fn spawn_level(commands: Commands) {
//     commands.spawn()
// }
