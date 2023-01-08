#![allow(unused_variables)]
mod common_imports;
mod troop_imports;
use troop_imports::*;

#[no_mangle]
pub unsafe extern "C" fn battle_action(me: EntityId) -> f32 {
    let nearest_enemy = get_nearest_enemy(me);
    if nearest_enemy.is_missing() {
        scan_enemies(me);
        0.0001
    } else {
        if get_distance(me, nearest_enemy) > 36. {
            move_towards(
                me,
                get_x_of(nearest_enemy),
                get_y_of(nearest_enemy),
                32. * get_script_value(me, SPEED_MOD_ID, 1.0),
            );
            0.1
        } else {
            move_towards(me, 0., 0., 0.);
            if get_random() < get_script_value(nearest_enemy, DODGE_CHANCE_ID, 0.0) {
                attack_enemy(me, nearest_enemy, -1) * get_script_value(me, ATTACK_SPEED_MOD_ID, 1.0)
            } else {
                attack_enemy(me, nearest_enemy, 1) * get_script_value(me, ATTACK_SPEED_MOD_ID, 1.0)
            }
        }
    }
}
