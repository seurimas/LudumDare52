use bevy::{
    asset::{AssetLoader, LoadedAsset},
    ecs::system::EntityCommands,
    prelude::*,
    reflect::TypeUuid,
    utils::HashMap,
};
use bevy_wasm_scripting::*;
use serde::Deserialize;

use crate::{
    attacks::{attack_phase_system, AttackAssetLoader, AttackType},
    delivery::*,
    harvest::spawn_harvest_spot,
    loading::*,
    GameState,
};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TroopType>()
            .init_asset_loader::<TroopAssetLoader>()
            .add_asset::<AttackType>()
            .init_asset_loader::<AttackAssetLoader>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_debug_enemy))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_staging_spot))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(
                ScriptSystemWithCommands::<_, Troop>::wrap(IntoSystem::into_system(
                    troop_battle_action_system,
                )),
            ))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(attack_phase_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(troop_staging_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(troop_cooldown_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(troop_movement_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(troop_restitution_system),
            )
            .add_wasm_script_component::<Troop>();
    }
}

#[derive(Bundle)]
pub struct StagingBundle {
    sprite: SpriteSheetBundle,
    staging_location: StagingLocation,
    delivery_dropoff: DeliveryDropoff,
    delivery_anchor: DeliveryAnchor,
}

#[derive(Component, Default)]
pub struct StagingLocation {
    pub staged: Vec<i32>,
}

impl StagingLocation {
    pub fn stage(&mut self, new_troop: i32) {
        self.staged.push(new_troop);
    }
}

fn spawn_staging_spot(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    scripts: Res<DeliveryScripts>,
) {
    commands.spawn(StagingBundle {
        sprite: SpriteSheetBundle {
            texture_atlas: textures.locations.clone(),
            sprite: TextureAtlasSprite {
                index: 4,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(256., 0., 1.)),
            ..Default::default()
        },
        staging_location: Default::default(),
        delivery_anchor: DeliveryAnchor::new(0., -16., 32., 32 * 32),
        delivery_dropoff: DeliveryDropoff::new(scripts.staging.clone()),
    });
}

fn spawn_debug_enemy(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    troop_types: Res<TroopTypes>,
) {
    spawn_troop(
        &mut commands,
        Vec2::new(0., 300.),
        textures.troops.clone(),
        troop_types.get(-1).unwrap(),
        Faction::enemy(),
    );
}

#[derive(Clone, Deserialize, TypeUuid)]
#[uuid = "57cde8f9-c5e6-4a79-988d-214c3ea1df8e"]
pub struct TroopType {
    pub id: i32,
    pub health: i32,
    pub sprite_index: usize,
    pub size: f32,
    pub script_path: String,
    #[serde(skip_deserializing)]
    pub script: Option<Handle<WasmScript>>,
}

#[derive(Default)]
pub struct TroopAssetLoader;

impl AssetLoader for TroopAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut custom_asset = ron::de::from_bytes::<TroopType>(bytes)?;
            let script_path = custom_asset.script_path.clone();
            let wasm_bytes = load_context.read_asset_bytes(script_path).await?;
            custom_asset.script = Some(load_context.set_labeled_asset(
                "battle_script",
                LoadedAsset::new(WasmScript::Loaded(
                    format!("troop:{}", custom_asset.id),
                    wasm_bytes.to_vec(),
                )),
            ));
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["troop"]
    }
}

#[derive(Resource)]
pub struct TroopTypes(pub HashMap<i32, TroopType>);

impl FromWorld for TroopTypes {
    fn from_world(world: &mut World) -> Self {
        let troops = world.get_resource::<TroopAssets>().unwrap();
        let assets = world.get_resource::<Assets<TroopType>>().unwrap();
        let mut map = HashMap::new();
        troops.troops.iter().for_each(|troop| {
            if let Some(troop) = assets.get(troop) {
                map.insert(troop.id, troop.clone());
            }
        });
        Self(map)
    }
}

impl TroopTypes {
    pub fn get(&self, id: i32) -> Option<TroopType> {
        self.0.get(&id).cloned()
    }
}

#[derive(Component)]
pub struct Troop {
    pub troop_type: TroopType,
    pub health: i32,
    pub target: Option<(Vec2, f32)>,
    pub seen_troops: Vec<Entity>,
}

impl Troop {
    pub fn new(troop_type: TroopType) -> Self {
        Self {
            health: troop_type.health,
            troop_type,
            target: None,
            seen_troops: Vec::new(),
        }
    }

    pub fn scan(&mut self, seen_troops: Vec<Entity>) {
        self.seen_troops = seen_troops;
    }
}

#[derive(Component, Copy, Clone)]
pub struct Faction {
    pub faction_id: i32,
}

