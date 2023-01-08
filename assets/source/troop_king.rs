#![allow(unused_variables)]
mod common_imports;
mod troop_imports;
use troop_imports::*;

#[no_mangle]
pub unsafe extern "C" fn battle_action(me: EntityId) -> f32 {
    retreat(me, 500.);
    0.01
}

#[no_mangle]
pub unsafe extern "C" fn on_death(me: EntityId) -> Bool {
    Bool::r#true()
}
