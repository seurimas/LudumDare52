#![allow(unused_variables)]
mod common_imports;
mod delivery_imports;
use delivery_imports::*;

#[no_mangle]
pub unsafe extern "C" fn can_produce(me: EntityId) -> Bool {
    let harvestable = get_harvest_spot_harvestable(me);
    if !harvestable.is_missing() {
        (get_harvest_spot_progress_perc(me) >= 1.).into()
    } else {
        Bool::r#false()
    }
}

#[no_mangle]
pub unsafe extern "C" fn produce(me: EntityId) -> EntityId {
    set_harvest_spot_harvestable(me, -1);
    get_harvest_spot_harvestable(me)
}

#[no_mangle]
pub unsafe extern "C" fn can_receive(me: EntityId, delivery: EntityId) -> Bool {
    if !get_harvest_spot_harvestable(me).is_missing() {
        Bool::r#false()
    } else if get_harvestable_is_plant(delivery) == Bool::r#true() {
        (get_harvestable_is_real(delivery) == Bool::r#false()).into()
    } else {
        Bool::r#false()
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
        set_harvest_spot_progress_perc(me, 1.);
        attach_child(me, delivery);
    }
    despawn_entity(delivery);
}
