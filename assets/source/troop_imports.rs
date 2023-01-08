#![allow(dead_code)]
pub use common_imports::*;

extern "C" {
    pub fn scan_enemies(me: EntityId);
    pub fn get_enemy_count(me: EntityId) -> i32;
    pub fn get_enemy(me: EntityId, index: i32) -> EntityId;
    pub fn get_nearest_enemy(me: EntityId) -> EntityId;

    pub fn get_x_of(me: EntityId) -> f32;
    pub fn get_y_of(me: EntityId) -> f32;
    pub fn get_distance(me: EntityId, other: EntityId) -> f32;

    pub fn retreat(me: EntityId, speed: f32);
    pub fn move_towards(me: EntityId, x: f32, y: f32, speed: f32);
    pub fn attack_enemy(me: EntityId, enemy: EntityId, attack_id: i32) -> f32;
}
