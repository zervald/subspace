use crate::game::prelude::*;
use bevy::prelude::*;

const DETECTION_MIN: f32 = 0.01;

pub struct SensorPlugin;
impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventDetection>();
        app.add_event::<Ping>();
        app.add_systems(
            FixedUpdate,
            (passive_sensor).run_if(in_state(GameState::Cruising)),
        );
        app.add_systems(
            Update,
            temp_passible_toggle.run_if(in_state(GameState::Cruising)),
        );
    }
}

// Sensors
#[derive(Component, Debug)]
#[require(DetectedContacts)]
pub struct ActiveSensor {
    power: i32,
    time_interval: f32,
}

#[derive(Component, Debug)]
#[require(DetectedContacts)]
pub struct PassiveSensor {
    active: bool,
    power: i32,
}

impl Default for PassiveSensor {
    fn default() -> Self {
        Self {
            active: true,
            power: 10,
        }
    }
}

// Events
#[derive(Event)]
pub struct Ping {
    origin_entity: Entity,
    origin_sensor: ActiveSensor,
}

fn passive_sensor(
    p_sensor_query: Query<(Entity, &PassiveSensor)>,
    detection_query: Query<(Entity, &Emission)>,
    mut event: EventWriter<EventDetection>,
) {
    if detection_query.is_empty() || p_sensor_query.is_empty() {
        return;
    }

    let confidence_calc = |a: f32, b: f32| (a - b) / a;

    for (sensor_entity, sensor) in p_sensor_query {
        if !sensor.active {
            continue;
        }
        for (detected_entity, detection) in detection_query {
            if detected_entity == sensor_entity {
                continue;
            }
            // TODO: raycast to include "obstacles"
            let confidence: f32 = confidence_calc(sensor.power as f32, **detection as f32);
            if confidence > DETECTION_MIN {
                event.write(EventDetection {
                    detector: sensor_entity,
                    target: detected_entity,
                    confidence,
                });
            }
        }
    }
}

#[allow(dead_code, unused_variables)]
fn sensor_ping(
    mut event: EventReader<Ping>,
    p_sensor_query: Query<(Entity, &PassiveSensor)>,
    detection_query: Query<(Entity, &Emission)>,
) {
    for ping in event.read() {
        todo!();
    }
}

fn temp_passible_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut sensor: Single<&mut PassiveSensor, With<PlayerShip>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        sensor.active = !sensor.active
    }
}
