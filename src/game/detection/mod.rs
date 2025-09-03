use std::ops::DerefMut;

use crate::game::prelude::*;
use bevy::{platform::collections::HashMap, prelude::*};

const FULL_DETECTION_MIN: f32 = 0.5;

pub struct DetectionPlugin;
impl Plugin for DetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((sensor::SensorPlugin, contacts::ContactPlugin));
        app.add_event::<EventDetection>();
        app.add_systems(
            FixedUpdate,
            (detected_clean, detected_update)
                .chain()
                .run_if(in_state(GameState::Cruising)),
        );
        app.add_systems(
            Update,
            (player_fully_detected_remove, player_fully_detected_add)
                .chain()
                .run_if(in_state(GameState::Cruising)),
        );
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct DetectedContacts(pub HashMap<Entity, f32>);

#[derive(Event)]
pub struct EventDetection {
    pub detector: Entity,
    pub target: Entity,
    pub confidence: f32,
}

fn detected_update(
    mut query: Query<&mut DetectedContacts>,
    mut event_reader: EventReader<EventDetection>,
) {
    for event in event_reader.read() {
        let Ok(mut contacts) = query.get_mut(event.detector) else {
            continue;
        };
        contacts.deref_mut().insert(event.target, event.confidence);
    }
}

fn detected_clean(mut query: Query<&mut DetectedContacts>) {
    query.iter_mut().for_each(|mut contacts| {
        contacts.clear();
    });
}

fn player_fully_detected_add(
    contacts: Single<&DetectedContacts, With<PlayerShip>>,
    mut commands: Commands,
) {
    for (entity, value) in contacts.iter() {
        // TODO: change const to some equipment determining when confidence is enough for realtime position
        if *value >= FULL_DETECTION_MIN {
            commands.entity(*entity).try_insert(Visibility::Visible);
        }
    }
}

fn player_fully_detected_remove(
    query: Query<Entity, (With<Emission>, Without<PlayerShip>)>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).try_insert(Visibility::Hidden);
    }
}

pub mod contacts;
pub mod emission;
pub mod sensor;
