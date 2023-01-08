use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::HashMap,
};
use serde::Deserialize;

use crate::{
    battle::{Faction, StagingLocation, Troop, TroopCooldown},
    helper::HelperTextBundle,
    loading::{FontAssets, TextureAssets, WaveAssets},
    GameState, SafeInsert,
};

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Wave>()
            .init_resource::<CurrentWave>()
            .init_resource::<InvasionSpots>()
            .init_asset_loader::<WaveAssetLoader>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_wave_ui))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(wave_describe_system),
            )
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(restart_game))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(game_over_system))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(wave_spawning_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(wave_ending_system),
            );
    }
}
#[derive(Clone, Deserialize, TypeUuid)]
#[uuid = "76a9455a-7e0b-432a-b7d9-3bfc114d55cf"]
pub struct Wave {
    pub id: i32,
    pub north: Vec<(i32, HashMap<i32, f32>)>,
    pub east: Vec<(i32, HashMap<i32, f32>)>,
    pub south: Vec<(i32, HashMap<i32, f32>)>,
    pub west: Vec<(i32, HashMap<i32, f32>)>,
}

impl Wave {
    pub fn score(&self, time: f32) -> f32 {
        let troops = self.north.len() + self.east.len() + self.south.len() + self.west.len();
        let troop_value = troops as f32 * 5.;
        let modifier = (self.id + 1) as f32 * 10.;
        let time_loss = time * 2.;
        troop_value + modifier + time_loss
    }
}

#[derive(Default)]
pub struct WaveAssetLoader;

impl AssetLoader for WaveAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<Wave>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["wave"]
    }
}

#[derive(Resource)]
pub struct Waves(pub HashMap<i32, Wave>);

impl FromWorld for Waves {
    fn from_world(world: &mut World) -> Self {
        let waves = world.get_resource::<WaveAssets>().unwrap();
        let assets = world.get_resource::<Assets<Wave>>().unwrap();
        let mut map = HashMap::new();
        waves.waves.iter().for_each(|wave| {
            if let Some(wave) = assets.get(wave) {
                map.insert(wave.id, wave.clone());
            }
        });
        map.insert(
            -1,
            Wave {
                id: -1,
                north: vec![],
                east: vec![],
                south: vec![],
                west: vec![],
            },
        );
        Self(map)
    }
}

impl Waves {
    pub fn get(&self, id: i32) -> Wave {
        self.0
            .get(&id)
            .or_else(|| self.0.get(&-1))
            .cloned()
            .unwrap()
    }
}

pub const SPAWN_TIME: f32 = 5.;
pub const NORTH_X: f32 = 0.;
pub const NORTH_Y: f32 = 228.;
pub const EAST_X: f32 = 442.;
pub const EAST_Y: f32 = 0.;
pub const SOUTH_X: f32 = 0.;
pub const SOUTH_Y: f32 = -228.;
pub const WEST_X: f32 = -442.;
pub const WEST_Y: f32 = 0.;

#[derive(Resource, Default)]
pub struct InvasionSpots {
    north: Option<Entity>,
    east: Option<Entity>,
    south: Option<Entity>,
    west: Option<Entity>,
}

#[derive(Resource)]
pub struct CurrentWave {
    pub wave: Wave,
    pub time_in_wave: f32,
    pub score: f32,
    pub spawned: usize,
}

impl Default for CurrentWave {
    fn default() -> Self {
        Self {
            wave: Wave {
                id: -1,
                north: vec![],
                east: vec![],
                south: vec![],
                west: vec![],
            },
            time_in_wave: -5.,
            score: 0.,
            spawned: 0,
        }
    }
}

impl CurrentWave {
    pub fn game_over(&mut self) {
        self.wave = Wave {
            id: -2,
            north: vec![],
            east: vec![],
            south: vec![],
            west: vec![],
        };
        self.time_in_wave = -60.;
    }
    pub fn go_to_next_wave(&mut self, wave: Wave) {
        self.score = self.score + wave.score(self.time_in_wave);
        self.wave = wave;
        self.time_in_wave = -15.;
        self.spawned = 0;
    }
    pub fn max_spawned(&self) -> usize {
        [
            self.wave.north.len(),
            self.wave.east.len(),
            self.wave.south.len(),
            self.wave.west.len(),
        ]
        .iter()
        .max()
        .cloned()
        .unwrap()
    }
}

#[derive(Bundle)]
pub struct EnemyStagingBundle {
    sprite: SpriteSheetBundle,
    faction: Faction,
    staging_location: StagingLocation,
}

