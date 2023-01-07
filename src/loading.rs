use crate::{
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
                .init_resource::<HarvestableTypes>()
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
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct DeliveryScripts {
    #[asset(path = "scripts/field_spot.wasm")]
    pub field_spot: Handle<WasmScript>,
    #[asset(path = "scripts/market.wasm")]
    pub market: Handle<WasmScript>,
}

#[derive(AssetCollection, Resource)]
pub struct HarvestableAssets {
    #[asset(path = "harvestables/red_berry.harvest")]
    pub red_berry: Handle<HarvestableType>,
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
}
