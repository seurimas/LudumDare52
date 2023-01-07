mod delivery_imports;
use delivery_imports::*;

const TRUE: i8 = 1;
const FALSE: i8 = 0;

#[no_mangle]
pub unsafe extern "C" fn can_produce(me: EntityId) -> i8 {
    let harvestable = get_harvest_spot_harvestable(me);
    if harvestable != badEntity() {
        if get_harvest_spot_progress(me) >= 1. {
            TRUE
        } else {
            FALSE
        }
    } else {
        FALSE
    }
}

#[no_mangle]
pub unsafe extern "C" fn produce(me: EntityId) -> EntityId {
    set_harvest_spot_harvestable(me, -1);
    get_harvest_spot_harvestable(me)
}

#[no_mangle]
pub unsafe extern "C" fn can_receive(me: EntityId, delivery: EntityId) -> i8 {
    if get_harvestable_id(delivery) != -1 {
        TRUE
    } else {
        FALSE
    }
}

#[no_mangle]
pub unsafe extern "C" fn receive(me: EntityId, delivery: EntityId, from: EntityId) {
    let harvestable_id = get_harvestable_id(delivery);
    set_harvest_spot_harvestable(me, harvestable_id);
    despawn_entity(delivery);
}

#[no_mangle]
pub unsafe extern "C" fn rejected(me: EntityId, delivery: EntityId) {
    if get_harvestable_id(get_harvest_spot_harvestable(me)) == -1 {
        let harvestable_id = get_harvestable_id(delivery);
        set_harvest_spot_harvestable(me, harvestable_id);
    }
    despawn_entity(delivery);
}
