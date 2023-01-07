mod delivery_imports;
use delivery_imports::*;

#[no_mangle]
pub unsafe extern "C" fn can_produce(me: EntityId) -> Bool {
    Bool::TRUE()
}

#[no_mangle]
pub unsafe extern "C" fn produce(me: EntityId) -> EntityId {
    spawn_harvestable_by_id(3, false.into())
}

#[no_mangle]
pub unsafe extern "C" fn rejected(me: EntityId, delivery: EntityId) {
    despawn_entity(delivery);
}
