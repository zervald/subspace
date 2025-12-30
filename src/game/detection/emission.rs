use bevy::prelude::*;

// Emission
#[derive(Component, Deref, DerefMut)]
pub struct Emission(pub i32);

impl Default for Emission {
    fn default() -> Self {
        Self(5)
    }
}

impl From<i32> for Emission {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
