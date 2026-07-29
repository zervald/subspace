use crate::game::{cloud::Cloud, effect::effect_event::EventAddEffect};
use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(cloud_enter);
}

#[derive(Component)]
pub struct AffectedByCloud;

fn cloud_enter(
    trigger: On<CollisionStart>,
    mut commands: Commands,
    cloud_query: Query<(NameOrEntity, &Cloud), With<CollisionEventsEnabled>>,
    other_query: Query<NameOrEntity, With<AffectedByCloud>>,
) {
    let Ok((cloud_name, cloud)) = cloud_query.get(trigger.collider1) else {
        return;
    };
    let Ok(other) = other_query.get(trigger.collider2) else {
        return;
    };

    info!("`{other}` collided with Cloud `{cloud_name}`");
    commands.trigger(EventAddEffect::new(
        cloud_name.entity,
        other.entity,
        None,
        cloud.effects.clone(),
    ));
}

// TODO: find solution with lifetime,
// maybe lifetime::event(T: Event) ?
// fn cloud_exit(
//     trigger: On<CollisionEnd>,
//     mut commands: Commands,
//     cloud_query: Query<&EffectSource, With<Cloud>>,
// ) {
//     let effect_entity = match cloud_query.get(trigger.event_target()) {
//         Ok(e) => **e,
//         Err(_) => return,
//     };
//     if let Ok(mut ec) = commands.get_entity(effect_entity) {
//         ec.try_despawn();
//     }
// }
