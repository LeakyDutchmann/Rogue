use std::process::Output;

use super::*;

pub fn interact_with_structure(
    structure_identifier: Query<(Entity,&StructureId, &Transform), With<Interactable>>,
    keys: Res<ButtonInput<KeyCode>>,
    player_transform: Res<PlayerTransform>,
    worldgrid: Res<WorldGrid>,
    mut interaction_state: ResMut<InteractionState>,
    mut console: ResMut<Console>,
    mut close_writer: MessageWriter<CloseWindowRequest>,
    struct_reg: Res<StructureRegistry>,
) {
    if interaction_state.interacting == InteractionStage::Idle {
        if keys.just_pressed(KeyCode::KeyF) {
            let player_pos = player_transform.0.translation.truncate();
            let cell_x = (player_pos.x / CELL_SIZE).floor() as i32;
            let cell_y = (player_pos.y / CELL_SIZE).floor() as i32;
            let cells = get_cells_3x3((cell_x, cell_y));
            let entities = get_entities_in_cells(cells, &worldgrid);
            let mut near_structs: Vec<(String, Vec2, Entity)> = Vec::new();
            for entity in entities {
                if let Ok((entity, structure_id, tf)) = structure_identifier.get(entity) {
                    let pos = tf.translation.truncate();
                    near_structs.push((structure_id.id.clone(), pos, entity));
                }
            }
            let mut nearest_struct: Option<(String, Vec2, Entity)> = None;
            for (structure_id, pos, entity) in near_structs {
                if let Some((_struct_id, position, nearest_entity)) = &nearest_struct {
                    if player_pos.distance(pos) < player_pos.distance(position.clone()) {
                        nearest_struct = Some((structure_id, pos, entity));
                    }
                } else {
                    nearest_struct = Some((structure_id, pos, entity));
                }
            }
            if let Some((structure_id, position, entity)) = &nearest_struct {
                if let Some(definition) = struct_reg.structures.get(structure_id) {
                    interaction_state.interaction_type = definition.interaction.clone();
                    if let Some(ui_window) = &definition.ui_window_id {
                        interaction_state.ui_window_id = Some(ui_window.clone());
                    }
                }
                interaction_state.interacting = InteractionStage::Intializing;
                interaction_state.entity = Some(*entity);
                console.log(format!("Interacting with structure: {:?}", structure_id));        
            } else {
                console.log(format!("No interactable structure found near the player."));
            }
        } 
    } else {
        if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::KeyF) {
            interaction_state.interacting = InteractionStage::Idle;
            interaction_state.entity = None;
            interaction_state.interaction_type = InteractionType::None;
            interaction_state.ui_window_id = None;
            close_writer.write(CloseWindowRequest);
        }
    }
    
    
}

pub fn sync_oven_window(
    mut commands: Commands,
    interaction_state: Res<InteractionState>,
    mut window: Query<Entity, With<UiStructureWindow>>,
    mut input: Query<(Entity, &mut OvenInputSlot)>,
    mut output: Query<(Entity, &mut OvenOutputSlot)>,
    processing: Query<&Processing>,
    item_reg: Res<ItemRegistry>,
    mut console: ResMut<Console>,
) {
    if interaction_state.interacting == InteractionStage::Interacting {
        if let Some(entity) = interaction_state.entity {
            if let Ok(processing) = processing.get(entity) {
                for item_stack in &processing.input {
                    if let Some(item) = &item_stack.item_stored {
                        if let Some(item_def) = item_reg.items.get(item) {
                            for (input_e, mut slot) in input.iter_mut() {
                                if slot.item.as_ref() != Some(item) {
                                    commands.entity(input_e).remove::<ImageNode>();
                                    commands.entity(input_e).insert(ImageNode::new(item_def.icon.clone()));
                                    console.log(format!("Inserted icon for input item: {}", item));
                                    slot.item = Some(item.clone());
                                } else {
                                }
                                
                            }
                        }
                    } 
                }
                for item_stack in &processing.output {
                    if let Some(item) = &item_stack.item_stored {
                        if let Some(item_def) = item_reg.items.get(item) {
                            for (entity, slot) in output.iter() {
                                commands.entity(entity).insert(ImageNode::new(item_def.icon.clone()));
                            }
                        }
                    } 
                }
                
            }
            
            
        }
    }
}

pub fn interact_with_oven_window(
    mut reader: MessageReader<UiClick>,
    mut cursor_car: Query<&mut CursorCarrier>,
    interaction_state: Res<InteractionState>,
    mut oven_entity: Query<(Entity, &mut Processing)>,
    children: Query<&Children>,
    input: Query<&OvenInputSlot>,
    output: Query<&OvenOutputSlot>,
    mut console: ResMut<Console>,
) {
    if interaction_state.interacting == InteractionStage::Interacting {
        if let Ok((entity, mut processing)) = oven_entity.get_mut(interaction_state.entity.unwrap()) {
            for msg in reader.read() {
                if let Ok(mut cursor_carrier) = cursor_car.single_mut() {
                    if let Ok(children) = children.get(msg.entity) {
                        for child in children.iter() {
                            if let Ok(input) = input.get(child) {
                                if processing.input.is_empty() {
                                    if let Some(item) = &cursor_carrier.item {
                                        let item_stack = ItemStack {
                                            item_stored: Some(item.clone()),
                                            quantity: cursor_carrier.quantity,
                                        };
                                        processing.input.push(item_stack);                                
                                        cursor_carrier.clear();
                                        console.log(format!("Inserted new item to oven input"));
                                        break;
                                    }
                                } else {
                                    if let Some(item) = &cursor_carrier.item {
                                        let to_take = processing.input.remove(0);
                                        let item_stack = ItemStack {
                                            item_stored: Some(item.clone()),
                                            quantity: cursor_carrier.quantity,
                                        };
                                        processing.output.push(item_stack);
                                        cursor_carrier.clear();
                                        cursor_carrier.item = to_take.item_stored;
                                        cursor_carrier.quantity = to_take.quantity;
                                        console.log(format!("swapped from oven input"));
                                        break;
                                    } else {
                                        let to_take = processing.input.remove(0);
                                        cursor_carrier.item = to_take.item_stored;
                                        cursor_carrier.quantity = to_take.quantity;
                                        console.log(format!("took from input"));
                                        break;
                                    }
                                }
                            } else if let Ok(output) = output.get(msg.entity) {
                            }
                        }
                    }
                } 
            }
        }
    }
}
