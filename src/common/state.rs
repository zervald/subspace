use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    InGame,
    InMenu,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.add_systems(
            Update,
            (
                game_state_input_events,
                transition_to_in_game.run_if(in_state(AppState::GameOver)),
            ),
        );
    }
}

fn game_state_input_events(
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            AppState::InGame => next_state.set(AppState::GameOver),
            AppState::GameOver => next_state.set(AppState::InMenu),
            AppState::InMenu => next_state.set(AppState::InGame),
        }
    }
}

fn transition_to_in_game(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}
