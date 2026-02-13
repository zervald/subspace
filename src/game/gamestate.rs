use crate::screens::Screen;
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
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
        app.init_state::<GameState>();
        // app.add_systems(
        //     OnEnter(Screen::Gameplay),
        //     |mut ns: ResMut<NextState<GameState>>| ns.set(GameState::Cruising),
        // );
        app.add_systems(OnEnter(GameState::Destroyed), transition_to_title);
    }
}

fn transition_to_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
