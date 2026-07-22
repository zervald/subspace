use crate::{AppSystems, PausableSystems, game::z_ordering::RadarZOrdering};
use bevy::prelude::*;

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraFollowMark;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_follow);
        app.add_systems(
            Update,
            zoom_camera
                .in_set(AppSystems::RecordInput)
                .in_set(PausableSystems),
        );
    }
}

fn camera_follow(
    mut main_camera: Single<&mut Transform, With<MainCamera>>,
    followe: Single<&Transform, (With<CameraFollowMark>, Without<MainCamera>)>,
) {
    // TODO: Make screen refresh effect
    let (x, y) = (followe.translation.x, followe.translation.y);
    main_camera.translation = vec3(x, y, 1.);
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