impl Faction {
    pub fn player() -> Self {
        Faction { faction_id: 0 }
    }

    pub fn enemy() -> Self {
        Faction { faction_id: 1 }
    }

    fn color(&self) -> Color {
        match self.faction_id {
            0 => Color::GREEN,
            1 => Color::RED,
            _ => Color::YELLOW,
        }
    }
}

#[derive(Component)]
pub struct TroopCooldown(pub f32);

#[derive(Bundle)]
pub struct FactionIndicatorBundle {
    sprite: SpriteSheetBundle,
    faction: Faction,
}

#[derive(Bundle)]
pub struct TroopBundle {
    sprite: SpriteSheetBundle,
    troop: Troop,
    faction: Faction,
}

pub fn spawn_troop<'a, 'b, 'c>(
    commands: &'c mut Commands<'a, 'b>,
    position: Vec2,
    texture_atlas: Handle<TextureAtlas>,
    troop: TroopType,
    faction: Faction,
) {
    let faction_indicator = commands
        .spawn(FactionIndicatorBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                sprite: TextureAtlasSprite {
                    index: 15,
                    color: faction.color(),
                    ..Default::default()
                },
                ..Default::default()
            },
            faction,
        })
        .id();
    commands
        .spawn(TroopBundle {
            sprite: SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite {
                    index: troop.sprite_index,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 1.)),
                ..Default::default()
            },
            faction,
            troop: Troop::new(troop),
        })
        .add_child(faction_indicator);
}

fn troop_cooldown_system(
    mut commands: Commands,
    mut troops_with_cooldowns: Query<(Entity, &mut TroopCooldown)>,
    time: Res<Time>,
) {
    for (entity, mut troop) in troops_with_cooldowns.iter_mut() {
        troop.0 -= time.delta_seconds();
        if troop.0 < 0. {
            commands.entity(entity).remove::<TroopCooldown>();
        }
    }
}

fn troop_movement_system(
    mut troops: Query<(&GlobalTransform, &mut Transform, &mut Troop)>,
    time: Res<Time>,
) {
    for (global, mut transform, mut troop) in troops.iter_mut() {
        if let Some((target, speed)) = troop.target {
            let delta = target - Vec2::new(global.translation().x, global.translation().y);
            if delta.length_squared()
                > (speed * speed * time.delta_seconds() * time.delta_seconds())
            {
                let travel = delta.normalize() * time.delta_seconds() * speed;
                transform.translation.x += travel.x;
                transform.translation.y += travel.y;
            } else {
                troop.target = None;
            }
        }
    }
}

fn troop_restitution_system(
    mut troops: Query<(&GlobalTransform, &mut Transform, &Troop)>,
    time: Res<Time>,
) {
    let mut iter = troops.iter_combinations_mut();
    while let Some([(global_a, mut transform_a, troop_a), (global_b, mut transform_b, troop_b)]) =
        iter.fetch_next()
    {
        let delta = global_b.translation() - global_a.translation();
        let distance = delta.length();
        let size_bar = troop_a.troop_type.size + troop_b.troop_type.size;

        if distance > 0. && distance < size_bar {
            let restitution = delta.normalize() * (distance - size_bar) / 2.;
            restitution.clamp_length_max(time.delta_seconds() * 16.);
            transform_a.translation += restitution;
            transform_b.translation -= restitution;
        }
    }
}

fn troop_battle_action_system(
    mut commands: Commands,
    mut script_env: WasmScriptComponentEnv<Troop, ()>,
    troops: Query<(Entity, &Troop), Without<TroopCooldown>>,
) {
    for (entity, troop) in troops.iter() {
        match script_env.call_if_instantiated_1::<f64, f32>(
            troop.get_wasm_script_handle(),
            "battle_action",
            EntityId::from_entity(entity),
        ) {
            Ok(cooldown) => {
                if cooldown > 0. {
                    commands.entity(entity).insert(TroopCooldown(cooldown));
                }
            }
            Err(err) => {
                error!("Could not execute battle action: {}", err);
            }
        }
    }
}

fn troop_staging_system(
    mut staging_locations: Query<(&GlobalTransform, &mut StagingLocation)>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    troop_types: Res<TroopTypes>,
) {
    for (transform, mut staging_location) in staging_locations.iter_mut() {
        staging_location.staged.drain(..).for_each(|troop_type| {
            if let Some(troop_type) = troop_types.get(troop_type) {
                let position = Vec2::new(transform.translation().x, transform.translation().y)
                    + Vec2::new(
                        32. - rand::random::<f32>() * 64.,
                        32. - rand::random::<f32>() * 64.,
                    );
                spawn_troop(
                    &mut commands,
                    position,
                    textures.troops.clone(),
                    troop_type,
                    Faction::player(),
                );
            }
        });
    }
}
