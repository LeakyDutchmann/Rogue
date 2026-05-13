use super::*;

pub fn check_if_inventory_has_item(inventory: &Inventory, item: &String, quantity_request: i32) -> bool {
    let mut quantity_inside = 0;
    for slot in &inventory.items {
        if quantity_inside >= quantity_request {
            break;
        }
        if slot.item_stored.as_ref() == Some(item) {
            quantity_inside += slot.quantity;
        }
    }
    if quantity_inside >= quantity_request {
        true
    } else {
        false
    }
}

pub fn handle_lmb_slot_interaction(cursor_carrier: &mut CursorCarrier, item_stack: &mut ItemStack, item_reg: &ItemRegistry) {
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

pub fn handle_rmb_slot_interaction(
    cursor_carrier: &mut CursorCarrier,
    item_stack: &mut ItemStack,
    item_reg: &ItemRegistry) {
    if let Some(cursor_item) = &cursor_carrier.item {
        if let Some(item) = &item_stack.item_stored {
            if cursor_item == item {
                if let Some(def) = item_reg.items.get(item) {
                    if item_stack.quantity < def.max_stack as i32 {
                        item_stack.quantity += 1;
                        cursor_carrier.quantity -= 1;
                        if cursor_carrier.quantity == 0 {
                            cursor_carrier.clear();
                        }
                    }
                }
            } else if cursor_carrier.quantity == 1 {
                let c_item = cursor_item.clone();
                cursor_carrier.set(Some(item.clone()), item_stack.quantity);
                item_stack.set(Some(c_item), 1);
            }
        } else {
            item_stack.set(Some(cursor_item.clone()), 1);
            cursor_carrier.quantity -= 1; 
            if cursor_carrier.quantity == 0 {
                cursor_carrier.clear();
            }
        }
    }
}

pub fn handle_slot_interaction(
    cursor_carrier: &mut CursorCarrier, item_stack: &mut ItemStack, item_reg: &ItemRegistry, msg: &UiClick
) {
    if msg.double {
        return;
    }
    match msg.kind {
        ClickKind::LMB => handle_lmb_slot_interaction(cursor_carrier, item_stack, item_reg),
        ClickKind::RMB => handle_rmb_slot_interaction(cursor_carrier, item_stack, item_reg),
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