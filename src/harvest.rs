use bevy::{
    asset::{AssetLoader, LoadedAsset},
    ecs::system::EntityCommands,
    prelude::*,
    reflect::{erased_serde::__private::serde::Deserialize, TypeUuid},
    utils::HashMap,
};
use bevy_wasm_scripting::WasmScript;

use crate::{
    common_scripting::ScriptValues,
    delivery::{DeliveryAnchor, DeliveryDropoff, DeliverySource},
    helper::HelperTextBundle,
    loading::{DeliveryScripts, FontAssets, HarvestableAssets, TextureAssets},
    GameState,
};

pub struct HarvestPlugin;

impl Plugin for HarvestPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<HarvestableType>()
            .init_asset_loader::<HarvestableAssetLoader>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(spawn_harvest_spots),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(harvest_base_timer_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(harvest_highlight_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(harvestable_growth_system),
            );
    }
}

pub const HARVEST_FRAMES: usize = 12;

#[derive(Component)]
pub struct HarvestSpot {
    pub harvestable_type: Option<HarvestableType>,
    pub harvestable_entity: Option<Entity>,
    pub progress: f32,
    pub harvest_time: f32,
}

impl HarvestSpot {
    pub fn set_harvestable(&mut self, harvestable_type: Option<HarvestableType>) {
        self.harvestable_type = harvestable_type.clone();
        if let Some(harvestable_type) = harvestable_type.as_ref() {
            self.progress = 0.;
            self.harvestable_entity = None;
            self.harvest_time = harvestable_type.base_harvest_time;
        } else {
            self.progress = 0.;
        }
    }

    pub fn progress_percent(&self) -> i32 {
        (((self.progress / self.harvest_time) * 100.) as i32).clamp(0, 100)
    }
}

#[derive(Clone, Deserialize, TypeUuid)]
#[uuid = "a8e2ef38-b4e3-4468-b936-c397ee9b7afb"]
pub struct HarvestableType {
    pub id: i32,
    pub base_harvest_time: f32,
    pub sprite_index: usize,
    pub seed_sprite_index: Option<usize>,
    pub value: i32,
    pub plant: bool,
    pub troop_id: Option<i32>,
    pub name: String,
    pub description: String,
}

#[derive(Default)]
pub struct HarvestableAssetLoader;

impl AssetLoader for HarvestableAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<HarvestableType>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["harvest"]
    }
}

#[derive(Resource)]
pub struct HarvestableTypes(pub HashMap<i32, HarvestableType>);

impl FromWorld for HarvestableTypes {
    fn from_world(world: &mut World) -> Self {
        let harvestables = world.get_resource::<HarvestableAssets>().unwrap();
        let assets = world.get_resource::<Assets<HarvestableType>>().unwrap();
        let mut map = HashMap::new();
        harvestables.harvestables.iter().for_each(|harvestable| {
            if let Some(harvestable) = assets.get(harvestable) {
                map.insert(harvestable.id, harvestable.clone());
            }
        });
        Self(map)
    }
}

impl HarvestableTypes {
    pub fn get(&self, id: i32) -> Option<HarvestableType> {
        self.0.get(&id).cloned()
    }
}

#[derive(Component)]
pub struct Harvestable(pub HarvestableType, pub bool);

#[derive(Bundle)]
pub struct HarvestableBundle {
    pub sprite: SpriteSheetBundle,
    pub harvestable: Harvestable,
}

#[derive(Bundle)]
pub struct HarvestSpotBundle {
    sprite: SpriteSheetBundle,
    harvest_spot: HarvestSpot,
    script_values: ScriptValues,
    delivery_source: DeliverySource,
    delivery_location: DeliveryDropoff,
    delivery_anchor: DeliveryAnchor,
}

pub fn spawn_harvest_spot<'a, 'b, 'c>(
    commands: &'c mut Commands<'a, 'b>,
    position: Vec2,
    helper_font: Handle<Font>,
    texture_atlas: Handle<TextureAtlas>,
    script: Handle<WasmScript>,
    visibility: Visibility,
) -> Entity {
    let helper = commands
        .spawn(HelperTextBundle::new(
            "If you're seeing this, something is wrong.",
            helper_font,
        ))
        .id();
    commands
        .spawn(HarvestSpotBundle {
            sprite: SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 1.)),
                visibility,
                ..Default::default()
            },
            harvest_spot: HarvestSpot {
                harvestable_type: None,
                harvestable_entity: None,
                progress: 0.,
                harvest_time: 0.,
            },
            delivery_anchor: DeliveryAnchor::new(0., -8., 16., 16 * 16),
            delivery_source: DeliverySource::new(script.clone()),
            delivery_location: DeliveryDropoff::new(script),
            script_values: Default::default(),
        })
        .add_child(helper)
        .id()
}

fn spawn_harvest_spots(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
    scripts: Res<DeliveryScripts>,
) {
    for x in 0..2 {
        for y in 0..3 {
            spawn_harvest_spot(
                &mut commands,
                Vec2::new(48. + x as f32 * 32., 32. - y as f32 * 32.),
                fonts.fira_sans.clone(),
                textures.harvest_base.clone(),
                scripts.field_spot.clone(),
                Visibility::VISIBLE,
            );
        }
    }
}

fn harvest_base_timer_system(
    mut query: Query<(&mut TextureAtlasSprite, &mut HarvestSpot)>,
    time: Res<Time>,
) {
    for (mut sprite, mut spot) in query.iter_mut() {
        if spot.harvestable_type.is_some() {
            spot.progress += time.delta_seconds();
            let frame =
                (((spot.progress / spot.harvest_time) * (HARVEST_FRAMES - 1) as f32).floor()
                    as usize)
                    .clamp(0, HARVEST_FRAMES - 1);
            sprite.index = frame;
        } else {
            sprite.index = HARVEST_FRAMES;
            spot.harvestable_entity = None;
            spot.progress = 0.;
            spot.harvest_time = 0.;
        }
    }
}

fn harvest_highlight_system(
    spot_query: Query<&HarvestSpot>,
    mut harvestable_query: Query<(&Parent, &mut TextureAtlasSprite), With<Harvestable>>,
) {
    for (parent, mut sprite) in harvestable_query.iter_mut() {
        if let Ok(spot) = spot_query.get(parent.get()) {
            if spot.progress < spot.harvest_time {
                sprite
                    .color
                    .set_a((spot.progress / spot.harvest_time / 2.).clamp(0.2, 0.5));
            } else {
                sprite.color.set_a(1.);
            }
        }
    }
}

fn harvestable_growth_system(
    mut commands: Commands,
    mut spot_query: Query<(Entity, &mut HarvestSpot)>,
    textures: Res<TextureAssets>,
) {
    for (entity, mut spot) in spot_query.iter_mut() {
        if spot.harvestable_type.is_some() && spot.harvestable_entity.is_none() {
            let harvestable_entity = commands
                .spawn(HarvestableBundle {
                    sprite: SpriteSheetBundle {
                        texture_atlas: textures.harvestables.clone(),
                        sprite: TextureAtlasSprite {
                            index: spot.harvestable_type.as_ref().unwrap().sprite_index,
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
                        ..Default::default()
                    },
                    harvestable: Harvestable(spot.harvestable_type.clone().unwrap(), true),
                })
                .id();
            spot.harvestable_entity = Some(harvestable_entity);
            commands
                .get_entity(entity)
                .unwrap()
                .add_child(harvestable_entity);
        }
    }
}
