mod attacks;
mod audio;
mod battle;
mod battle_scripting;
mod common_scripting;
mod delivery;
mod delivery_scripting;
mod harvest;
mod helper;
mod loading;
mod market;
mod menu;
mod recruiting;
mod wave;

use crate::audio::InternalAudioPlugin;
use crate::harvest::HarvestPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use battle::BattlePlugin;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::{app::App, ecs::system::Command};
use delivery::DeliveryPlugin;
use helper::{helper_text_system, HelperPlugin};
use market::MarketPlugin;
use recruiting::RecruitingPlugin;
use wave::WavePlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(BattlePlugin)
            .add_plugin(DeliveryPlugin)
            .add_plugin(MarketPlugin)
            .add_plugin(HarvestPlugin)
            .add_plugin(RecruitingPlugin)
            .add_plugin(WavePlugin)
            .add_plugin(HelperPlugin)
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(
                |mut command: Commands, entities: Query<Entity>| {
                    for entity in entities.iter() {
                        command.entity(entity).despawn();
                    }
                },
            ));

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
pub struct SafeInsert<T> {
    pub entity: Entity,
    pub bundle: T,
}

impl<T: Bundle> SafeInsert<T> {
    pub fn new(entity: Entity, bundle: T) -> Self {
        Self { entity, bundle }
    }
}

impl<T> Command for SafeInsert<T>
where
    T: Bundle + 'static,
{
    fn write(self, world: &mut World) {
        if let Some(mut entity) = world.get_entity_mut(self.entity) {
            entity.insert(self.bundle);
        } else {
            // panic!("error[B0003]: Could not insert a bundle (of type `{}`) for entity {:?} because it doesn't exist in this World.", std::any::type_name::<T>(), self.entity);
        }
    }
}
