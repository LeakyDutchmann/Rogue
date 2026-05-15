
use super::*;

pub fn interact_with_structure(
    structure_identifier: Query<(Entity,&StructureId, &Transform), With<Interactable>>,
    keys: Res<ButtonInput<KeyCode>>,
    player_transform: Res<PlayerTransform>,
    worldgrid: Res<WorldGrid>,
    mut interaction_state: ResMut<InteractionState>,
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
                if let Some((_, position, _)) = &nearest_struct {
                    if player_pos.distance(pos) < player_pos.distance(position.clone()) {
                        nearest_struct = Some((structure_id, pos, entity));
                    }
                } else {
                    nearest_struct = Some((structure_id, pos, entity));
                }
            }
            if let Some((structure_id, _, entity)) = &nearest_struct {
                if let Some(definition) = struct_reg.structures.get(structure_id) {
                    interaction_state.interaction_type = definition.interaction.clone();
                    if let Some(ui_window) = &definition.ui_window_id {
                        interaction_state.ui_window_id = Some(ui_window.clone());
                    }
                }
                interaction_state.interacting = InteractionStage::Intializing;
                interaction_state.entity = Some(*entity);
            } else {
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

pub fn interact_with_workbench(
    mut reader: MessageReader<UiClick>,
    mut cursor_car: Query<&mut CursorCarrier>,
    interaction_state: Res<InteractionState>,
    recipe_reg: Res<RecipeRegistry>,
    work_bench_slot: Query<&WorkBenchSlot>,
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

pub fn quick_move_from_container(
    interaction_state: Res<InteractionState>,
    item_reg: Res<ItemRegistry>,
    mut reader: MessageReader<QuickMoveFromContainer>,
    mut inventory: Query<&mut Inventory, With<Player>>,
    mut chest: Query<&mut Chest>,
    mut processing: Query<&mut Processing>,    
) {
    for msg in reader.read() {
        let target = determine_target_container(msg.container, &interaction_state);
        match msg.container {
            ContainerType::Inventory => {
                if let Ok(mut inventory) = inventory.single_mut() {
                    if let Some(target) = target {
                        match target {
                            ContainerType::Input { entity } => {
                                if let Ok(mut processing) = processing.get_mut(entity) {
                                    quick_move_to(&mut inventory.items, &mut processing.input, msg.index, &item_reg);
                                }
                            }
                            ContainerType::Chest{entity} => {
                                if let Ok(mut chest) = chest.get_mut(entity) {
                                    quick_move_to(&mut inventory.items, &mut chest.items, msg.index, &item_reg);
                                }
                            }
                            _ => {}
                        }
                    }
                } 
            }
            ContainerType::Chest{entity} => {
                if let Ok(mut chest) = chest.get_mut(entity) {
                    if let Ok(mut inventory) = inventory.single_mut() {
                        quick_move_to(&mut chest.items, &mut inventory.items , msg.index, &item_reg);
                    }
                } 
            }
            ContainerType::Input{entity} => {
                if let Ok(mut processing) = processing.get_mut(entity) {
                    if let Ok(mut inventory) = inventory.single_mut() {
                        quick_move_to(&mut processing.input, &mut inventory.items , msg.index, &item_reg);
                    }
                } 
            }
            ContainerType::Output{entity} => {
                if let Ok(mut processing) = processing.get_mut(entity) {
                    if let Ok(mut inventory) = inventory.single_mut() {
                        quick_move_to(&mut processing.output, &mut inventory.items , msg.index, &item_reg);
                    }
                } 
            }
        }
    }
}
