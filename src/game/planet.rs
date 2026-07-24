use crate::game::gravity::GravitySource;
use crate::game::prelude::*;
use avian2d::prelude::*;
use bevy::{
    color::palettes::{css::GREEN, tailwind::BLUE_400},
    prelude::*,
    ui_widgets::observe,
};

const ROTATE_SPEED: f32 = 1.0;
const SIZE_RADIUS: f32 = 10.;

#[derive(Component, Debug)]
#[require(GameEntity, CollisionEventsEnabled, RigidBody::Static)]
pub struct Planet;

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            // WARN: State should be cruising
            (rotate_planets).run_if(|| false),
        );
    }
}

pub fn test_planet(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    // NOTE: velocity calc by distance from star?

    (
        Name::new("Rock"),
        Planet,
        Collider::circle(SIZE_RADIUS),
        CollisionEventsEnabled,
        Mesh2d(meshes.add(Circle::new(SIZE_RADIUS))),
        MeshMaterial2d(materials.add(Color::from(BLUE_400))),
        GravitySource::default(),
        RigidBody::Static,
        Transform::from_xyz(100., 100., RadarZOrdering::Planet.z_order()),
        observe(obs_planet_collision),
        children![(
            Station {
                name: "Pebble".into(),
            },
            Collider::circle(2.),
            Mesh2d(meshes.add(Circle::new(2.))),
            MeshMaterial2d(materials.add(Color::from(GREEN))),
            RigidBody::Static,
            Transform::from_xyz(-50., 0., RadarZOrdering::Planet.z_order()),
        )],
    )
}

fn obs_planet_collision(
    activate: On<CollisionStart>,
    planet_query: Query<&Name, (With<CollisionEventsEnabled>, With<Planet>)>,
    mut query: Query<&mut Health, With<CollisionEventsEnabled>>,
) {
    if activate.body1.is_none() {
        return;
    }
    let planet_entity = activate.collider1;
    let other_entity = activate.collider2;

    if let Ok(mut health) = query.get_mut(other_entity) {
        let name: String = match planet_query.get(planet_entity) {
            Ok(n) => n.to_string(),
            Err(_) => "No name".into(),
        };
        info!("PLANET COLLISION: {other_entity} collided with planet {name}");
        **health = 0;
    };
}

#[allow(dead_code)]
fn rotate_planets(mut query: Query<&mut Transform, With<Planet>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_secs());
    }
}
