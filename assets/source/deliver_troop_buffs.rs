#![allow(unused_variables)]
mod common_imports;
mod delivery_imports;
use delivery_imports::*;

pub const SOLDIER: i32 = 4;

#[no_mangle]
pub unsafe extern "C" fn can_receive(me: EntityId, delivery: EntityId) -> Bool {
    if get_harvestable_is_real(delivery) == Bool::r#true() {
        match get_harvestable_id(delivery) {
            0 | 1 | 2 | 6 => Bool::r#true(),
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
            heal_troop(me, (get_random() * 3.) as i32 + 4);
        }
        1 => {
            // Grapes increase speed
            set_script_value(
                me,
                SPEED_MOD_ID,
                get_script_value(me, SPEED_MOD_ID, 1.) * 1.2,
            );
        }
        2 => {
            // Lemons increase attack speed
            set_script_value(
                me,
                ATTACK_SPEED_MOD_ID,
                get_script_value(me, ATTACK_SPEED_MOD_ID, 1.) * 0.80,
            );
        }
        6 => {
            // X increase dodge rate
            set_script_value(
                me,
                DODGE_CHANCE_ID,
                1. - ((1. - get_script_value(me, DODGE_CHANCE_ID, 0.)) * 0.8),
            );
        }
        _ => {}
    }
    despawn_entity(delivery);
}
