use crate::game::prelude::*;
use avian2d::prelude::*;

pub struct DockingPlugin;
impl Plugin for DockingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(transition_player_to_docking);
        app.add_observer(obs_docking_collision);
    }
}

#[derive(Component, Debug, Default, Clone)]
#[require(CollisionEventsEnabled)]
pub struct Dockable;

#[derive(Event, Debug)]
pub struct DockingStart {
    dock_id: Entity,
    ship_id: Entity,
}

fn transition_player_to_docking(
    event: On<DockingStart>,
    mut next_state: ResMut<NextState<GameState>>,
    playership: Single<Entity, With<PlayerShip>>,
) {
    if event.ship_id == *playership {
        next_state.set(GameState::Docking);
    }
}

const MAX_VEL_DOCKING: f32 = 10.0;

fn obs_docking_collision(
    event: On<CollisionStart>,
    mut commands: Commands,
    dock_query: Query<NameOrEntity, With<Dockable>>,
    ships_query: Query<(NameOrEntity, &LinearVelocity), With<Spaceship>>,
) {
    if let Ok(dock) = dock_query.get(event.collider1)
        && let Ok((ship, ship_velocity)) = ships_query.get(event.collider2)
    {
        info!("DOCKING: {ship} in docking range with dock `{dock}`");
        if ship_velocity.length() < MAX_VEL_DOCKING {
            // Docking
            commands.trigger(DockingStart {
                dock_id: dock.entity,
                ship_id: ship.entity,
            });
        }
    }
}
