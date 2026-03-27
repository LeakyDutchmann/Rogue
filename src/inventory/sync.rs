use super::*;

pub fn sync_player_inventory(
    mut commands: Commands,
    inventory: Query<&Inventory, (With<Player>, Changed<Inventory>)>,
    mut slots: Query<(Entity, &mut SlotIcon)>,
    item_registry: Res<ItemRegistry>,
) {
    if let Ok(inventory) = inventory.single() {
        for (slot_e, slot_icon) in slots.iter_mut() {
            let slot_idx = slot_icon.index;
            if let Some(item_stack) = inventory.items.get(slot_idx) {
                if let Some(itemid) = &item_stack.item_stored {
                    if let Some(def) = item_registry.items.get(itemid) {
                        let icon = def.icon.clone();
                        commands.entity(slot_e).insert(ImageNode::new(icon));
                    }
                } else {
                    commands.entity(slot_e).remove::<ImageNode>();
                }
            } else {
                commands.entity(slot_e).remove::<ImageNode>();
            } 
        } 
    } 
}

pub fn sync_player_held_item(
    player: Query<(Entity, &mut Inventory, &ActiveSlot), With<Player>>,
    mut hand: Query<(&ChildOf, &mut HeldItem)>,
) {
    if let Ok((player_e, inventory, active_slot)) = player.single() {
        for (child_of, mut held_item) in hand.iter_mut() {
            if child_of.0 != player_e {
                continue;
            }
            let item_stack = inventory.items.get(active_slot.index as usize).unwrap();
            if let Some(item_id) = &item_stack.item_stored {
                if held_item.held != Some(item_id.clone()) {
                    held_item.last_held = held_item.held.clone();
                    held_item.held = Some(item_id.clone());
                }
            } else {
                if held_item.held.is_some() {
                    held_item.last_held = held_item.held.clone();
                    held_item.held = None;
                }
            }
        }
    }
}

