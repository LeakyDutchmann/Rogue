use std::process::Output;

use bevy::{render::texture, transform::commands};

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


pub fn handle_slot_interaction(cursor_carrier: &mut CursorCarrier, item_stack: &mut ItemStack, item_reg: &ItemRegistry) {
    if let Some(cr_item) = &cursor_carrier.item {
        if let Some(item) = &item_stack.item_stored {
            if cr_item == item {
                if let Some(def) = item_reg.items.get(item) {
                    let can_put = def.max_stack.wrapping_sub(item_stack.quantity as usize);
                    if can_put >= cursor_carrier.quantity as usize {
                        item_stack.quantity += cursor_carrier.quantity as i32;
                        cursor_carrier.clear();
                    }
                    if can_put < cursor_carrier.quantity as usize {
                        item_stack.quantity += can_put as i32;
                        cursor_carrier.quantity -= can_put as i32;
                    }
                }
            } else {
                let item_s_quan = item_stack.quantity;
                let item_s_stored = item_stack.item_stored.clone();
                item_stack.quantity = cursor_carrier.quantity;
                item_stack.item_stored = cursor_carrier.item.clone();
                cursor_carrier.quantity = item_s_quan;
                cursor_carrier.item = item_s_stored;
            }
        } else {
            item_stack.quantity = cursor_carrier.quantity;
            item_stack.item_stored = cursor_carrier.item.clone();
            cursor_carrier.clear();
        }
    } else {
        if let Some(item) = &item_stack.item_stored {
            cursor_carrier.item = Some(item.clone());
            cursor_carrier.quantity = item_stack.quantity;
            item_stack.quantity = 0;
            item_stack.item_stored = None;
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
    item_reg: ResMut<ItemRegistry>,
    mut writer: MessageWriter<UiSlotUpdate>,
) {
    if interaction_state.interacting == InteractionStage::Interacting {
        let entity = interaction_state.entity.unwrap();
        if let Ok((entity, mut processing)) = oven_entity.get_mut(entity) {
            for msg in reader.read() {
                if let Ok(children) = children.get(msg.entity) {
                    for child in children.iter() {
                        if let Ok(mut cursor_carrier) = cursor_car.single_mut() {
                            if let Ok(input_slot) = input.get(child) {
                                if let Some(item_stack) = processing.input.get_mut(input_slot.index) {
                                    handle_slot_interaction(&mut cursor_carrier, item_stack, &item_reg);
                                    writer.write(UiSlotUpdate {
                                        entity: msg.entity,
                                        to_quantity: item_stack.quantity as usize,
                                        to_item: item_stack.item_stored.clone().unwrap_or_default(),
                                    });
                                }
                            } else if let Ok(output_slot) = output.get(child) {
                                if let Some(item_stack) = processing.output.get_mut(output_slot.index) {
                                    handle_slot_interaction(&mut cursor_carrier, item_stack, &item_reg);
                                    writer.write(UiSlotUpdate {
                                        entity: msg.entity,
                                        to_quantity: item_stack.quantity as usize,
                                        to_item: item_stack.item_stored.clone().unwrap_or_default(),
                                    });
                                }
                            }
                        }  
                    }
                }
            }
        }
    }
}

pub fn interact_with_workbench(
    mut reader: MessageReader<UiClick>,
    mut cursor_car: Query<&mut CursorCarrier>,
    interaction_state: Res<InteractionState>,
    recipe_reg: Res<RecipeRegistry>,
    work_bench_slot: Query<&WorkBenchSlot>,
    item_reg: ResMut<ItemRegistry>,
    inventory: Query<&Inventory>,
    mut writer_remove_from_inv: MessageWriter<RemoveFromInventory>,
    mut console: ResMut<Console>
) {
    if interaction_state.interacting != InteractionStage::Interacting {
        return;
    }
    for msg in reader.read() {
        if let Ok(work_bench_slot) = work_bench_slot.get(msg.entity) {
            console.log(format!("ok"));
            if work_bench_slot.item.is_none() {
                console.log(format!("empty slot"));
                return;
            }
            let item = work_bench_slot.item.as_ref().unwrap();
            let inventory = inventory.single().expect("no inventory found");
            let mut cursor = cursor_car.single_mut().expect("no cursor carrier found");
            if let Some(recipe) = recipe_reg.recipes.get(item) {
                let mut ingredients = Vec::new();
                let mut missing_ingredients = Vec::new();
                for (item, quantity) in &recipe.ingredients {
                    if check_if_inventory_has_item(inventory, item, quantity.clone()) {
                        ingredients.push((item.clone(), quantity.clone()));
                    } else {
                        missing_ingredients.push((item.clone(), quantity.clone()));
                        println!("missing ingredient: {} ({})", item, quantity);
                    }
                }
    
                if missing_ingredients.is_empty() {
                    for (item, quantity) in ingredients {
                        writer_remove_from_inv.write(RemoveFromInventory {
                            quantity: quantity.clone(),
                            item: item.clone(),
                        });
                    }
                    cursor.item = Some(item.clone());
                    cursor.quantity = 1;
                } else {
                    console.log(format!("missing ingredients: {:?}", missing_ingredients));
                }
            }
        }
    }
}

pub fn check_inventory(
    inventory: Query<&Inventory>,
) {
    let inventory = inventory.single().expect("no inventory found");
    for item in &inventory.items {
        println!("item: {} ({})", item.item_stored.as_deref().unwrap_or(""), item.quantity);
    }
}

pub fn interact_with_chest_slots(
    mut reader: MessageReader<UiClick>,
    mut cursor_car: Query<&mut CursorCarrier>,
    interaction_state: Res<InteractionState>,
    recipe_reg: Res<RecipeRegistry>,
    mut chest_slot: Query<&mut ChestSlot>,
    item_reg: Res<ItemRegistry>,
    mut chest_id: Query<&mut Chest>,
    inventory: Query<&Inventory>,
    mut writer_remove_from_inv: MessageWriter<RemoveFromInventory>,
    mut console: ResMut<Console>
) {
    for message in reader.read() {
        if interaction_state.interacting != InteractionStage::Interacting {
            continue;
        }
        let entity_interacting = interaction_state.entity.unwrap();
        if let Ok(mut chest) = chest_id.get_mut(entity_interacting) {
            let mut cursor_carrier = cursor_car.single_mut().expect("no cursor carrier found");
            if let Ok(mut slot) = chest_slot.get_mut(message.entity) {
                if let Some(item_stack) = chest.items.get_mut(&slot.index) {
                    handle_slot_interaction(&mut cursor_carrier, item_stack, &item_reg);
                }
                
            }
        }
    }
}