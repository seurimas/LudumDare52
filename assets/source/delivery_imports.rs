#![allow(dead_code)]
pub use common_imports::*;

pub const RECRUIT: i32 = 3;

extern "C" {
    pub fn get_harvestable_id(me: EntityId) -> i32;
    pub fn get_harvestable_value(me: EntityId) -> i32;
    pub fn get_harvestable_is_plant(me: EntityId) -> Bool;
    pub fn get_harvestable_troop_id(me: EntityId) -> i32;

    pub fn get_harvest_spot_progress(me: EntityId) -> f32;
    pub fn get_harvest_spot_progress_perc(me: EntityId) -> f32;
    pub fn get_harvest_spot_harvest_time(me: EntityId) -> f32;
    pub fn get_harvest_spot_harvestable(me: EntityId) -> EntityId;

    pub fn set_harvest_spot_progress(me: EntityId, progress: f32);
    pub fn set_harvest_spot_progress_perc(me: EntityId, progress_perc: f32);
    pub fn set_harvest_spot_harvest_time(me: EntityId, harvest_time: f32);
    pub fn set_harvest_spot_harvestable(me: EntityId, harvestable_id: i32);

    pub fn get_free_child_harvest_spot(me: EntityId) -> EntityId;
    pub fn set_visibility(me: EntityId, new_visibility: Bool);
    pub fn stage_troop(me: EntityId, troop_id: i32);
}