fn spawn_invader_staging(
    commands: &mut Commands,
    font: Handle<Font>,
    texture: Handle<TextureAtlas>,
    x: f32,
    y: f32,
) -> Entity {
    let helper = commands
        .spawn(HelperTextBundle::new("Invaders can come from here!", font))
        .id();
    commands
        .spawn(EnemyStagingBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: texture,
                sprite: TextureAtlasSprite {
                    index: 5,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                ..Default::default()
            },
            faction: Faction::enemy(),
            staging_location: Default::default(),
        })
        .add_child(helper)
        .id()
}

#[derive(Component)]
pub struct WaveText;

fn setup_wave_ui(mut commands: Commands, fonts: Res<FontAssets>) {
    commands.spawn((
        TextBundle::from_section(
            "Wave XX",
            TextStyle {
                font: fonts.fira_sans.clone(),
                font_size: 24.,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Relative,
            position: UiRect {
                top: Val::Px(8.),
                left: Val::Percent(50.),
                ..Default::default()
            },
            ..Default::default()
        }),
        WaveText,
    ));
}

fn restart_game(
    mut current_wave: ResMut<CurrentWave>,
    mut state: ResMut<State<GameState>>,
    input: Res<Input<KeyCode>>,
) {
    if current_wave.wave.id <= -2 && input.just_pressed(KeyCode::Escape) {
        current_wave.go_to_next_wave(Wave {
            id: -1,
            north: vec![],
            east: vec![],
            south: vec![],
            west: vec![],
        });
        state.set(GameState::Menu);
    }
}

fn wave_describe_system(
    current_wave: ResMut<CurrentWave>,
    fonts: Res<FontAssets>,
    mut wave_text: Query<(&mut Text), With<WaveText>>,
) {
    let main_style = TextStyle {
        color: Color::WHITE,
        font: fonts.fira_sans.clone(),
        font_size: 24.,
    };
    if current_wave.wave.id >= 0 {
        *wave_text.single_mut() = Text::from_sections([
            TextSection::new(format!("Wave {}\n", current_wave.wave.id + 1), main_style),
            TextSection::new(
                format!("Time - {}", current_wave.time_in_wave as i32),
                TextStyle {
                    color: Color::WHITE,
                    font: fonts.fira_sans.clone(),
                    font_size: 18.,
                },
            ),
        ]);
    } else if current_wave.wave.id == -1 {
        *wave_text.single_mut() = Text::from_sections([
            TextSection::new("Welcome!\n", main_style),
            TextSection::new(
                "The first wave will spawn when you have deployed a troop.",
                TextStyle {
                    color: Color::WHITE,
                    font: fonts.fira_sans.clone(),
                    font_size: 18.,
                },
            ),
        ]);
    } else if current_wave.wave.id == -2 {
        *wave_text.single_mut() = Text::from_sections([
            TextSection::new("Game Over!\n", main_style),
            TextSection::new(
                "Your king has died...\n",
                TextStyle {
                    color: Color::RED,
                    font: fonts.fira_sans.clone(),
                    font_size: 18.,
                },
            ),
            TextSection::new(
                format!("You achieved a score of {}\n", current_wave.score),
                TextStyle {
                    color: Color::YELLOW,
                    font: fonts.fira_sans.clone(),
                    font_size: 18.,
                },
            ),
            TextSection::new(
                "Press ESC to return to the main menu.\n",
                TextStyle {
                    color: Color::YELLOW,
                    font: fonts.fira_sans.clone(),
                    font_size: 18.,
                },
            ),
        ]);
    } else if current_wave.wave.id == -3 {
        *wave_text.single_mut() = Text::from_sections([
            TextSection::new("Game Over!\n", main_style),
            TextSection::new(
                "You have fended off the invaders!\n",
                TextStyle {
                    color: Color::GREEN,
                    font: fonts.fira_sans.clone(),
                    font_size: 24.,
                },
            ),
            TextSection::new(
                format!("You achieved a score of {}\n", current_wave.score),
                TextStyle {
                    color: Color::YELLOW,
                    font: fonts.fira_sans.clone(),
                    font_size: 18.,
                },
            ),
            TextSection::new(
                "Press ESC to return to the main menu.\n",
                TextStyle {
                    color: Color::YELLOW,
                    font: fonts.fira_sans.clone(),
                    font_size: 18.,
                },
            ),
        ]);
    }
}

fn game_over_system(
    mut current_wave: ResMut<CurrentWave>,
    mut commands: Commands,
    troops: Query<(Entity, &Troop, &Faction)>,
) {
    if current_wave.wave.id < 0 {
        // Pre-initialized state?
        return;
    }
    for (entity, troop, faction) in troops.iter() {
        if troop.troop_type.id == 87 {
            return;
        }
    }
    for (entity, _troop, faction) in troops.iter() {
        if faction.faction_id == Faction::player().faction_id {
            commands.add(SafeInsert::new(entity, TroopCooldown(999.)));
        }
    }
    current_wave.game_over();
}

