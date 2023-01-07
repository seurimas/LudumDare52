use bevy::prelude::*;
use bevy_wasm_scripting::*;
use wasmer::*;

use crate::battle::*;
use crate::common_scripting::*;

type BattleScriptComponents = (&'static Faction, &'static Children);
type BattleScriptResources = ();

impl WasmScriptComponent for Troop {
    type ImportQueriedComponents = BattleScriptComponents;
    type ImportResources = BattleScriptResources;

    fn get_imports_from_world(
        wasmer_store: &mut bevy_wasm_scripting::WasmerStore,
        world: &bevy_wasm_scripting::WorldPointer,
    ) -> Imports {
        get_battle_imports_from_world::<Troop>(wasmer_store, world)
    }
    fn get_wasm_script_handle(&self) -> &Handle<WasmScript> {
        &self.troop_type.script.as_ref().unwrap()
    }
}

fn get_battle_imports_from_world<S: 'static + Send + Sync>(
    wasmer_store: &mut bevy_wasm_scripting::WasmerStore,
    world: &bevy_wasm_scripting::WorldPointer,
) -> Imports {
    let env = FunctionEnv::new(&mut wasmer_store.0, world.clone());
    imports! {
        "env" => {
            "despawn_entity" => Function::new_typed_with_env(&mut wasmer_store.0, &env, despawn_entity::<S>),
            "attach_child" => Function::new_typed_with_env(&mut wasmer_store.0, &env, attach_child::<S>),
            "spawn_harvestable_by_id" => Function::new_typed_with_env(&mut wasmer_store.0, &env, spawn_harvestable_by_id::<S>),

            "scan_enemies" => Function::new_typed_with_env(&mut wasmer_store.0, &env, scan_enemies),
            "get_enemy_count" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_enemy_count),
            "get_enemy" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_enemy),
            "get_nearest_enemy" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_nearest_enemy),

            "get_x_of" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_x_of),
            "get_y_of" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_y_of),
            "get_distance" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_distance),

            "move_towards" => Function::new_typed_with_env(&mut wasmer_store.0, &env, move_towards),
            "attack_enemy" => Function::new_typed_with_env(&mut wasmer_store.0, &env, attack_enemy),
        }
    }
}

pub fn scan_enemies(env: FunctionEnvMut<WorldPointer>, me: EntityId) {}
pub fn get_enemy_count(env: FunctionEnvMut<WorldPointer>, me: EntityId) -> i32 {
    0
}
pub fn get_enemy(env: FunctionEnvMut<WorldPointer>, me: EntityId, index: i32) -> EntityId {
    EntityId::missing()
}
pub fn get_nearest_enemy(env: FunctionEnvMut<WorldPointer>, me: EntityId) -> EntityId {
    EntityId::missing()
}
pub fn get_x_of(env: FunctionEnvMut<WorldPointer>, me: EntityId) -> f32 {
    env.data()
        .read()
        .get::<Transform>(me.to_entity())
        .map(|transform| transform.translation.y)
        .unwrap_or(0.)
}
pub fn get_y_of(env: FunctionEnvMut<WorldPointer>, me: EntityId) -> f32 {
    env.data()
        .read()
        .get::<Transform>(me.to_entity())
        .map(|transform| transform.translation.y)
        .unwrap_or(0.)
}
pub fn get_distance(env: FunctionEnvMut<WorldPointer>, me: EntityId, other: EntityId) -> f32 {
    if let (Some(transform_a), Some(transform_b)) = (
        env.data().read().get::<Transform>(me.to_entity()),
        env.data().read().get::<Transform>(other.to_entity()),
    ) {
        transform_a.translation.distance(transform_b.translation)
    } else {
        -1.
    }
}

pub fn move_towards(env: FunctionEnvMut<WorldPointer>, me: EntityId, x: f32, y: f32, speed: f32) {
    if let Some(mut troop) = env.data().write().get_mut::<Troop>(me.to_entity()) {
        troop.target = Some((Vec2::new(x, y), speed));
    }
}

pub fn attack_enemy(
    env: FunctionEnvMut<WorldPointer>,
    me: EntityId,
    enemy: EntityId,
    attack_id: i32,
) {
}
