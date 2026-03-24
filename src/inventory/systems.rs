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
                    if let Some(item_id) = item.item_stored {
                        item.quantity -= 1;
                        writer.write(SpawnItemRequest {
                            position: tf.translation.truncate(),
                            item_id,
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
            if let Some(item_id) = carrier.item {
                for _ in 0..carrier.quantity {
                    writer.write(SpawnItemRequest {
                        position: player_tf.0.translation.truncate(),
                        item_id,
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
            match msg.event {
                ScrollDir::ScrollUp => {
                    if slot.index != 8 {
                        slot.index += 1;
                    } else {
                        slot.index = 0;
                    }
                }
                ScrollDir::ScrollDown => {
                    if slot.index != 0 {
                        slot.index -= 1;
                    } else {
                        slot.index = 8;
                    }
                }
                _ => {}
            }

        }   
    }
}

pub fn item_click_handler(
    mut reader: MessageReader<SlotClicked>,
    mut cursor: Query<(Entity, &mut CursorCarrier)>,
    mut writer: MessageWriter<InsertToInventory>,
    mut writer_get: MessageWriter<GetFromInventory>,
) {
    for msg in reader.read() {
        
        if let Ok((entity, mut cursor)) = cursor.single_mut() {
            if let Some(item) = cursor.item {
                writer.write(InsertToInventory {
                    item: item,
                    quantity: cursor.quantity,
                    slot: Some(msg.slot_index),
                });
            } else if cursor.item.is_none() {
                let mut quantity = ItemQuantity::Empty;
                match msg.click_type {
                    ClickType::CtrlLeftSingle => {
                        quantity = ItemQuantity::One;
                    }
                    ClickType::LeftSingle => {
                        quantity = ItemQuantity::MaxFromOne;
                    }
                    ClickType::ShiftLeftSingle => {
                        quantity = ItemQuantity::HalfStack;
                    }
                }
                writer_get.write(GetFromInventory {
                    quantity: quantity,
                    slot: msg.slot_index,
                });
            }
        }
    }
}

pub fn double_click_handler(
    mut commands: Commands,
    mut reader: MessageReader<DoubleClicked>,
    mut inventory: Query<&mut Inventory, With<Player>>,
    mut cursor: Query<(Entity, &mut CursorCarrier)>,
    registry: Res<ItemRegistry>,
    
) {
    for msg in reader.read() {
        if let Ok((entity, mut cursor)) = cursor.single_mut() {
            if let Ok(mut inventory) = inventory.single_mut() {
                if let Some(item_stack) = inventory.items.get_mut(msg.slot_index) {
                    let item_id = match inventory.items.get(msg.slot_index).and_then(|s| s.item_stored) {
                        Some(id) => id,
                        None => if cursor.item.is_none() { return } else { cursor.item.unwrap() },
                    };
                    if let Some(def) = registry.items.get(&item_id) {
                        if cursor.item.is_none() {
                            cursor.item = Some(item_id);
                        }
                        if cursor.item != Some(item_id) {
                            return;
                        }
                        let mut remaining = def.max_stack as i32 - cursor.quantity;
                        if let Some(stack) = inventory.items.get_mut(msg.slot_index) {
                            if stack.item_stored == Some(item_id) {
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
                            if i == msg.slot_index {
                                continue; // пропускаємо вже оброблений слот
                            }
                    
                            if remaining <= 0 {
                                break;
                            }
                            if stack.item_stored == Some(item_id) {
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
}

pub fn item_take_handler(
    mut commands: Commands,
    mut reader: MessageReader<GetFromInventory>,
    mut inventory: Query<&mut Inventory, With<Player>>,
    mut cursor: Query<(Entity, &mut CursorCarrier)>,
    registry: Res<ItemRegistry>,
) {
    for msg in reader.read() {
        if let Ok(mut inventory) = inventory.single_mut() {
            if let Ok((entity, mut cursor)) = cursor.single_mut() {
                if let Some(mut item_stack) = inventory.items.get_mut(msg.slot) {
                    if let Some(item_id) = item_stack.item_stored {
                        cursor.item = Some(item_id);
                        if let Some(def) = registry.items.get(&item_id) {
                            if let Ok(quantity) = msg.quantity.match_quantity(def.max_stack as i32, item_stack.quantity) {
                                cursor.quantity += quantity;
                                item_stack.quantity -= quantity;
                                if item_stack.quantity == 0 {
                                    item_stack.item_stored = None;
                                }
                            } 
                        }       
                    }
                }
            }
        }
    }
}

pub fn item_put_handler(
    mut commands: Commands,
    mut reader: MessageReader<InsertToInventory>,
    mut inventory: Query<&mut Inventory, With<Player>>,
    mut cursor: Query<(Entity, &mut CursorCarrier)>,
    registry: Res<ItemRegistry>,
) {
    for msg in reader.read() {
        let mut quantity_to_put = msg.quantity;
        if let Ok(mut inventory) = inventory.single_mut() {
            if let Ok((entity, mut cursor)) = cursor.single_mut() {
                if let Some(item_id) = cursor.item {
                    if let Some(def) = registry.items.get(&item_id) {
                        if let Some(slot) = msg.slot {
                            if let Some(mut item_stack) = inventory.items.get_mut(slot) {
                                if item_stack.item_stored == Some(item_id) {
                                    if item_stack.quantity < def.max_stack as i32 {
                                        let free = def.max_stack as i32 - item_stack.quantity;
                                        if quantity_to_put <= free {
                                            item_stack.quantity += quantity_to_put;
                                            cursor.clear();
                                            break;
                                        } else {
                                            item_stack.quantity = def.max_stack as i32;
                                            quantity_to_put -= free;
                                            cursor.quantity = quantity_to_put;
                                        }
                                    }
                                } else if item_stack.item_stored.is_none() {
                                    item_stack.item_stored = Some(item_id);
                                    item_stack.quantity = quantity_to_put;
                                    cursor.clear();
                                    break;
                                } else if item_stack.item_stored != Some(item_id) && item_stack.item_stored.is_some() {
                                    cursor.item = item_stack.item_stored;
                                    cursor.quantity = item_stack.quantity;
                                    item_stack.item_stored = Some(item_id);
                                    item_stack.quantity = quantity_to_put;
                                    break;
                                }
                            }
                        } else {
                            let mut pushed = false;
                            for slot in inventory.items.iter_mut() {
                                if let Some(stored_id) = slot.item_stored.clone() {
                                    if stored_id == item_id {
                                        if slot.quantity < def.max_stack as i32 {
                                            let free = def.max_stack as i32 - slot.quantity;
                                            if free >= quantity_to_put {
                                                slot.quantity += quantity_to_put;
                                                quantity_to_put = 0;
                                                pushed = true;
                                                cursor.clear();
                                                break;
                                            } else if free < quantity_to_put {
                                                let remaining = quantity_to_put - free;
                                                slot.quantity = def.max_stack as i32;
                                                quantity_to_put = remaining;
                                                cursor.quantity = quantity_to_put;
                                            }     
                                        }
                                    }
                                }
                            }
                            if !pushed {
                                for slot in inventory.items.iter_mut() {
                                    if slot.item_stored.is_none() {
                                        if quantity_to_put <= def.max_stack as i32 {
                                            slot.item_stored = Some(item_id);
                                            slot.quantity += quantity_to_put;
                                            cursor.clear();
                                            break;
                                        } else if quantity_to_put > def.max_stack as i32 {
                                            slot.item_stored = Some(item_id);
                                            slot.quantity = def.max_stack as i32;
                                            quantity_to_put -= def.max_stack as i32;
                                            cursor.quantity = quantity_to_put;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

