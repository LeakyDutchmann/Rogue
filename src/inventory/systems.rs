use super::*;

pub fn drop_item(
    mut player: Query<(&Transform, &mut Inventory, &ActiveSlot), With<Player>>,
    mut reader: MessageReader<KeyPressed>,
    mut writer: MessageWriter<SpawnItemRequest>,
) {
    for msg in reader.read() {
        if msg.key == KeyCode::KeyG {
            if let Ok((tf, mut inventory, active_slot)) = player.single_mut() {
                if let Some(item) = inventory.items.get_mut(active_slot.index as usize) {
                    if let Some(item_id) = &item.item_stored {
                        item.quantity -= 1;
                        writer.write(SpawnItemRequest {
                            position: tf.translation.truncate(),
                            item_id: item_id.clone(),
                        });
                        if item.quantity == 0 {
                            item.item_stored = None;
                        }
                    }
                }
            }
        }
    }
}

pub fn drop_cursor_item(
    mut cursor: Query<&mut CursorCarrier>,
    mut reader: MessageReader<DropFromCursor>,
    mut writer: MessageWriter<SpawnItemRequest>,
    player_tf: Res<PlayerTransform>,
) {
    for msg in reader.read() {
        if let Ok(mut carrier) = cursor.single_mut() {
            if let Some(item_id) = &carrier.item {
                for _ in 0..carrier.quantity {
                    writer.write(SpawnItemRequest {
                        position: player_tf.0.translation.truncate() + msg.direction * 25.0,
                        item_id: item_id.clone(),
                    });
                }
                carrier.clear();
                println!("Item dropped");
            }

        }
    }
}

pub fn toggle_inventory(
    mut reader: MessageReader<KeyPressed>,
    mut state: ResMut<InventoryOpen>,
) { 
    for msg in reader.read() {
        if msg.key == KeyCode::Tab {
            state.0 = !state.0;
        }
    }
}

pub fn pick_active_slot_scroll(
    mut reader: MessageReader<ScrollMessage>,
    mut active_slot: Query<&mut ActiveSlot, With<Player>>,   
) {
    if let Ok(mut slot) = active_slot.single_mut() {
        for msg in reader.read() {
            if msg.delta.y < 0.0 {
                if slot.index != 0 {
                    slot.index -= 1; 
                } else {
                    slot.index = 8;
                }       
            } else {
                if slot.index != 8 {
                    slot.index += 1;
                } else {
                    slot.index = 0;
                }
            }

        }   
    }
}

