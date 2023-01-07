use bevy::prelude::*;
use bevy_wasm_scripting::*;
use wasmer::*;

use crate::{
    harvest::{Harvestable, HarvestableBundle, HarvestableTypes},
    loading::*,
};

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
