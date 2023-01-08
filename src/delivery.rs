use bevy::prelude::*;
use bevy_wasm_scripting::*;

use crate::GameState;

pub struct DeliveryPlugin;

impl Plugin for DeliveryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeliveryItem>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(
                ScriptSystemWithCommands::<_, DeliverySource>::wrap(IntoSystem::into_system(
                    delivery_sourcing_system,
                )),
            ))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(
                ScriptSystemWithCommands::<_, DeliverySource>::wrap(IntoSystem::into_system(
                    delivery_dropoff_system,
                )),
            ))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(delivery_dragging_system),
            )
            .add_wasm_script_component::<DeliverySource>()
            .add_wasm_script_component::<DeliveryDropoff>();
    }
}

#[derive(Resource)]
pub enum DeliveryItem {
    Nothing,
    FromSource { delivered: Entity, source: Entity },
}

impl Default for DeliveryItem {
    fn default() -> Self {
        Self::Nothing
    }
}

#[derive(Component)]
pub struct DeliveryDropoff {
    pub script: Handle<WasmScript>,
}

impl DeliveryDropoff {
    pub fn new(script: Handle<WasmScript>) -> Self {
        Self { script }
    }
}

#[derive(Component)]
pub struct DeliverySource {
    pub script: Handle<WasmScript>,
}

impl DeliverySource {
    pub fn new(script: Handle<WasmScript>) -> Self {
        Self { script }
    }
}

#[derive(Component)]
pub struct DeliveryAnchor {
    x: f32,
    y: f32,
    height: f32,
    distance_sq: i32,
}

impl DeliveryAnchor {
    pub fn new(x: f32, y: f32, height: f32, distance_sq: i32) -> Self {
        Self {
            x,
            y,
            height,
            distance_sq,
        }
    }
}

fn get_anchor_distance_sq(
    mouse_location: Vec3,
    transform: &GlobalTransform,
    anchor: &DeliveryAnchor,
) -> i32 {
    let anchor_base = Vec2::new(transform.translation().x, transform.translation().y)
        + Vec2::new(anchor.x, anchor.y);
    let mouse_location = if mouse_location.y > anchor_base.y + anchor.height {
        Vec2::new(mouse_location.x, mouse_location.y - anchor.height)
    } else if mouse_location.y > anchor_base.y {
        Vec2::new(mouse_location.x, anchor_base.y)
    } else {
        Vec2::new(mouse_location.x, mouse_location.y)
    };
    let result = mouse_location.distance_squared(anchor_base) as i32;
    result
}

fn delivery_dragging_system(
    mut mouse_location: Local<Vec2>,
    mut commands: Commands,
    parents: Query<&Parent>,
    mut transforms: Query<(&mut Visibility, &mut Transform)>,
    delivery_item: Res<DeliveryItem>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut cursor_moved: EventReader<CursorMoved>,
) {
    let (camera, camera_transform) = camera.single();
    for event in cursor_moved.iter() {
        *mouse_location = event.position;
    }
    let mouse_world_location = camera
        .viewport_to_world(camera_transform, *mouse_location)
        .unwrap()
        .origin;
    match *delivery_item {
        DeliveryItem::Nothing => {}
        DeliveryItem::FromSource {
            delivered,
            source: _,
        } => {
            if parents.get(delivered).is_ok() {
                if let Some(mut delivered) = commands.get_entity(delivered) {
                    delivered.remove_parent();
                }
            } else {
                if let Ok((mut visibility, mut transform)) = transforms.get_mut(delivered) {
                    *visibility = Visibility::VISIBLE;
                    transform.translation.x = mouse_world_location.x;
                    transform.translation.y = mouse_world_location.y;
                    transform.translation.z = 10.;
                }
            }
        }
    }
}

