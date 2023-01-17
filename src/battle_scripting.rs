use bevy::prelude::*;
use bevy_wasm_scripting::*;
use rand::random;
use wasmer::*;

use crate::attacks::spawn_attack;
use crate::attacks::AttackType;
use crate::attacks::AttackTypes;
use crate::battle::*;
use crate::common_scripting::*;
use crate::loading::TextureAssets;

type BattleScriptComponents = (&'static Faction, &'static Children);
type BattleScriptResources = Res<'static, AttackTypes>;

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
            "play_sound" => Function::new_typed_with_env(&mut wasmer_store.0, &env, play_sound),
            "get_random" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_random),
            "get_script_value" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_script_value),
            "set_script_value" => Function::new_typed_with_env(&mut wasmer_store.0, &env, set_script_value),
            "heal_troop" => Function::new_typed_with_env(&mut wasmer_store.0, &env, heal_troop),

            "scan_enemies" => Function::new_typed_with_env(&mut wasmer_store.0, &env, scan_enemies),
            "get_enemy_count" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_enemy_count),
            "get_enemy" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_enemy),
            "get_nearest_enemy" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_nearest_enemy),

            "get_x_of" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_x_of),
            "get_y_of" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_y_of),
            "get_distance" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_distance),

            "retreat" => Function::new_typed_with_env(&mut wasmer_store.0, &env, retreat),
            "move_towards" => Function::new_typed_with_env(&mut wasmer_store.0, &env, move_towards),
            "attack_enemy" => Function::new_typed_with_env(&mut wasmer_store.0, &env, attack_enemy::<S>),
        }
    }
}

pub fn scan_enemies(env: FunctionEnvMut<WorldPointer>, me: EntityId) {
    let scanned = if let Some(my_faction) = env.data().read().get::<Faction>(me.to_entity()) {
        let world = env.data().write();
        let mut query = world.query::<(Entity, &Troop, &Faction)>();
        let mut seen_entities = Vec::new();
        for (entity, _troop, faction) in query.iter(world) {
            if entity == me.to_entity() {
                continue;
            } else if faction.faction_id != my_faction.faction_id {
                seen_entities.push(entity);
            }
        }
        seen_entities
    } else {
        Vec::new()
    };
    if let Some(mut troop) = env.data().write().get_mut::<Troop>(me.to_entity()) {
        troop.scan(scanned);
    }
}
pub fn get_enemy_count(env: FunctionEnvMut<WorldPointer>, me: EntityId) -> i32 {
    if let Some(troop) = env.data().read().get::<Troop>(me.to_entity()) {
        troop.seen_troops.len() as i32
    } else {
        0
    }
}
pub fn get_enemy(env: FunctionEnvMut<WorldPointer>, me: EntityId, index: i32) -> EntityId {
    if let Some(troop) = env.data().read().get::<Troop>(me.to_entity()) {
        troop
            .seen_troops
            .get(index as usize)
            .map(|id| EntityId::from_entity(*id))
            .unwrap_or(EntityId::missing())
    } else {
        EntityId::missing()
    }
}

pub fn get_nearest_enemy(env: FunctionEnvMut<WorldPointer>, me: EntityId) -> EntityId {
    let my_transform = env.data().read().get::<GlobalTransform>(me.to_entity());
    if my_transform.is_none() {
        return EntityId::missing();
    }
    let my_transform = my_transform.unwrap();
    let my_location = Vec2::new(my_transform.translation().x, my_transform.translation().y);
    env.data()
        .read()
        .get::<Troop>(me.to_entity())
        .and_then(|troop| {
            troop.seen_troops.iter().min_by_key(|other| {
                let other_transform = env.data().read().get::<GlobalTransform>(**other);
                if other_transform.is_none() {
                    return i32::MAX;
                }
                let other_transform = other_transform.unwrap();
                let other_location = Vec2::new(
                    other_transform.translation().x,
                    other_transform.translation().y,
                );
                let distance = my_location.distance_squared(other_location) as i32;
                distance
            })
        })
        .filter(|entity| env.data().read().get_entity(**entity).is_some())
        .map(|entity| EntityId::from_entity(*entity))
        .unwrap_or(EntityId::missing())
}

pub fn get_x_of(env: FunctionEnvMut<WorldPointer>, me: EntityId) -> f32 {
    env.data()
        .read()
        .get::<Transform>(me.to_entity())
        .map(|transform| transform.translation.x)
        .unwrap_or_else(|| {
            warn!("Could not find transform for {:?}.", me.to_entity());
            0.
        })
}
pub fn get_y_of(env: FunctionEnvMut<WorldPointer>, me: EntityId) -> f32 {
    env.data()
        .read()
        .get::<Transform>(me.to_entity())
        .map(|transform| transform.translation.y)
        .unwrap_or_else(|| {
            warn!("Could not find transform for {:?}.", me.to_entity());
            0.
        })
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

pub fn retreat(env: FunctionEnvMut<WorldPointer>, me: EntityId, speed: f32) {
    if let Some(mut troop) = env.data().write().get_mut::<Troop>(me.to_entity()) {
        troop.target = Some((troop.staging_point.clone(), speed));
    }
}

pub fn move_towards(env: FunctionEnvMut<WorldPointer>, me: EntityId, x: f32, y: f32, speed: f32) {
    if let Some(mut troop) = env.data().write().get_mut::<Troop>(me.to_entity()) {
        troop.target = Some((Vec2::new(x, y), speed));
    }
}

pub fn attack_enemy<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    me: EntityId,
    enemy: EntityId,
    attack_id: i32,
) -> f32 {
    let dodge_chance = env
        .data()
        .read()
        .get::<ScriptValues>(enemy.to_entity())
        .and_then(|script_values| script_values.0.get(&DODGE_CHANCE_ID))
        .cloned()
        .unwrap_or(0.);
    let attack_id = if random::<f32>() < dodge_chance {
        -attack_id
    } else {
        attack_id
    };
    if let (Some(attack_type), Some(sprites)) = (
        env.data()
            .read()
            .get_resource::<AttackTypes>()
            .and_then(|attack_types| attack_types.get(attack_id)),
        env.data().read().get_resource::<TextureAssets>(),
    ) {
        let cooldown = attack_type.cooldown;
        spawn_attack(
            &mut env.data().commands::<S>(),
            me.to_entity(),
            enemy.to_entity(),
            sprites.attacks.clone(),
            attack_type,
        );
        cooldown
    } else {
        0.
    }
}
