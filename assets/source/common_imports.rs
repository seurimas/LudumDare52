#![allow(dead_code)]
// This is a transparent struct around u64, for use with entity ids.
// f64 is the best type to cooperate with wasm-bindgen for web builds.
// See: https://github.com/rustwasm/wasm-bindgen/issues/35
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct EntityId(pub f64);

impl EntityId {
    pub fn is_missing(&self) -> bool {
        self.0.to_bits() == u64::MAX
    }
}

// This is a transparent struct around a bool-like value.
// i8 cooperates with web builds.
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Bool(pub i8);

impl Bool {
    pub fn r#true() -> Self {
        Self(1)
    }

    pub fn r#false() -> Self {
        Self(0)
    }
}

impl From<bool> for Bool {
    fn from(val: bool) -> Self {
        if val {
            Self::r#true()
        } else {
            Self::r#false()
        }
    }
}

pub const SPEED_MOD_ID: i32 = 0;
pub const ATTACK_SPEED_MOD_ID: i32 = 1;
pub const ATTACK_RANGE_MOD_ID: i32 = 2;
pub const DODGE_CHANCE_ID: i32 = 3;

extern "C" {
    pub fn despawn_entity(me: EntityId);
    pub fn spawn_harvestable_by_id(id: i32, real: Bool) -> EntityId;
    pub fn attach_child(me: EntityId, child: EntityId);
    pub fn play_sound(sound_id: i32);
    pub fn get_random() -> f32;

    pub fn get_script_value(me: EntityId, script_value_id: i32, default: f32) -> f32;
    pub fn set_script_value(me: EntityId, script_value_id: i32, new_value: f32);
    pub fn heal_troop(me: EntityId, amount: i32);
}
