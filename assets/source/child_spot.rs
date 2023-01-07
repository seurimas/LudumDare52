mod delivery_imports;
use delivery_imports::*;

#[no_mangle]
pub unsafe extern "C" fn can_produce(me: EntityId) -> Bool {
    let harvestable = get_harvest_spot_harvestable(me);
    if !harvestable.is_missing() {
        (get_harvest_spot_progress_perc(me) >= 1.).into()
    } else {
        Bool::FALSE()
    }
}

#[no_mangle]
pub unsafe extern "C" fn produce(me: EntityId) -> EntityId {
    set_harvest_spot_harvestable(me, -1);
    set_visibility(me, Bool::FALSE());
    get_harvest_spot_harvestable(me)
}

#[no_mangle]
pub unsafe extern "C" fn can_receive(me: EntityId, delivery: EntityId) -> Bool {
    Bool::FALSE()
}

#[no_mangle]
pub unsafe extern "C" fn receive(me: EntityId, delivery: EntityId, from: EntityId) {}

#[no_mangle]
pub unsafe extern "C" fn rejected(me: EntityId, delivery: EntityId) {
    if get_harvestable_id(get_harvest_spot_harvestable(me)) == -1 {
        let harvestable_id = get_harvestable_id(delivery);
        set_harvest_spot_harvestable(me, harvestable_id);
        set_harvest_spot_progress_perc(me, 1.);
        attach_child(me, delivery);
        set_visibility(me, Bool::TRUE());
    }
    despawn_entity(delivery);
}
