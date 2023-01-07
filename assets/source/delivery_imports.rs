// This is a transparent struct around u64, for use with entity ids.
// f64 is the best type to cooperate with wasm-bindgen for web builds.
// See: https://github.com/rustwasm/wasm-bindgen/issues/35
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct EntityId(pub f64);

// This is a transparent struct around a bool-like value.
// i8 cooperates with web builds.
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Bool(pub i8);

impl Bool {
    pub fn TRUE() -> Self {
        Self(1)
    }

    pub fn FALSE() -> Self {
        Self(0)
    }
}

impl From<bool> for Bool {
    fn from(val: bool) -> Self {
        if val {
            Self::TRUE()
        } else {
            Self::FALSE()
        }
    }
}

pub fn badEntity() -> EntityId {
    EntityId(f64::from_bits(u64::MAX))
}

extern "C" {
    pub fn despawn_entity(me: EntityId);
    pub fn spawn_harvestable_by_id(id: i32, real: Bool) -> EntityId;
    pub fn attach_child(me: EntityId, child: EntityId);
    pub fn get_harvestable_id(me: EntityId) -> i32;
    pub fn get_harvestable_value(me: EntityId) -> i32;
    pub fn get_harvest_spot_progress(me: EntityId) -> f32;
    pub fn get_harvest_spot_progress_perc(me: EntityId) -> f32;
    pub fn get_harvest_spot_harvest_time(me: EntityId) -> f32;
    pub fn get_harvest_spot_harvestable(me: EntityId) -> EntityId;
    pub fn set_harvest_spot_progress(me: EntityId, progress: f32);
    pub fn set_harvest_spot_progress_perc(me: EntityId, progress_perc: f32);
    pub fn set_harvest_spot_harvest_time(me: EntityId, harvest_time: f32);
    pub fn set_harvest_spot_harvestable(me: EntityId, harvestable_id: i32);
}
