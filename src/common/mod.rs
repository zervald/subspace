#![allow(unused)]
use bevy::prelude::*;

mod despawn;
pub mod lifetime;
pub mod utils;

pub use crate::common::lifetime::Lifetime;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((despawn::DespawnPlugin, lifetime::LifetimePlugin));
    }
}
