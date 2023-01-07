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

extern "C" {
    pub fn despawn_entity(me: EntityId);
    pub fn spawn_harvestable_by_id(id: i32, real: Bool) -> EntityId;
    pub fn attach_child(me: EntityId, child: EntityId);
}
