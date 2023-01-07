use bevy::prelude::*;

use crate::{delivery::*, harvest::spawn_harvest_spot, loading::*, GameState};

pub struct RecruitingPlugin;

impl Plugin for RecruitingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_recruitment))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(spawn_training_ground),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(spawn_archery_field),
            );
    }
}

#[derive(Bundle)]
pub struct RecruiterBundle {
    sprite: SpriteSheetBundle,
    delivery_source: DeliverySource,
    delivery_anchor: DeliveryAnchor,
}

#[derive(Bundle)]
pub struct TrainerBundle {
    sprite: SpriteSheetBundle,
    delivery_dropoff: DeliveryDropoff,
    delivery_anchor: DeliveryAnchor,
}

fn spawn_recruitment(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    scripts: Res<DeliveryScripts>,
) {
    commands.spawn(RecruiterBundle {
        sprite: SpriteSheetBundle {
            texture_atlas: textures.locations.clone(),
            sprite: TextureAtlasSprite {
                index: 1,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-128., 0., 1.)),
            ..Default::default()
        },
        delivery_anchor: DeliveryAnchor::new(0., -16., 32., 32 * 32),
        delivery_source: DeliverySource::new(scripts.recruitment.clone()),
    });
}

fn spawn_training_ground(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    scripts: Res<DeliveryScripts>,
) {
    let spot_one = spawn_harvest_spot(
        &mut commands,
        Vec2::new(-48., -16.),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
    )
    .insert(Visibility::INVISIBLE)
    .id();
    let spot_two = spawn_harvest_spot(
        &mut commands,
        Vec2::new(0., -36.),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
    )
    .insert(Visibility::INVISIBLE)
    .id();
    let spot_three = spawn_harvest_spot(
        &mut commands,
        Vec2::new(48., -16.),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
    )
    .insert(Visibility::INVISIBLE)
    .id();
    commands
        .spawn(TrainerBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: textures.locations.clone(),
                sprite: TextureAtlasSprite {
                    index: 2,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-128., 128., 1.)),
                ..Default::default()
            },
            delivery_anchor: DeliveryAnchor::new(0., -16., 32., 32 * 32),
            delivery_dropoff: DeliveryDropoff::new(scripts.practice_field.clone()),
        })
        .add_child(spot_one)
        .add_child(spot_two)
        .add_child(spot_three);
}

fn spawn_archery_field(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    scripts: Res<DeliveryScripts>,
) {
    let spot_one = spawn_harvest_spot(
        &mut commands,
        Vec2::new(-48., -16.),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
    )
    .insert(Visibility::INVISIBLE)
    .id();
    let spot_two = spawn_harvest_spot(
        &mut commands,
        Vec2::new(0., -36.),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
    )
    .insert(Visibility::INVISIBLE)
    .id();
    let spot_three = spawn_harvest_spot(
        &mut commands,
        Vec2::new(48., -16.),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
    )
    .insert(Visibility::INVISIBLE)
    .id();
    commands
        .spawn(TrainerBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: textures.locations.clone(),
                sprite: TextureAtlasSprite {
                    index: 3,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-128., -128., 1.)),
                ..Default::default()
            },
            delivery_anchor: DeliveryAnchor::new(0., -16., 32., 32 * 32),
            delivery_dropoff: DeliveryDropoff::new(scripts.archery_field.clone()),
        })
        .add_child(spot_one)
        .add_child(spot_two)
        .add_child(spot_three);
}