use bevy::{
    asset::{AssetLoader, LoadedAsset},
    ecs::system::EntityCommands,
    prelude::*,
    reflect::TypeUuid,
    utils::HashMap,
};
use bevy_wasm_scripting::*;
use serde::Deserialize;

use crate::{battle::Troop, loading::AttackAssets};

#[derive(Clone, Deserialize, TypeUuid)]
#[uuid = "7969889c-91e5-4d22-b6b4-55b0eaeed27d"]
pub struct AttackType {
    pub id: i32,
    pub phases: Vec<AttackPhase>,
}

#[derive(Clone, Deserialize)]
pub enum AttackPhase {
    Projectile { sprite_index: usize, speed: f32 },
    Overlay { sprite_index: usize, duration: f32 },
    Damage { amount: i32 },
    Hidden { duration: f32 },
}

#[derive(Default)]
pub struct AttackAssetLoader;

impl AssetLoader for AttackAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<AttackType>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["attack"]
    }
}

#[derive(Resource)]
pub struct AttackTypes(pub HashMap<i32, AttackType>);

impl FromWorld for AttackTypes {
    fn from_world(world: &mut World) -> Self {
        let attacks = world.get_resource::<AttackAssets>().unwrap();
        let assets = world.get_resource::<Assets<AttackType>>().unwrap();
        let mut map = HashMap::new();
        attacks.attacks.iter().for_each(|attack| {
            if let Some(attack) = assets.get(attack) {
                map.insert(attack.id, attack.clone());
            }
        });
        Self(map)
    }
}

impl AttackTypes {
    pub fn get(&self, id: i32) -> Option<AttackType> {
        self.0.get(&id).cloned()
    }
}

#[derive(Component)]
pub struct Attack {
    attacker: Entity,
    target: Entity,
    attack_type: AttackType,
    phase: Option<AttackPhase>,
    remaining_phases: Vec<AttackPhase>,
}

#[derive(Bundle)]
pub struct AttackBundle {
    sprite: SpriteSheetBundle,
    attack: Attack,
}

pub fn spawn_attack<'a, 'b, 'c>(
    commands: &'c mut Commands<'a, 'b>,
    attacker: Entity,
    target: Entity,
    texture_atlas: Handle<TextureAtlas>,
    attack_type: AttackType,
) {
    println!("Spawning attack...");
    commands.spawn(AttackBundle {
        sprite: SpriteSheetBundle {
            texture_atlas,
            visibility: Visibility::INVISIBLE,
            ..Default::default()
        },
        attack: Attack {
            attacker,
            target,
            remaining_phases: attack_type.phases.clone(),
            attack_type,
            phase: None,
        },
    });
}

pub fn attack_phase_system(
    mut commands: Commands,
    mut attacks: Query<(
        Entity,
        &mut Attack,
        &mut TextureAtlasSprite,
        &mut Transform,
        &mut Visibility,
    )>,
    mut troops: Query<(&mut Troop, &GlobalTransform)>,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_seconds();
    for (entity, mut attack, sprite, transform, visibility) in attacks.iter_mut() {
        let target = troops.get_mut(attack.target);
        if target.is_err() {
            commands.entity(entity).despawn();
        }
    }
    for (entity, mut attack, mut sprite, mut transform, mut visibility) in attacks.iter_mut() {
        let target = attack.target.clone();
        match &mut attack.phase {
            Some(AttackPhase::Projectile {
                sprite_index,
                speed,
            }) => {
                if let Ok(target) = troops.get(target) {
                    let delta = target.1.translation() - transform.translation;
                    if delta.length_squared() < delta_seconds * delta_seconds * *speed * *speed {
                        attack.phase = None;
                    } else {
                        let delta = delta.normalize();
                        transform.translation += delta * delta_seconds * *speed;
                        transform.rotation = Quat::from_rotation_z(f32::atan2(delta.y, delta.x));
                    }
                }
            }
            Some(AttackPhase::Overlay {
                sprite_index,
                duration,
            }) => {
                *duration -= delta_seconds;
                if *duration < 0. {
                    attack.phase = None;
                }
            }
            Some(AttackPhase::Damage { amount }) => {
                if let Ok(mut target) = troops.get_mut(target) {
                    target.0.health -= *amount;
                }
                attack.phase = None;
            }
            Some(AttackPhase::Hidden { duration }) => {
                *duration -= delta_seconds;
                if *duration < 0. {
                    attack.phase = None;
                }
            }
            None => {}
        }
        if attack.phase.is_none() {
            if attack.remaining_phases.len() > 0 {
                attack.phase = Some(attack.remaining_phases.remove(0));
                match attack.phase.as_ref().unwrap() {
                    AttackPhase::Projectile {
                        sprite_index,
                        speed,
                    } => {
                        if let Ok((_troop, my_global)) = troops.get(attack.attacker) {
                            transform.translation.x = my_global.translation().x;
                            transform.translation.y = my_global.translation().y;
                            transform.translation.z = 20.;
                        }
                        sprite.index = *sprite_index;
                        *visibility = Visibility::VISIBLE;
                    }
                    AttackPhase::Overlay {
                        sprite_index,
                        duration,
                    } => {
                        if let Ok((_troop, my_global)) = troops.get(attack.target) {
                            transform.translation.x = my_global.translation().x;
                            transform.translation.y = my_global.translation().y;
                            transform.translation.z = 20.;
                        }
                        sprite.index = *sprite_index;
                        *visibility = Visibility::VISIBLE;
                    }
                    AttackPhase::Damage { amount } => {
                        *visibility = Visibility::INVISIBLE;
                    }
                    AttackPhase::Hidden { duration } => {
                        *visibility = Visibility::INVISIBLE;
                    }
                }
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}
