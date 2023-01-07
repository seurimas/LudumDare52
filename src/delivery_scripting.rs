use bevy::prelude::*;
use bevy_wasm_scripting::*;
use wasmer::*;

use crate::delivery::*;
use crate::harvest::*;
use crate::loading::*;

// We don't need to include Commands or components referenced through Commands.
type DeliveryScriptComponents = (
    &'static HarvestSpot,
    &'static Harvestable,
    &'static Children,
);
type DeliveryScriptResources = (
    ResMut<'static, DeliveryItem>,
    Res<'static, DeliveryScripts>,
    Res<'static, TextureAssets>,
    Res<'static, HarvestableTypes>,
);

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
            "attach_child" => Function::new_typed_with_env(&mut wasmer_store.0, &env, attach_child::<S>),
            "spawn_harvestable_by_id" => Function::new_typed_with_env(&mut wasmer_store.0, &env, spawn_harvestable_by_id::<S>),

            "get_harvestable_id" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_harvestable_id),
            "get_harvestable_value" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_harvestable_value),
            "get_harvestable_is_plant" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_harvestable_is_plant),

            "get_harvest_spot_progress" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_harvest_spot_progress),
            "get_harvest_spot_progress_perc" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_harvest_spot_progress_perc),
            "get_harvest_spot_harvest_time" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_harvest_spot_harvest_time),
            "get_harvest_spot_harvestable" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_harvest_spot_harvestable),
            "set_harvest_spot_progress" => Function::new_typed_with_env(&mut wasmer_store.0, &env, set_harvest_spot_progress::<S>),
            "set_harvest_spot_progress_perc" => Function::new_typed_with_env(&mut wasmer_store.0, &env, set_harvest_spot_progress_perc::<S>),
            "set_harvest_spot_harvest_time" => Function::new_typed_with_env(&mut wasmer_store.0, &env, set_harvest_spot_harvest_time::<S>),
            "set_harvest_spot_harvestable" => Function::new_typed_with_env(&mut wasmer_store.0, &env, set_harvest_spot_harvestable::<S>),

            "get_free_child_harvest_spot" => Function::new_typed_with_env(&mut wasmer_store.0, &env, get_free_child_harvest_spot::<S>),
            "set_visibility" => Function::new_typed_with_env(&mut wasmer_store.0, &env, set_visibility::<S>),
        }
    }
}

fn despawn_entity<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
) {
    env.data()
        .commands::<S>()
        .entity(entity_id.to_entity())
        .despawn();
}

fn attach_child<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    me: EntityId,
    child: EntityId,
) {
    env.data()
        .commands::<S>()
        .entity(me.to_entity())
        .add_child(child.to_entity());
}

fn spawn_harvestable_by_id<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    id: i32,
    real: i8,
) -> EntityId {
    let world = env.data().read();
    let sprite_assets = world.get_resource::<TextureAssets>().unwrap();
    let harvestables = world.get_resource::<HarvestableTypes>().unwrap();
    if let Some(harvestable) = harvestables.get(id) {
        println!("Spawning {}", harvestable.id);
        EntityId::from_entity({
            env.data()
                .commands::<S>()
                .spawn(HarvestableBundle {
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprite_assets.harvestables.clone(),
                        sprite: TextureAtlasSprite {
                            index: if real == 1 {
                                harvestable.sprite_index
                            } else {
                                harvestable
                                    .seed_sprite_index
                                    .unwrap_or(harvestable.sprite_index)
                            },
                            ..Default::default()
                        },
                        visibility: Visibility::INVISIBLE,
                        ..Default::default()
                    },
                    harvestable: Harvestable(harvestable, real == 1),
                })
                .id()
        })
    } else {
        println!("No harvestable {}", id);
        EntityId::missing()
    }
}

fn get_harvestable_id(env: FunctionEnvMut<WorldPointer>, entity_id: EntityId) -> i32 {
    env.data()
        .read()
        .get::<Harvestable>(entity_id.to_entity())
        .map(|harvestable| harvestable.0.id)
        .unwrap_or(-1)
}

fn get_harvestable_value(env: FunctionEnvMut<WorldPointer>, entity_id: EntityId) -> i32 {
    env.data()
        .read()
        .get::<Harvestable>(entity_id.to_entity())
        .map(|harvestable| {
            if harvestable.1 {
                harvestable.0.value
            } else {
                -1
            }
        })
        .unwrap_or(-1)
}

