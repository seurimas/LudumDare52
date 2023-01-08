#![allow(unused_variables)]
mod common_imports;
mod delivery_imports;
use delivery_imports::*;

#[no_mangle]
pub unsafe extern "C" fn can_produce(me: EntityId) -> Bool {
    Bool::r#true()
}

#[no_mangle]
pub unsafe extern "C" fn produce(me: EntityId) -> EntityId {
    let harvestable_id = if get_random() > 0.5 { 0 } else { 1 };
    spawn_harvestable_by_id(harvestable_id, false.into())
}

#[no_mangle]
pub unsafe extern "C" fn can_receive(me: EntityId, delivery: EntityId) -> Bool {
    if get_harvestable_value(delivery) != -1 {
        Bool::r#true()
    } else {
        Bool::r#false()
    }
}

#[no_mangle]
pub unsafe extern "C" fn receive(me: EntityId, delivery: EntityId, from: EntityId) {
    despawn_entity(delivery);
}

#[no_mangle]
pub unsafe extern "C" fn rejected(me: EntityId, delivery: EntityId) {
    despawn_entity(delivery);
}
