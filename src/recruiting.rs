use bevy::prelude::*;

use crate::{
    delivery::*, harvest::spawn_harvest_spot, helper::HelperTextBundle, loading::*, GameState,
};

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
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
    scripts: Res<DeliveryScripts>,
) {
    let helper = commands
        .spawn(HelperTextBundle::new(
            "Boot Camp - Send recruits to further training to gain soldiers",
            fonts.fira_sans.clone(),
        ))
        .id();
    commands
        .spawn(RecruiterBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: textures.locations.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-64., 0., 1.)),
                ..Default::default()
            },
            delivery_anchor: DeliveryAnchor::new(0., -16., 32., 32 * 32),
            delivery_source: DeliverySource::new(scripts.recruitment.clone()),
        })
        .add_child(helper);
}

fn spawn_training_ground(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
    scripts: Res<DeliveryScripts>,
) {
    let helper = commands
        .spawn(HelperTextBundle::new(
            "Basic Training - Grow recruits and harvest soldiers",
            fonts.fira_sans.clone(),
        ))
        .id();
    let spot_one = spawn_harvest_spot(
        &mut commands,
        Vec2::new(-48., -16.),
        fonts.fira_sans.clone(),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
        Visibility::INVISIBLE,
    );
    let spot_two = spawn_harvest_spot(
        &mut commands,
        Vec2::new(0., -36.),
        fonts.fira_sans.clone(),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
        Visibility::INVISIBLE,
    );
    let spot_three = spawn_harvest_spot(
        &mut commands,
        Vec2::new(48., -16.),
        fonts.fira_sans.clone(),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
        Visibility::INVISIBLE,
    );
    commands
        .spawn(TrainerBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: textures.locations.clone(),
                sprite: TextureAtlasSprite {
                    index: 2,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-64., 64., 1.)),
                ..Default::default()
            },
            delivery_anchor: DeliveryAnchor::new(0., -16., 32., 32 * 32),
            delivery_dropoff: DeliveryDropoff::new(scripts.practice_field.clone()),
        })
        .add_child(spot_one)
        .add_child(spot_two)
        .add_child(spot_three)
        .add_child(helper);
}

fn spawn_archery_field(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
    scripts: Res<DeliveryScripts>,
) {
    let helper = commands
        .spawn(HelperTextBundle::new(
            "Bow Training - Grow recruits and harvest archers",
            fonts.fira_sans.clone(),
        ))
        .id();
    let spot_one = spawn_harvest_spot(
        &mut commands,
        Vec2::new(-48., -16.),
        fonts.fira_sans.clone(),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
        Visibility::INVISIBLE,
    );
    let spot_two = spawn_harvest_spot(
        &mut commands,
        Vec2::new(0., -36.),
        fonts.fira_sans.clone(),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
        Visibility::INVISIBLE,
    );
    let spot_three = spawn_harvest_spot(
        &mut commands,
        Vec2::new(48., -16.),
        fonts.fira_sans.clone(),
        textures.harvest_base.clone(),
        scripts.child_spot.clone(),
        Visibility::INVISIBLE,
    );
    commands
        .spawn(TrainerBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: textures.locations.clone(),
                sprite: TextureAtlasSprite {
                    index: 3,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-64., -64., 1.)),
                ..Default::default()
            },
            delivery_anchor: DeliveryAnchor::new(0., -16., 32., 32 * 32),
            delivery_dropoff: DeliveryDropoff::new(scripts.archery_field.clone()),
        })
        .add_child(spot_one)
        .add_child(spot_two)
        .add_child(spot_three)
        .add_child(helper);
}
