#![allow(dead_code)]
use crate::common::state::AppState;
use crate::game::gravity::GravitySource;
use avian2d::prelude::CollisionStarted;
use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
    render::diagnostic::RenderDiagnosticsPlugin,
};
use iyes_perf_ui::prelude::*;
// use crate::schedule::InGameSet;

const GRAVITY_DUI: bool = false;

#[derive(Component)]
struct DebugUI;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            // Perf ui
            app.add_plugins(FrameTimeDiagnosticsPlugin::default())
                .add_plugins(EntityCountDiagnosticsPlugin)
                // NOTE: SystemInformationDiagnosticsPlugin not available while dynamic libs enabled @see Cargo.toml
                .add_plugins(SystemInformationDiagnosticsPlugin)
                .add_plugins(RenderDiagnosticsPlugin)
                .add_plugins(PerfUiPlugin)
                .add_systems(Startup, spawn_perf_ui)
            // Custom
                // .add_systems(EguiPrimaryContextPass, ui_example_system)
                .add_systems(Update, gravity_dui.run_if(in_state(AppState::InGame).and(|| GRAVITY_DUI)))
                .add_systems(PreUpdate, despawn_dui)
            // .add_systems(Update, print_collisions);
            // .add_systems(Update, print_position.after(InGameSet::EntityUpdates))
            ;
        }
    }
}

fn gravity_dui(
    mut commands: Commands,
    query: Query<(Entity, &GravitySource)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, gravity_source) in query {
        commands.entity(entity).with_child((
            DebugUI,
            Transform::from_xyz(0., 0., 1.),
            Mesh2d(meshes.add(Circle::new(gravity_source.reach))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Srgba {
                red: 0.5,
                green: 0.0,
                blue: 0.5,
                alpha: 0.5,
            }))),
        ));
    }
}

fn despawn_dui(mut commands: Commands, query: Query<Entity, With<DebugUI>>) {
    for entity in query {
        commands.entity(entity).try_despawn();
    }
}

fn spawn_perf_ui(mut commands: Commands) {
    // commands.spawn(PerfUiAllEntries::default());
    commands.spawn(PerfUiDefaultEntries::default());
}

fn print_collisions(mut collision_event_reader: EventReader<CollisionStarted>) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        println!("Entities {entity1} and {entity2} are colliding");
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    // Log the entity ID and translation of each entity with a `Position` component.
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at position {:?},",
            entity, transform.translation
        );
    }
}
