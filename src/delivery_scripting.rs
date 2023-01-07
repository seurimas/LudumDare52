use bevy::prelude::*;
use bevy_wasm_scripting::*;
use wasmer::*;

use crate::delivery::*;
use crate::harvest::*;

type DeliveryScriptComponents = (&'static HarvestSpot,);
type DeliveryScriptResources = (ResMut<'static, DeliveryItem>,);

impl WasmScriptComponent for DeliveryDropoff {
    type ImportQueriedComponents = DeliveryScriptComponents;
    type ImportResources = DeliveryScriptResources;

    fn get_imports_from_world(
        wasmer_store: &mut bevy_wasm_scripting::WasmerStore,
        world: &bevy_wasm_scripting::WorldPointer,
    ) -> Imports {
        get_delivery_imports_from_world::<DeliverySource>(wasmer_store, world)
    }
    fn get_wasm_script_handle(&self) -> &Handle<WasmScript> {
        &self.script
    }
}

impl WasmScriptComponent for DeliverySource {
    type ImportQueriedComponents = DeliveryScriptComponents;
    type ImportResources = DeliveryScriptResources;

    fn get_imports_from_world(
        wasmer_store: &mut bevy_wasm_scripting::WasmerStore,
        world: &bevy_wasm_scripting::WorldPointer,
    ) -> Imports {
        get_delivery_imports_from_world::<DeliverySource>(wasmer_store, world)
    }

    fn get_wasm_script_handle(&self) -> &Handle<WasmScript> {
        &self.script
    }
}

fn get_delivery_imports_from_world<S: 'static + Send + Sync>(
    wasmer_store: &mut bevy_wasm_scripting::WasmerStore,
    world: &bevy_wasm_scripting::WorldPointer,
) -> Imports {
    let env = FunctionEnv::new(&mut wasmer_store.0, world.clone());
    imports! {
        "env" => {
            "despawn_entity" => Function::new_typed_with_env(&mut wasmer_store.0, &env, despawn_entity::<S>),
            "get_harvestable_id" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_harvestable_id),
            "get_harvest_spot_progress" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_harvest_spot_progress),
            "get_harvest_spot_harvestable" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_harvest_spot_harvestable),
            "set_harvest_spot_harvestable" => Function::new_typed_with_env(&mut wasmer_store.0, &env, set_harvest_spot_harvestable),
        }
    }
}

fn despawn_entity<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
) {
    println!("Despawning!");
    env.data()
        .commands::<S>()
        .entity(entity_id.to_entity())
        .despawn();
}

fn get_harvestable_id(env: FunctionEnvMut<WorldPointer>, entity_id: EntityId) -> i32 {
    env.data()
        .read()
        .get::<Harvestable>(entity_id.to_entity())
        .map(|harvestable| harvestable.0.id)
        .unwrap_or(-1)
}

fn get_harvest_spot_progress(env: FunctionEnvMut<WorldPointer>, entity_id: EntityId) -> f32 {
    env.data()
        .read()
        .get::<HarvestSpot>(entity_id.to_entity())
        .map(|harvest_spot| (harvest_spot.progress / harvest_spot.harvest_time).clamp(0., 1.))
        .unwrap_or(0.0)
}

fn get_harvest_spot_harvestable(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
) -> EntityId {
    env.data()
        .read()
        .get::<HarvestSpot>(entity_id.to_entity())
        .and_then(|harvest_spot| harvest_spot.harvestable_entity)
        .map(|harvestable_entity| EntityId::from_bits(harvestable_entity.to_bits()))
        .unwrap_or(EntityId::from_bits(u64::MAX))
}

fn set_harvest_spot_harvestable(env: FunctionEnvMut<WorldPointer>, me: EntityId, harvestable: i32) {
    let world = env.data().write();
    let harvestable = world
        .get_resource::<HarvestableTypes>()
        .unwrap()
        .get(harvestable);
    if let Some(mut harvest_spot) = world.get_mut::<HarvestSpot>(me.to_entity()) {
        harvest_spot.set_harvestable(harvestable);
    }
}
