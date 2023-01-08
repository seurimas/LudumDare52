use crate::{
    attacks::{AttackType, AttackTypes},
    battle::{TroopType, TroopTypes},
    harvest::{Harvestable, HarvestableType, HarvestableTypes},
    GameState,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_wasm_scripting::WasmScript;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<DeliveryScripts>()
                .with_collection::<HarvestableAssets>()
                .with_collection::<AttackAssets>()
                .with_collection::<TroopAssets>()
                .init_resource::<HarvestableTypes>()
                .init_resource::<TroopTypes>()
                .init_resource::<AttackTypes>()
                .continue_to_state(GameState::Menu),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(
        paths(
            "audio/hit0.mp3",
            "audio/hit1.mp3",
            "audio/hit2.mp3",
            "audio/hit3.mp3",
            "audio/hit4.mp3",
            "audio/shoot0.mp3",
            "audio/shoot1.mp3",
            "audio/shoot2.mp3",
            "audio/shoot3.mp3",
            "audio/shoot4.mp3",
            "audio/loot0.mp3",
            "audio/loot1.mp3",
        ),
        collection(typed)
    )]
    pub collection: Vec<Handle<AudioSource>>,
}

#[derive(AssetCollection, Resource)]
pub struct DeliveryScripts {
    #[asset(path = "scripts/field_spot.wasm")]
    pub field_spot: Handle<WasmScript>,
    #[asset(path = "scripts/market.wasm")]
    pub market: Handle<WasmScript>,
    #[asset(path = "scripts/recruitment.wasm")]
    pub recruitment: Handle<WasmScript>,
    #[asset(path = "scripts/practice_field.wasm")]
    pub practice_field: Handle<WasmScript>,
    #[asset(path = "scripts/archery_field.wasm")]
    pub archery_field: Handle<WasmScript>,
    #[asset(path = "scripts/child_spot.wasm")]
    pub child_spot: Handle<WasmScript>,
    #[asset(path = "scripts/staging.wasm")]
    pub staging: Handle<WasmScript>,
    #[asset(path = "scripts/deliver_troop_buffs.wasm")]
    pub deliver_troop_buffs: Handle<WasmScript>,
}

#[derive(AssetCollection, Resource)]
pub struct HarvestableAssets {
    #[asset(
        paths(
            "harvestables/archer.harvest",
            "harvestables/grapes.harvest",
            "harvestables/recruit.harvest",
            "harvestables/red_berry.harvest",
            "harvestables/soldier.harvest",
        ),
        collection(typed)
    )]
    pub harvestables: Vec<Handle<HarvestableType>>,
}

#[derive(AssetCollection, Resource)]
pub struct AttackAssets {
    #[asset(
        paths("attacks/slash.attack", "attacks/arrow.attack",),
        collection(typed)
    )]
    pub attacks: Vec<Handle<AttackType>>,
}

#[derive(AssetCollection, Resource)]
pub struct TroopAssets {
    #[asset(
        paths("troops/archer.troop", "troops/soldier.troop", "troops/enemy.troop",),
        collection(typed)
    )]
    pub troops: Vec<Handle<TroopType>>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., rows = 2, columns = 16))]
    #[asset(path = "textures/HarvestBase.png")]
    pub harvest_base: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., rows = 8, columns = 8))]
    #[asset(path = "textures/Locations.png")]
    pub locations: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., rows = 4, columns = 4))]
    #[asset(path = "textures/Harvestables.png")]
    pub harvestables: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., rows = 4, columns = 4))]
    #[asset(path = "textures/Troops.png")]
    pub troops: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., rows = 4, columns = 4))]
    #[asset(path = "textures/Attacks.png")]
    pub attacks: Handle<TextureAtlas>,
}
