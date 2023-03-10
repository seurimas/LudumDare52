use bevy::prelude::*;

use crate::{delivery::*, helper::HelperTextBundle, loading::*, GameState};

pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_market));
    }
}

#[derive(Bundle)]
pub struct MarketBundle {
    sprite: SpriteSheetBundle,
    delivery_source: DeliverySource,
    delivery_location: DeliveryDropoff,
    delivery_anchor: DeliveryAnchor,
}

fn spawn_market(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
    scripts: Res<DeliveryScripts>,
) {
    let helper = commands
        .spawn(HelperTextBundle::new(
            "Marketplace - Drag seeds to nearby plots to grow food for soldiers",
            fonts.fira_sans.clone(),
        ))
        .id();
    commands
        .spawn(MarketBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: textures.locations.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(128., 64., 1.)),
                ..Default::default()
            },
            delivery_anchor: DeliveryAnchor::new(0., -16., 32., 32 * 32),
            delivery_source: DeliverySource::new(scripts.market.clone()),
            delivery_location: DeliveryDropoff::new(scripts.market.clone()),
        })
        .add_child(helper);
}
