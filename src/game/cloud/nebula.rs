use crate::game::{effects::obscured::obscure_bundle, prelude::*};
use bevy::prelude::*;

pub struct Nebula;

fn effect() -> Vec<impl Bundle> {
    vec![obscure_bundle(), ob]
}