pub fn item_click_handler(
    mut reader: MessageReader<UiClick>,
    cursor: Query<&mut CursorCarrier>,
    mut writer: MessageWriter<InsertToInventory>,
    mut writer_get: MessageWriter<GetFromInventory>,
    slot: Query<&SlotIcon>,
    children: Query<&Children>,
) {
    for msg in reader.read() {
        if let Ok(children) = children.get(msg.entity) {
            for child in children.iter() {
                if let Ok(slot) = slot.get(child) {
                    if let Ok(cursor) = cursor.single() {
                        if !msg.double {
                            if let Some(_item) = &cursor.item {
                                writer.write(InsertToInventory {
                                    quantity: cursor.quantity,
                                    slot: Some(slot.index),
                                });
                            } else if cursor.item.is_none() {
                                let mut quantity: Option<ItemQuantity> = None;
                                if msg.ctrl_pressed {
                                    quantity = Some(ItemQuantity::One);
                                } else if msg.shift_pressed {
                                    quantity = Some(ItemQuantity::HalfStack);
                                } else {
                                    quantity = Some(ItemQuantity::MaxFromOne);
                                }
                                if let Some(quantity) = quantity {
                                    writer_get.write(GetFromInventory {
                                        quantity: quantity,
                                        slot: slot.index,
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

pub fn double_click_handler(
    mut reader: MessageReader<UiClick>,
    slot: Query<&SlotIcon>,
    children: Query<&Children>,
    mut inventory: Query<&mut Inventory, With<Player>>,
    mut cursor: Query<&mut CursorCarrier>,
    registry: Res<ItemRegistry>,
) {
    for msg in reader.read() {
        if !msg.double {
            continue;
        }
        let mut opt_slot_index: Option<usize> = None;
        if let Ok(children) = children.get(msg.entity) {
            for child in children.iter() {
                if let Ok(slot) = slot.get(child) {
                    opt_slot_index = Some(slot.index);
                }
            }
        }
        if opt_slot_index.is_none() {
            continue;
        }
        let slot_index = opt_slot_index.unwrap();
        if let Ok(mut cursor) = cursor.single_mut() {
            if let Ok(mut inventory) = inventory.single_mut() {
                let item_id = match inventory.items.get(slot_index).and_then(|s| s.item_stored.as_ref()) {
                    Some(id) => if cursor.item.is_none() {id.clone()} else {cursor.item.clone().unwrap()},
                    None => if cursor.item.is_none() { continue } else { cursor.item.clone().unwrap() },
                };
                if let Some(def) = registry.items.get(&item_id) {
                    let mut remaining = def.max_stack as i32 - cursor.quantity;
                    if let Some(stack) = inventory.items.get_mut(slot_index) {
                        if stack.item_stored.as_ref() == Some(&item_id) {
                            let take = remaining.min(stack.quantity);
                            cursor.quantity += take;
                            stack.quantity -= take;
                            remaining -= take;
                            if stack.quantity == 0 {
                                stack.item_stored = None;
                            }
                        }
                    }
                    for (i, stack) in inventory.items.iter_mut().enumerate() {
                        if i == slot_index {
                            continue; 
                        }
                        if remaining <= 0 {
                            break;
                        }
                        if stack.item_stored.as_ref() == Some(&item_id) {
                            let take = remaining.min(stack.quantity);
                            cursor.quantity += take;
                            stack.quantity -= take;
                            remaining -= take;
                            if stack.quantity == 0 {
                                stack.item_stored = None;
                            }
                        }
                    }
                }
            }
        
        }
    }
}

pub fn remove_from_inventory(
    mut reader: MessageReader<RemoveFromInventory>, 
    mut inventory: Query<&mut Inventory, With<Player>>,
) {
    for msg in reader.read() {
        if let Ok(mut inv) = inventory.single_mut() {
            let mut quantity_to_remove = msg.quantity;
            for slot in inv.items.iter_mut() {
                if let Some(item_id) = &slot.item_stored {
                    if item_id == &msg.item {
                        if quantity_to_remove >= 0 {
                            if slot.quantity > quantity_to_remove {
                                slot.quantity -= quantity_to_remove;
                                quantity_to_remove = 0;
                            } else if slot.quantity == quantity_to_remove {
                                slot.quantity = 0;
                                quantity_to_remove = 0;
                                slot.item_stored = None;
                            } else if slot.quantity < quantity_to_remove {
                                quantity_to_remove -= slot.quantity;
                                slot.quantity = 0;
                                slot.item_stored = None;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub fn ui_slot_click_handler(
    mut commands: Commands,
    mut reader: MessageReader<UiClick>,
    mut inventory: Query<&mut Inventory, With<Player>>,
    mut slot: Query<&mut UiSlot>,
    mut cursor_carrier: Query<&mut CursorCarrier>,
    item_reg: Res<ItemRegistry>,
    interaction_state: Res<InteractionState>,
    mut chest: Query<&mut Chest>,
    mut processing: Query<&mut Processing>,
    mut console: ResMut<Console>,
    mut writer: MessageWriter<QuickMoveFromContainer>,
) {
    for msg in reader.read() {
        if let Ok(mut uislot) = slot.get_mut(msg.entity) {
            let mut cursor_c = cursor_carrier.single_mut().unwrap();
            match uislot.kind {
                UiSlotKind::Inventory => {
                    if let Ok(mut player_inventory) = inventory.single_mut() {
                        if let Some(item_stack) = player_inventory.items.get_mut(uislot.index) {
                            if msg.shift_pressed {
                                writer.write({
                                    QuickMoveFromContainer {
                                        container: ContainerType::Inventory,
                                        index: uislot.index,
                                    }
                                });
                            } else {
                                 handle_slot_interaction(&mut cursor_c, item_stack, &item_reg, msg);
                            }
                            console.log(format!("Handling Inventory"));
                        }
                    }
                }
                UiSlotKind::Chest => {
                    if let Ok(mut chest) = chest.get_mut(interaction_state.entity.unwrap()) {
                        if let Some(item_stack) = chest.items.get_mut(uislot.index) {
                            if msg.shift_pressed {
                                writer.write({
                                    QuickMoveFromContainer {
                                        container: ContainerType::Chest { entity: interaction_state.entity.unwrap() },
                                        index: uislot.index,
                                    }
                                });
                            } else {
                                 handle_slot_interaction(&mut cursor_c, item_stack, &item_reg, msg);
                            }
                            console.log(format!("Handling Chest"));
                        }
                    }
                }
                UiSlotKind::Output => {
                    if let Ok(mut processing) = processing.get_mut(interaction_state.entity.unwrap()) {
                        if let Some(item_stack) = processing.output.get_mut(uislot.index) {
                            if msg.shift_pressed {
                                writer.write({
                                    QuickMoveFromContainer {
                                        container: ContainerType::Output { entity: interaction_state.entity.unwrap() },
                                        index: uislot.index,
                                    }
                                });
                            } else {
                                 handle_slot_interaction(&mut cursor_c, item_stack, &item_reg, msg);
                            }
                            console.log(format!("Handling Output"));
                        }
                    }
                }
                UiSlotKind::Input => {
                    if let Ok(mut processing) = processing.get_mut(interaction_state.entity.unwrap()) {
                        if let Some(item_stack) = processing.input.get_mut(uislot.index) {
                            if msg.shift_pressed {
                                writer.write({
                                    QuickMoveFromContainer {
                                        container: ContainerType::Input { entity: interaction_state.entity.unwrap() },
                                        index: uislot.index,
                                    }
                                });
                            } else {
                                 handle_slot_interaction(&mut cursor_c, item_stack, &item_reg, msg);
                            }
                            console.log(format!("Handling Input"));
                        }
                    }
                }
            }
        }
    }
}

pub fn quick_move_to(from: &mut Vec<ItemStack>, to: &mut Vec<ItemStack>, index_from: usize, item_reg: &ItemRegistry) {
    if let Some(item_stack) = from.get_mut(index_from) {
        if let Some(item) = &item_stack.item_stored {
            if let Some(def) = item_reg.items.get(item) {
                for other_stack in to.iter_mut() {
                    if other_stack.item_stored.as_ref() == Some(item) {
                        if other_stack.quantity < def.max_stack as i32 {
                            let free_space = def.max_stack as i32 - other_stack.quantity;
                            if item_stack.quantity <= free_space {
                                other_stack.quantity += item_stack.quantity;
                                item_stack.quantity = 0;
                                break;
                            } else {
                                other_stack.quantity = def.max_stack as i32;
                                item_stack.quantity -= free_space;
                                if item_stack.quantity == 0 {
                                    break;
                                }
                            }
                        }
                    } 
                }
                if item_stack.quantity == 0 {
                    item_stack.clear();
                } else {
                    for other_stack in to.iter_mut() {
                        if other_stack.item_stored.as_ref() == None {
                            other_stack.set(item_stack.item_stored.clone(), item_stack.quantity);
                            item_stack.clear();
                            break;
                        }
                    }
                }
            }
            
        }
    }
}

pub fn determine_target_container(from: ContainerType, interaction_state: &InteractionState) -> Option<ContainerType> {
    match from {
        ContainerType::Inventory => {
            match interaction_state.interaction_type {
                InteractionType::BasicOven => {
                    Some(ContainerType::Input { entity: interaction_state.entity.unwrap() })
                }
                InteractionType::Chest => {
                    Some(ContainerType::Chest { entity: interaction_state.entity.unwrap() })
                }
                _ => {
                    None
                }
            }
        }
        ContainerType::Chest { .. } => {
            Some(ContainerType::Inventory)
        }
        ContainerType::Input { .. } => {
            Some(ContainerType::Inventory)
        }
        ContainerType::Output { .. } => {
            Some(ContainerType::Inventory)
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