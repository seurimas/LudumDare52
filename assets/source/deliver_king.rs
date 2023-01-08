#![allow(unused_variables)]
mod common_imports;
mod delivery_imports;
use delivery_imports::*;

pub const SOLDIER: i32 = 4;

#[no_mangle]
pub unsafe extern "C" fn can_receive(me: EntityId, delivery: EntityId) -> Bool {
    if get_harvestable_is_real(delivery) == Bool::r#true() {
        match get_harvestable_id(delivery) {
            0 => Bool::r#true(),
            _ => Bool::r#false(),
        }
    } else {
        Bool::r#false()
    }
}

#[no_mangle]
pub unsafe extern "C" fn receive(me: EntityId, delivery: EntityId, from: EntityId) {
    match get_harvestable_id(delivery) {
        0 => {
            // Red berries increase health.
            heal_troop(me, 1);
        }
        _ => {}
    }
    despawn_entity(delivery);
}
