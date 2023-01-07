// This is a transparent struct around 64, for use with entity ids.
// f64 is the best type to cooperate with wasm-bindgen for web builds.
// See: https://github.com/rustwasm/wasm-bindgen/issues/35
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct EntityId(pub f64);

pub fn badEntity() -> EntityId {
    EntityId(f64::from_bits(u64::MAX))
}

extern "C" {
    pub fn despawn_entity(me: EntityId);
    pub fn get_harvestable_id(me: EntityId) -> i32;
    pub fn get_harvest_spot_harvestable(me: EntityId) -> EntityId;
    pub fn get_harvest_spot_progress(me: EntityId) -> f32;
    pub fn set_harvest_spot_harvestable(me: EntityId, harvestable_id: i32);
}
