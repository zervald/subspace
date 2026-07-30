use crate::game::prelude::*;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::*;

pub(super) fn plugin(_app: &mut App) {
    _app.add_systems(
        Update,
        crash_damage
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone)]
pub struct CrashResistance(f32);

impl Default for CrashResistance {
    fn default() -> Self {
        // NOTE: total total impulse threshold before getting damaged
        Self(5.0)
    }
}

impl CrashResistance {
    pub const fn get(&self) -> f32 {
        self.0
    }
}

fn crash_damage(
    query: Query<(NameOrEntity, Option<&CrashResistance>), With<Health>>,
    collisions: Collisions,
    mut commands: Commands,
) {
    let mut total_impulse = 0.0;

    for (entity, resistance) in &query {
        for contact_pair in collisions.collisions_with(entity.entity) {
            total_impulse += contact_pair.total_normal_impulse_magnitude();
        }

        let threshold = match resistance {
            Some(r) => r.0,
            None => CrashResistance::default().0,
        };

        info!(
            "COLLISION: #{entity} total_impulse / threshold : {} / {}",
            total_impulse, threshold,
        );

        if total_impulse > threshold {
            commands.trigger(TakeDamage {
                entity: entity.entity,
                damage: total_impulse as i32,
            });
        }
    }
}

// TODO:
// #[derive(PhysicsLayer, Default)]
// enum SpaceLayer {
//     #[default]
//     Default, // Layer 0 - the default layer that objects are assigned to
//     Ships,
//     DamagingObjects,
//     Sensor,
// }
//
// pub fn ship_layer() -> CollisionLayers {
//     CollisionLayers::new(
//         SpaceLayer::Ships,
//         [
//             SpaceLayer::Default,
//             SpaceLayer::Ships,
//             SpaceLayer::DamagingObjects,
//         ],
//     )
// }
//
