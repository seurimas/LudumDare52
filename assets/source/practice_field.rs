mod delivery_imports;
use delivery_imports::*;

pub const SOLDIER: i32 = 4;

#[no_mangle]
pub unsafe extern "C" fn can_receive(me: EntityId, delivery: EntityId) -> Bool {
    (get_harvestable_id(delivery) == RECRUIT).into()
}

#[no_mangle]
pub unsafe extern "C" fn receive(me: EntityId, delivery: EntityId, from: EntityId) {
    let spot = get_free_child_harvest_spot(me);
    if !spot.is_missing() {
        set_visibility(spot, Bool::TRUE());
        set_harvest_spot_harvestable(spot, SOLDIER);
        despawn_entity(delivery);
    } else {
        despawn_entity(delivery);
    }
}
