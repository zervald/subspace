use bevy::ecs::observer::IntoObserver;
use bevy::ecs::{lifecycle::HookContext, observer::IntoEntityObserver, world::DeferredWorld};
use bevy::prelude::*;

#[macro_export]
macro_rules! attach_observer_on_add {
    ($component:ty, $observer:expr) => {
        |trigger: On<Add, $component>, mut commands: Commands| {
            if let Ok(mut ec) = commands.get_entity(trigger.entity) {
                ec.observe($observer);
            }
        }
    };
}