fn get_harvestable_is_plant(env: FunctionEnvMut<WorldPointer>, entity_id: EntityId) -> i32 {
    env.data()
        .read()
        .get::<Harvestable>(entity_id.to_entity())
        .map(|harvestable| if harvestable.0.plant { 1 } else { 0 })
        .unwrap_or(0)
}

fn get_harvest_spot_progress(env: FunctionEnvMut<WorldPointer>, entity_id: EntityId) -> f32 {
    env.data()
        .read()
        .get::<HarvestSpot>(entity_id.to_entity())
        .map(|harvest_spot| harvest_spot.progress)
        .unwrap_or(0.0)
}

fn get_harvest_spot_progress_perc(env: FunctionEnvMut<WorldPointer>, entity_id: EntityId) -> f32 {
    env.data()
        .read()
        .get::<HarvestSpot>(entity_id.to_entity())
        .map(|harvest_spot| (harvest_spot.progress / harvest_spot.harvest_time).clamp(0., 1.))
        .unwrap_or(0.0)
}

fn get_harvest_spot_harvest_time(env: FunctionEnvMut<WorldPointer>, entity_id: EntityId) -> f32 {
    env.data()
        .read()
        .get::<HarvestSpot>(entity_id.to_entity())
        .map(|harvest_spot| harvest_spot.harvest_time)
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

fn set_harvest_spot_progress<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
    progress: f32,
) {
    if let Some(mut harvest_spot) = env
        .data()
        .write()
        .get_mut::<HarvestSpot>(entity_id.to_entity())
    {
        harvest_spot.progress = progress;
    } else {
        env.data().commands::<S>().add(move |world: &mut World| {
            if let Some(mut harvest_spot) = world.get_mut::<HarvestSpot>(entity_id.to_entity()) {
                harvest_spot.progress = progress;
            }
        });
    }
}

fn set_harvest_spot_progress_perc<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
    progress_perc: f32,
) {
    if let Some(mut harvest_spot) = env
        .data()
        .write()
        .get_mut::<HarvestSpot>(entity_id.to_entity())
    {
        harvest_spot.progress = progress_perc * harvest_spot.harvest_time;
    } else {
        env.data().commands::<S>().add(move |world: &mut World| {
            if let Some(mut harvest_spot) = world.get_mut::<HarvestSpot>(entity_id.to_entity()) {
                harvest_spot.progress = progress_perc * harvest_spot.harvest_time
            }
        });
    }
}

fn set_harvest_spot_harvest_time<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
    harvest_time: f32,
) {
    if let Some(mut harvest_spot) = env
        .data()
        .write()
        .get_mut::<HarvestSpot>(entity_id.to_entity())
    {
        harvest_spot.harvest_time = harvest_time;
    } else {
        env.data().commands::<S>().add(move |world: &mut World| {
            if let Some(mut harvest_spot) = world.get_mut::<HarvestSpot>(entity_id.to_entity()) {
                harvest_spot.harvest_time = harvest_time;
            }
        });
    }
}

fn set_harvest_spot_harvestable<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
    harvestable: i32,
) {
    let world = env.data().write();
    let harvestable = world
        .get_resource::<HarvestableTypes>()
        .unwrap()
        .get(harvestable);
    if let Some(mut harvest_spot) = world.get_mut::<HarvestSpot>(entity_id.to_entity()) {
        harvest_spot.set_harvestable(harvestable);
    } else {
        env.data().commands::<S>().add(move |world: &mut World| {
            if let Some(mut harvest_spot) = world.get_mut::<HarvestSpot>(entity_id.to_entity()) {
                harvest_spot.set_harvestable(harvestable);
            }
        });
    }
}

fn get_free_child_harvest_spot<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
) -> EntityId {
    if let Some(children) = env.data().read().get::<Children>(entity_id.to_entity()) {
        for &child in children.iter() {
            if let Some(harvest_spot) = env.data().read().get::<HarvestSpot>(child) {
                if harvest_spot.harvestable_type.is_none() {
                    return EntityId::from_entity(child);
                }
            }
        }
    }
    return EntityId::missing();
}

fn set_visibility<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
    new_visibility: i8,
) {
    if let Some(mut visibility) = env
        .data()
        .write()
        .get_mut::<Visibility>(entity_id.to_entity())
    {
        visibility.is_visible = new_visibility == 1;
    }
}
