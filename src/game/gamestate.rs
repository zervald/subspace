use crate::common::state::AppState;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Cruising,
    Docking,
    Jumping,
    Destroyed,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(
                OnEnter(AppState::InGame),
                |mut ns: ResMut<NextState<GameState>>| ns.set(GameState::Cruising),
            )
            .add_systems(OnEnter(GameState::Destroyed), transition_to_gameover);
    }
}

fn transition_to_gameover(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::GameOver);
}
