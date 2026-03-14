use crate::items::assemble_item;

use super::*;

pub fn setup_inventory(
    mut commands: Commands,
) {
    let mut slots = Vec::new();
    for i in 0..9 {
        slots.push(commands.spawn((
            Slot {
                index: i,
            },
            Node {
                border: UiRect::all(Val::Px(2.0)),
                width: Val::Px(48.0),
                height: Val::Px(48.0),
                ..default()
            },
            BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
        )).with_children(|parent| {
            parent.spawn((
                SlotIcon {
                    index: i,
                },
            ));
        }).id());
    }
    let root = commands.spawn((
            Node {
                border: UiRect::all(Val::Px(2.0)),
                width: Val::Px(500.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Percent(50.0),
                margin: UiRect::left(Val::Px(-250.0)), // центрування
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            BorderColor::all(Color::srgb(1.0, 1.0, 1.0)),
        )).id();
    for slot in slots {
        commands.entity(root).add_child(slot);
    }
}

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
                    if let Some(def) = item_registry.items.get(&itemid) {
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

pub fn show_active_slot(
    active_slot: Query<&ActiveSlot>,
    mut ui_slots: Query<(&mut BorderColor, &Slot)>
) {
    if let Ok(active_slot) = active_slot.single()  {
        for (mut bd_color, slot) in ui_slots.iter_mut() {
            if slot.index == active_slot.index {
                bd_color.set_all(Color::srgb(1.0, 1.0, 1.0));
            } else {
                bd_color.set_all(Color::srgb(0.5, 0.5, 0.5));
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
            let item_stack = inventory.items.get(active_slot.index).unwrap();
            if let Some(item_id) = &item_stack.item_stored {
                if held_item.held != Some(*item_id) {
                    held_item.last_held = held_item.held;
                    held_item.held = Some(*item_id);
                }
            } else {
                if held_item.held.is_some() {
                    held_item.last_held = held_item.held;
                    held_item.held = None;
                }
            }
        }
    }
}

pub fn drop_item(
    mut player: Query<(&Transform, &mut Inventory, &ActiveSlot), With<Player>>,
    mut reader: MessageReader<KeyPressed>,
    mut writer: MessageWriter<SpawnItemRequest>
) {
    for msg in reader.read() {
        if msg.key == KeyCode::KeyG {
            if let Ok((tf, mut inventory, active_slot)) = player.single_mut() {
                if let Some(item) = inventory.items.get_mut(active_slot.index) {
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
                    
                    println!("Item dropped");
                }
            }
        }
    }
}







