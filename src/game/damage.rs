use crate::game::prelude::*;

#[derive(EntityEvent, Debug, Clone)]
pub struct TakeDamage {
    pub entity: Entity,
    pub damage: i32,
    // TODO:
    // damage_type
}
