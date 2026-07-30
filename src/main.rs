// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]
// NOTE: Allow dead_code during early developement
#![allow(dead_code)]

mod asset_tracking;
mod audio;
mod common;
mod dev_scene;
#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod menus;
mod screens;
mod theme;

use avian2d::{
    PhysicsPlugins,
    dynamics::integrator::Gravity,
    schedule::{Physics, PhysicsTime},
};
use bevy::{asset::AssetMetaCheck, prelude::*};

use crate::game::radar_camera::MainCamera;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Subspace".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    // level: bevy::log::Level::TRACE,
                    // filter: "wgpu=warn,bevy_ecs=info".to_string(),
                    ..default()
                }),
        );
        // TODO: Physics as PausableSystem
        app.add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity::ZERO);

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            audio::AudioPlugin,
            common::CommonPlugin,
            #[cfg(feature = "dev")]
            dev_tools::DevToolsPlugin,
            game::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // pause physics
        app.add_systems(OnEnter(Pause(true)), toggle_physics);
        app.add_systems(OnEnter(Pause(false)), toggle_physics);

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Pause(pub bool);

fn toggle_physics(mut time: ResMut<Time<Physics>>, state: Res<State<Pause>>) {
    let is_paused = state.get().0;
    match is_paused {
        true => time.pause(),
        false => time.unpause(),
    }
}

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d, MainCamera));
}
