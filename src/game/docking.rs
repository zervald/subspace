use crate::game::prelude::*;
use bevy::prelude::*;

pub struct DockingPlugin;
impl Plugin for DockingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventDockingStart>()
            .add_systems(FixedUpdate, transition_to_docking);
    }
}

#[derive(Event, Debug)]
pub struct EventDockingStart {
    ship_id: Entity,
    station_id: Entity,
}

fn transition_to_docking(
    event: EventReader<EventDockingStart>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if event.is_empty() {
        return;
    }
    next_state.set(GameState::Docking);
}
