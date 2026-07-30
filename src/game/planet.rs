use crate::game::prelude::*;
use avian2d::prelude::*;
use bevy::prelude::*;

const ROTATE_SPEED: f32 = 1.0;

#[derive(Component, Debug)]
#[require(GameEntity, CollisionEventsEnabled)]
pub struct Planet;

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            // TODO: visual, so rotation is visible
            (rotate_planets).run_if(|| false),
        );
    }
}

pub fn obs_planet_collision(
    activate: On<CollisionStart>,
    planet_query: Query<NameOrEntity, (With<CollisionEventsEnabled>, With<Planet>)>,
    mut query: Query<(NameOrEntity, &mut Health), With<CollisionEventsEnabled>>,
) {
    if activate.body1.is_none() {
        return;
    }
    let planet_entity = activate.collider1;
    let other_entity = activate.collider2;

    if let Ok((other_name, mut health)) = query.get_mut(other_entity) {
        let Ok(planet) = planet_query.get(planet_entity) else {
            return;
        };
        info!("PLANET COLLISION: {other_name} collided with planet {planet}");
        **health = 0;
    };
}

// NOTE: velocity calc by distance from star?
fn rotate_planets(mut query: Query<&mut Transform, With<Planet>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_secs());
    }
}
