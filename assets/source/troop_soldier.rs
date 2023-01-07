#![allow(unused_variables)]
mod common_imports;
mod troop_imports;
use troop_imports::*;

#[no_mangle]
pub unsafe extern "C" fn battle_action(me: EntityId) -> f32 {
    move_towards(me, 0., 0., 32.);
    3.
}
