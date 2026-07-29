use crate::game::prelude::*;
use avian2d::prelude::*;

pub struct DockingPlugin;
impl Plugin for DockingPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MsgDockingStart>();
    }
}

#[derive(Message, Debug)]
pub struct MsgDockingStart {
    ship_id: Entity,
    dock_id: Entity,
}

fn transition_to_docking(
    msg: MessageReader<MsgDockingStart>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if msg.is_empty() {
        return;
    }
    next_state.set(GameState::Docking);
}

const MAX_VEL_DOCKING: f32 = 10.0;

fn obs_docking_collision(
    trigger: On<CollisionStart>,
    station_query: Query<(NameOrEntity, &Station)>,
    mut ships_query: Query<(&mut LinearVelocity, &mut Health), With<Spaceship>>,
) {
    if trigger.body1.is_none() {
        return;
    }
    let station_entity = trigger.collider1;
    let other_entity = trigger.collider2;

    if let Ok((vel, _health)) = ships_query.get_mut(other_entity)
        && let Ok((name, _station)) = &station_query.get(station_entity)
    {
        info!("DOCKING: {other_entity} collided with station `{name}`");
        if vel.length() < MAX_VEL_DOCKING {
            //Docking
            // TODO:
            // event.write(EventDocking {
            //     ship_id: other_entity,
            //     station_id: station_entity,
            // });
        }
    }
}