fn wave_ending_system(
    mut current_wave: ResMut<CurrentWave>,
    enemies: Query<(&Troop, &Faction)>,
    waves: Res<Waves>,
) {
    if current_wave.time_in_wave > SPAWN_TIME + 2. {
        if !enemies
            .iter()
            .any(|(_troop, faction)| faction.faction_id == Faction::enemy().faction_id)
        {
            let next_wave = waves.get(current_wave.wave.id + 1);
            if next_wave.id == -1 {
                current_wave.go_to_next_wave(Wave {
                    id: -3,
                    north: vec![],
                    east: vec![],
                    south: vec![],
                    west: vec![],
                })
            } else {
                current_wave.go_to_next_wave(next_wave);
            }
        }
    }
}

fn wave_spawning_system(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
    mut current_wave: ResMut<CurrentWave>,
    mut invasions: ResMut<InvasionSpots>,
    mut staging: Query<&mut StagingLocation>,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_seconds();
    current_wave.time_in_wave += delta_seconds;
    if current_wave.time_in_wave > 0. {
        let wanted_spawn =
            (current_wave.max_spawned() as f32 * current_wave.time_in_wave / SPAWN_TIME) as usize;
        if current_wave.spawned < wanted_spawn {
            for i in (current_wave.spawned)..wanted_spawn {
                if let Some((troop, buffs)) = current_wave.wave.north.get(i) {
                    staging
                        .get_mut(invasions.north.unwrap())
                        .map(|mut staging| staging.stage_with_buffs(*troop, buffs.clone()))
                        .unwrap();
                }
                if let Some((troop, buffs)) = current_wave.wave.east.get(i) {
                    staging
                        .get_mut(invasions.east.unwrap())
                        .map(|mut staging| staging.stage_with_buffs(*troop, buffs.clone()))
                        .unwrap();
                }
                if let Some((troop, buffs)) = current_wave.wave.south.get(i) {
                    staging
                        .get_mut(invasions.south.unwrap())
                        .map(|mut staging| staging.stage_with_buffs(*troop, buffs.clone()))
                        .unwrap();
                }
                if let Some((troop, buffs)) = current_wave.wave.west.get(i) {
                    staging
                        .get_mut(invasions.west.unwrap())
                        .map(|mut staging| staging.stage_with_buffs(*troop, buffs.clone()))
                        .unwrap();
                }
            }
            current_wave.spawned = wanted_spawn;
        }
        if current_wave.time_in_wave > SPAWN_TIME + 2. {
            if let Some(entity) = invasions.north {
                if let Some(entity) = commands.get_entity(entity) {
                    entity.despawn_recursive();
                }
                invasions.north = None;
            }
            if let Some(entity) = invasions.east {
                if let Some(entity) = commands.get_entity(entity) {
                    entity.despawn_recursive();
                }
                invasions.east = None;
            }
            if let Some(entity) = invasions.south {
                if let Some(entity) = commands.get_entity(entity) {
                    entity.despawn_recursive();
                }
                invasions.south = None;
            }
            if let Some(entity) = invasions.west {
                if let Some(entity) = commands.get_entity(entity) {
                    entity.despawn_recursive();
                }
                invasions.west = None;
            }
        }
    } else {
        if invasions.north.is_none() && current_wave.wave.north.len() > 0 {
            invasions.north = Some(spawn_invader_staging(
                &mut commands,
                fonts.fira_sans.clone(),
                textures.locations.clone(),
                NORTH_X,
                NORTH_Y,
            ));
        }
        if invasions.east.is_none() && current_wave.wave.east.len() > 0 {
            invasions.east = Some(spawn_invader_staging(
                &mut commands,
                fonts.fira_sans.clone(),
                textures.locations.clone(),
                EAST_X,
                EAST_Y,
            ));
        }
        if invasions.south.is_none() && current_wave.wave.south.len() > 0 {
            invasions.south = Some(spawn_invader_staging(
                &mut commands,
                fonts.fira_sans.clone(),
                textures.locations.clone(),
                SOUTH_X,
                SOUTH_Y,
            ));
        }
        if invasions.west.is_none() && current_wave.wave.west.len() > 0 {
            invasions.west = Some(spawn_invader_staging(
                &mut commands,
                fonts.fira_sans.clone(),
                textures.locations.clone(),
                WEST_X,
                WEST_Y,
            ));
        }
    }
}
