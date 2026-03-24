use super::*;

pub fn show_active_slot(
    active_slot: Query<&ActiveSlot>,
    mut ui_slots: Query<(&mut BorderColor, &Slot)>
) {
    if let Ok(active_slot) = active_slot.single()  {
        for (mut bd_color, slot) in ui_slots.iter_mut() {
            if slot.index == active_slot.index as usize {
                bd_color.set_all(Color::srgb(1.0, 1.0, 1.0));
            } else {
                bd_color.set_all(Color::srgb(0.5, 0.5, 0.5));
            }
        }
    }
}

pub fn show_inventory(
    state: Res<InventoryOpen>,
    mut query: Query<&mut Visibility, With<InventoryGrid>>,
) {
    if !state.is_changed() {
        return;
    }

    for mut vis in query.iter_mut() {
        *vis = if state.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub fn ui_cursor_update(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &mut Node, &mut CursorCarrier, &mut Text)>,
    image_node: Query<&mut ImageNode>,
    registry: Res<ItemRegistry>
) {
    if let Some(position) = window.cursor_position() {
        for ((entity, mut node, cursor, mut text)) in query.iter_mut() {
            if cursor.quantity == 0 || cursor.quantity == 1 {
                text.0 = "".to_string();
            } else {
                text.0 = cursor.quantity.to_string();
            }
            node.left = Val::Px(position.x);
            node.top = Val::Px(position.y);
            if let Some(item_id) = cursor.item {
                if cursor.is_changed() {
                    if let Some(def) = registry.items.get(&item_id) {
                        commands.entity(entity).insert(ImageNode::new(def.icon.clone()));
                    }
                }
            } else if cursor.item.is_none() {
                commands.entity(entity).remove::<ImageNode>();
            }
        }
    }
}

pub fn update_item_count(
    mut slots: Query<(&mut Text, &ChildOf), With<SlotCounter>>,
    children: Query<&Children>,
    slot_idx: Query<&SlotIcon>,
    inventory: Query<&Inventory, With<Player>>,
) {
    let mut slot_items: Vec<(usize, i32)> = Vec::new();
    if let Ok(inv) = inventory.single() {
        for (i, item) in inv.items.iter().enumerate() {
            slot_items.push((i, item.quantity));
        }
    }
    for (mut text, parent) in slots.iter_mut() {
        if let Ok(children) = children.get(parent.0) {
            for child in children {
                if let Ok(idx) = slot_idx.get(*child) {
                    for slot in &slot_items {
                        if slot.0 == idx.index {
                            if slot.1 == 0 || slot.1 == 1 {
                                text.0 = "".to_string();
                            } else {
                                text.0 = slot.1.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
}