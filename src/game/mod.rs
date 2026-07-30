use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        asteroids::AsteroidPlugin,
        cloud::CloudPlugin,
        detection::DetectionPlugin,
        docking::DockingPlugin,
        gravity::GravityPlugin,
        health::HealthPlugin,
        effects::EffectsPlugin,
        missile::MissilePlugin,
        planet::PlanetPlugin,
        gamestate::GameStatePlugin,
        radar_camera::CameraPlugin,
        playership::PlayershipPlugin,
        spaceship::SpaceshipPlugin,
        station::StationPlugin,
    ));
}

#[derive(Component, Default)]
pub struct GameEntity;

pub mod asteroids;
pub mod cloud;
pub mod collision;
pub mod damage;
pub mod detection;
pub mod docking;
pub mod effects;
pub mod gamestate;
pub mod gravity;
pub mod health;
pub mod missile;
pub mod planet;
pub mod playership;
pub mod prelude;
pub mod radar_camera;
pub mod spaceship;
pub mod station;
pub mod z_ordering;
