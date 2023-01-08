use bevy::{
    prelude::*,
    text::{Text2dBounds, Text2dSize},
};

use crate::{
    battle::{Faction, Troop},
    common_scripting::{ScriptValues, ATTACK_SPEED_MOD_ID, DODGE_CHANCE_ID, SPEED_MOD_ID},
    harvest::HarvestSpot,
    loading::FontAssets,
    GameState,
};

pub struct HelperPlugin;

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(helper_text_system),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(helper_troop_system))
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(helper_harvest_system),
        );
    }
}

#[derive(Component)]
pub struct HelperText {
    max_distance_squared: i32,
}

#[derive(Bundle)]
pub struct HelperTextBundle {
    pub text: Text2dBundle,
    pub helper_text: HelperText,
}

impl HelperTextBundle {
    pub fn new(text: &str, font: Handle<Font>) -> Self {
        HelperTextBundle {
            text: Text2dBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font,
                        font_size: 14.,
                        color: Color::MIDNIGHT_BLUE,
                    },
                ),
                text_2d_bounds: Text2dBounds {
                    size: Vec2::new(200., 500.),
                },
                ..Default::default()
            },
            helper_text: HelperText {
                max_distance_squared: 32 * 32,
            },
        }
    }
}

pub fn helper_text_system(
    mut mouse_location: Local<Vec2>,
    camera: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<&GlobalTransform>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut helper_texts: Query<(
        &HelperText,
        &Parent,
        &mut Transform,
        &mut Visibility,
        &Text2dSize,
    )>,
) {
    let (camera, camera_transform) = camera.single();
    for event in cursor_moved.iter() {
        *mouse_location = event.position;
    }
    let mouse_world_location = camera
        .viewport_to_world(camera_transform, *mouse_location)
        .unwrap()
        .origin;
    let mouse_world_location = Vec2::new(mouse_world_location.x, mouse_world_location.y);
    for (_text, _parent, _transform, mut visibility, _size) in helper_texts.iter_mut() {
        *visibility = Visibility::INVISIBLE;
    }
    if let Some((text, parent, mut text_transform, mut visibility, size)) = helper_texts
        .iter_mut()
        .min_by_key(|(_text, parent, _transform, _visibility, _size)| {
            if let Ok(transform) = transforms.get(parent.get()) {
                let txy = Vec2::new(transform.translation().x, transform.translation().y);
                txy.distance_squared(mouse_world_location) as i32
            } else {
                i32::MAX
            }
        })
    {
        if let Ok(transform) = transforms.get(parent.get()) {
            let txy = Vec2::new(transform.translation().x, transform.translation().y);
            let distance_to_cursor = txy.distance_squared(mouse_world_location) as i32;
            if distance_to_cursor < text.max_distance_squared {
                *visibility = Visibility::VISIBLE;
                text_transform.translation = Vec3::new(-size.size.x / 2., 48., 100.);
            }
        }
    }
}

pub fn helper_troop_system(
    fonts: Res<FontAssets>,
    mut helper_texts: Query<(&mut Text, &Parent), With<HelperText>>,
    troops: Query<(&Troop, &Faction, &ScriptValues)>,
) {
    for (mut text, parent) in helper_texts.iter_mut() {
        if let Ok((troop, faction, script_values)) = troops.get(parent.get()) {
            let mut description = vec![if faction.faction_id == Faction::player().faction_id {
                TextSection::new(
                    format!("Allied {}\n", troop.troop_type.name),
                    TextStyle {
                        color: Color::GREEN,
                        font: fonts.fira_sans.clone(),
                        font_size: 12.,
                    },
                )
            } else {
                TextSection::new(
                    format!("Enemy {}\n", troop.troop_type.name),
                    TextStyle {
                        color: Color::RED,
                        font: fonts.fira_sans.clone(),
                        font_size: 12.,
                    },
                )
            }];
            description.push(TextSection::new(
                format!("Health: {}/{}\n", troop.health, troop.troop_type.health),
                TextStyle {
                    color: Color::ORANGE_RED,
                    font: fonts.fira_sans.clone(),
                    font_size: 12.,
                },
            ));
            if let Some(speed_buff) = script_values.0.get(&SPEED_MOD_ID) {
                let speed_buff_count = ((speed_buff - 1.) / 0.1) as i32;
                description.push(TextSection::new(
                    format!("MOV {}\n", speed_buff_count),
                    TextStyle {
                        color: Color::BLUE,
                        font: fonts.fira_sans.clone(),
                        font_size: 14.,
                    },
                ));
            }
            if let Some(speed_buff) = script_values.0.get(&ATTACK_SPEED_MOD_ID) {
                let speed_buff_count = ((1. - speed_buff) / 0.1) as i32;
                description.push(TextSection::new(
                    format!("ATK {}\n", speed_buff_count),
                    TextStyle {
                        color: Color::BLUE,
                        font: fonts.fira_sans.clone(),
                        font_size: 14.,
                    },
                ));
            }
            if let Some(dodge_chance) = script_values.0.get(&DODGE_CHANCE_ID) {
                let dodge_chance_count = (dodge_chance / 0.05) as i32;
                description.push(TextSection::new(
                    format!("DODGE {}", dodge_chance_count),
                    TextStyle {
                        color: Color::BLUE,
                        font: fonts.fira_sans.clone(),
                        font_size: 14.,
                    },
                ));
            }
            *text = Text::from_sections(description);
        }
    }
}

pub fn helper_harvest_system(
    fonts: Res<FontAssets>,
    mut helper_texts: Query<(&mut Text, &Parent), With<HelperText>>,
    harvest_spots: Query<(&HarvestSpot, &ScriptValues)>,
) {
    for (mut text, parent) in helper_texts.iter_mut() {
        if let Ok((harvest_spot, script_values)) = harvest_spots.get(parent.get()) {
            let mut description = vec![if harvest_spot.harvestable_type.is_none() {
                TextSection::new(
                    "Plant something here!\n",
                    TextStyle {
                        color: Color::WHITE,
                        font: fonts.fira_sans.clone(),
                        font_size: 14.,
                    },
                )
            } else if harvest_spot.progress_percent() < 100 {
                TextSection::new(
                    format!(
                        "Growing {}: {}%\n",
                        harvest_spot.harvestable_type.as_ref().unwrap().name,
                        harvest_spot.progress_percent()
                    ),
                    TextStyle {
                        color: Color::WHITE,
                        font: fonts.fira_sans.clone(),
                        font_size: 18.,
                    },
                )
            } else {
                TextSection::new(
                    format!(
                        "Harvest {}!\n",
                        harvest_spot.harvestable_type.as_ref().unwrap().name
                    ),
                    TextStyle {
                        color: Color::WHITE,
                        font: fonts.fira_sans.clone(),
                        font_size: 18.,
                    },
                )
            }];
            if let Some(harvestable_type) = &harvest_spot.harvestable_type {
                description.push(TextSection::new(
                    harvestable_type.description.clone(),
                    TextStyle {
                        color: Color::BLUE,
                        font: fonts.fira_sans.clone(),
                        font_size: 16.,
                    },
                ));
            }
            *text = Text::from_sections(description);
        }
    }
}