fn delivery_sourcing_system(
    mut mouse_location: Local<Vec2>,
    mut script_env: WasmScriptComponentEnv<DeliverySource, ()>,
    delivery_anchors: Query<(Entity, &GlobalTransform, &DeliverySource, &DeliveryAnchor)>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut cursor_moved: EventReader<CursorMoved>,
) {
    let (camera, camera_transform) = camera.single();
    for event in cursor_moved.iter() {
        *mouse_location = event.position;
    }
    let mouse_world_location = camera
        .viewport_to_world(camera_transform, *mouse_location)
        .unwrap()
        .origin;
    match *script_env.resources.0 {
        DeliveryItem::Nothing => {
            let closest_anchor = delivery_anchors
                .iter()
                .filter(|(entity, _delivery_transform, source, _anchor)| {
                    match script_env.call_if_instantiated_1::<EntityId, i8>(
                        &source.script,
                        "can_produce",
                        EntityId::from_entity(*entity),
                    ) {
                        Ok(can_produce) => {
                            if can_produce == 1 {
                                true
                            } else {
                                false
                            }
                        }
                        Err(err) => {
                            error!("Error in can_produce: {:?}", err);
                            false
                        }
                    }
                })
                .min_by_key(|(_entity, delivery_transform, _source, anchor)| {
                    get_anchor_distance_sq(mouse_world_location, delivery_transform, anchor)
                });
            if let Some((entity, delivery_transform, source, anchor)) = closest_anchor {
                let distance_to_anchor =
                    get_anchor_distance_sq(mouse_world_location, delivery_transform, anchor);
                if mouse_buttons.just_pressed(MouseButton::Left) {
                    if distance_to_anchor < anchor.distance_sq {
                        match script_env.call_if_instantiated_1::<EntityId, EntityId>(
                            &source.script,
                            "produce",
                            EntityId::from_entity(entity),
                        ) {
                            Ok(produced_entity) => {
                                if !produced_entity.is_missing() {
                                    println!("Produced!");
                                    *script_env.resources.0 = DeliveryItem::FromSource {
                                        delivered: produced_entity.to_entity(),
                                        source: entity.clone(),
                                    }
                                }
                            }
                            Err(err) => {
                                error!("Error in produce: {:?}", err);
                            }
                        }
                    }
                } else {
                    // if distance_to_anchor < anchor.distance_sq {
                    //     match script_env.call_if_instantiated_1::<EntityId, ()>(
                    //         &source.script,
                    //         "hover",
                    //         EntityId::from_entity(entity),
                    //     ) {
                    //         Ok(()) => {
                    //             // Hovering over this, and it worked!
                    //         }
                    //         Err(err) => {
                    //             error!("Error in hover: {:?}", err);
                    //         }
                    //     }
                    // }
                }
            }
        }
        DeliveryItem::FromSource { .. } => {}
    }
}

fn delivery_dropoff_system(
    mut mouse_location: Local<Vec2>,
    mut script_env: WasmScriptComponentEnv<DeliverySource, ()>,
    delivery_source: Query<&DeliverySource>,
    delivery_anchors: Query<(Entity, &GlobalTransform, &DeliveryDropoff, &DeliveryAnchor)>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut cursor_moved: EventReader<CursorMoved>,
) {
    let (camera, camera_transform) = camera.single();
    for event in cursor_moved.iter() {
        *mouse_location = event.position;
    }
    let mouse_world_location = camera
        .viewport_to_world(camera_transform, *mouse_location)
        .unwrap()
        .origin;

    match *script_env.resources.0 {
        DeliveryItem::Nothing => {}
        DeliveryItem::FromSource { delivered, source } => {
            if mouse_buttons.just_released(MouseButton::Left) {
                let closest_anchor = delivery_anchors
                    .iter()
                    .filter(|(entity, _delivery_transform, dropoff, _anchor)| {
                        match script_env.call_if_instantiated_2::<EntityId, EntityId, i8>(
                            &dropoff.script,
                            "can_receive",
                            EntityId::from_entity(*entity),
                            EntityId::from_entity(delivered),
                        ) {
                            Ok(can_produce) => {
                                if can_produce == 1 {
                                    true
                                } else {
                                    false
                                }
                            }
                            Err(err) => {
                                error!("Error in can_receive: {:?}", err);
                                false
                            }
                        }
                    })
                    .min_by_key(|(_entity, delivery_transform, _dropoff, anchor)| {
                        get_anchor_distance_sq(mouse_world_location, delivery_transform, anchor)
                    });
                if let Some((entity, delivery_transform, dropoff, anchor)) = closest_anchor {
                    let distance_to_anchor =
                        get_anchor_distance_sq(mouse_world_location, delivery_transform, anchor);
                    if distance_to_anchor < anchor.distance_sq {
                        match script_env.call_if_instantiated_3::<EntityId, EntityId, EntityId, ()>(
                            &dropoff.script,
                            "receive",
                            EntityId::from_entity(entity),
                            EntityId::from_entity(delivered),
                            EntityId::from_entity(source),
                        ) {
                            Ok(()) => {
                                println!("Received!");
                                *script_env.resources.0 = DeliveryItem::Nothing;
                            }
                            Err(err) => {
                                error!("Error in receive: {:?}", err);
                            }
                        }
                        return;
                    }
                }
                if let Ok(delivery_source) = delivery_source.get(source) {
                    match script_env.call_if_instantiated_2::<EntityId, EntityId, ()>(
                        &delivery_source.script,
                        "rejected",
                        EntityId::from_entity(source),
                        EntityId::from_entity(delivered),
                    ) {
                        Ok(()) => {
                            println!("Rejected!");
                        }
                        Err(err) => {
                            error!("Error in rejected: {:?}", err);
                        }
                    }
                }
                *script_env.resources.0 = DeliveryItem::Nothing;
            }
        }
    }
}
