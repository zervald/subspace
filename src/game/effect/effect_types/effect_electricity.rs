use bevy::prelude::*;

#[derive(Component)]
pub struct ElectricityEffect(pub i32);

impl Default for ElectricityEffect {
    fn default() -> Self {
        Self(5)
    }
}

impl From<i32> for ElectricityEffect {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
