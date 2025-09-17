use std::time::Duration;

use bevy::prelude::*;

use crate::{common::state::AppState, game::prelude::GameState};

#[derive(Component, Debug)]
pub enum Lifetime {
    Timer(Timer),
    State(GameState),
    Entity(Entity),
}

impl From<Entity> for Lifetime {
    fn from(v: Entity) -> Self {
        Self::Entity(v)
    }
}

impl From<GameState> for Lifetime {
    fn from(v: GameState) -> Self {
        Self::State(v)
    }
}

impl Lifetime {
    pub fn tick_timer(&mut self, delta: Duration) {
        if let Lifetime::Timer(timer) = self {
            timer.tick(delta);
        }
    }

    pub fn timer_finished(&self) -> bool {
        match self {
            Lifetime::Timer(timer) => timer.finished(),
            _ => false,
        }
    }

    pub fn from_secs(duration: f32) -> Self {
        Lifetime::Timer(Timer::from_seconds(duration, TimerMode::Once))
    }

    /// Returns `true` if the lifetime is [`Timer`].
    ///
    /// [`Timer`]: Lifetime::Timer
    #[must_use]
    pub fn is_timer(&self) -> bool {
        matches!(self, Self::Timer(..))
    }

    /// Returns `true` if the lifetime is [`State`].
    ///
    /// [`State`]: Lifetime::State
    #[must_use]
    pub fn is_state(&self) -> bool {
        matches!(self, Self::State(..))
    }

    /// Returns `true` if the lifetime is [`Entity`].
    ///
    /// [`Entity`]: Lifetime::Entity
    #[must_use]
    pub fn is_entity(&self) -> bool {
        matches!(self, Self::Entity(..))
    }

    pub fn as_timer(&self) -> Option<&Timer> {
        if let Self::Timer(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_state(&self) -> Option<&GameState> {
        if let Self::State(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_entity(&self) -> Option<&Entity> {
        if let Self::Entity(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

pub fn update_lifetime(
    mut query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    current_state: Res<State<GameState>>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.tick_timer(time.delta());
        if lifetime.timer_finished() {
            commands.entity(entity).try_despawn();
        }

        if let Some(state) = lifetime.as_state()
            && current_state.get() != state
        {
            commands.entity(entity).try_despawn();
        };

        let Some(entity_watched) = lifetime.as_entity() else {
            continue;
        };
        if commands.get_entity(*entity_watched).is_err() {
            commands.entity(entity).try_despawn()
        };
    }
}

pub struct LifetimePlugin;
impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_lifetime.run_if(in_state(AppState::InGame)),
        );
    }
}
