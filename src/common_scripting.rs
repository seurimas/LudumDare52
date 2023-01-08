use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::prelude::{Audio, *};
use bevy_wasm_scripting::*;
use wasmer::*;

use crate::{
    battle::Troop,
    harvest::{Harvestable, HarvestableBundle, HarvestableTypes},
    loading::*,
};

#[derive(Component, Default)]
pub struct ScriptValues(pub HashMap<i32, f32>);

pub fn get_random(_env: FunctionEnvMut<WorldPointer>) -> f32 {
    rand::random::<f32>()
}

pub fn heal_troop(env: FunctionEnvMut<WorldPointer>, entity_id: EntityId, amount: i32) {
    env.data()
        .write()
        .get_mut::<Troop>(entity_id.to_entity())
        .map(|mut troop| troop.health += 1);
}

pub fn set_script_value(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
    script_value_id: i32,
    new_value: f32,
) {
    env.data()
        .write()
        .get_mut::<ScriptValues>(entity_id.to_entity())
        .and_then(|mut values| values.0.insert(script_value_id, new_value));
}

pub fn get_script_value(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
    script_value_id: i32,
    default: f32,
) -> f32 {
    env.data()
        .read()
        .get::<ScriptValues>(entity_id.to_entity())
        .and_then(|values| values.0.get(&script_value_id))
        .cloned()
        .unwrap_or(default)
}

pub fn play_sound(env: FunctionEnvMut<WorldPointer>, sound_id: i32) {
    let world = env.data().read();
    let audio_assets = world.get_resource::<AudioAssets>().unwrap();
    let audio = world.get_resource::<Audio>().unwrap();
    audio.play(audio_assets.collection[sound_id as usize].clone());
}

pub fn despawn_entity<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    entity_id: EntityId,
) {
    env.data()
        .commands::<S>()
        .entity(entity_id.to_entity())
        .despawn();
}

pub fn attach_child<S: 'static + Send + Sync>(
    env: FunctionEnvMut<WorldPointer>,
    me: EntityId,
    child: EntityId,
) {
    env.data()
        .commands::<S>()
        .entity(me.to_entity())
        .add_child(child.to_entity());
}

pub fn spawn_harvestable_by_id<S: 'static + Send + Sync>(
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
