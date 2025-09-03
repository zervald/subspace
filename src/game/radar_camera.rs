use crate::game::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(Camera2d, GameEntity)]
pub struct MainCamera;

#[derive(Component)]
pub struct FollowCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            camera_follow.run_if(in_state(GameState::Cruising)),
        );
        app.add_systems(PreUpdate, zoom_camera.run_if(in_state(GameState::Cruising)));
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

fn camera_follow(
    mut camera_query: Single<&mut Transform, With<MainCamera>>,
    follow_query: Single<&Transform, (With<FollowCamera>, Without<MainCamera>)>,
) {
    // TODO: Make screen refresh effect
    camera_query.translation = follow_query.translation;
}

fn zoom_camera(
    camera_query: Single<&mut Projection, With<MainCamera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut projection = camera_query.into_inner();
    // Camera zoom controls
    if let Projection::Orthographic(projection2d) = &mut *projection {
        if input.just_pressed(KeyCode::Equal) {
            projection2d.scale *= 0.5f32;
        }

        if input.just_pressed(KeyCode::Minus) {
            projection2d.scale *= 2.0f32;
        }
    }
}
