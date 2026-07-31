use bevy::prelude::*;
use bevy_alchemy::{Delay, Effecting};

use crate::game::detection::sensor::PassiveSensor;

#[derive(Component, Debug, Clone)]
#[require(Delay)]
pub struct Obscured {
    magnitude: i32,
}

fn obscure_passive_sensors(
    effects: Query<(&Effecting, &Delay, &Obscured)>,
    mut targets: Query<&mut PassiveSensor>,
) {
    for (target, delay, obscurement) in effects {
        // We wait until the delay finishes to apply the damage.
        if !delay.timer.is_finished() {
            continue;
        }

        // Skip if the target doesn't have health.
        let Ok(mut passive_sensor) = targets.get_mut(target.0) else {
            continue;
        };

        // Otherwise, deal the damage.
        passive_sensor.current_power -= obscurement.magnitude;
    }
}
