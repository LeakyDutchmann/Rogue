use crate::items::assemble_item;

use super::*;


#[derive(Component)]
pub struct HotBar;


#[derive(Component)]
pub struct InventoryGrid;


pub fn setup_inventory(
    mut commands: Commands,
) {
    let mut slots = Vec::new();
    for i in 0..36 {
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
    let hotbar = commands.spawn((
            Node {
                border: UiRect::all(Val::Px(2.0)),
                width: Val::Px(432.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Percent(50.0),
                margin: UiRect::left(Val::Px(-216.0)), // центрування
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            BorderColor::all(Color::srgb(1.0, 1.0, 1.0)),
            HotBar,
        )).id();
    let slice_for_h_bar = &slots[0..9];
    for slot in slice_for_h_bar {
        commands.entity(hotbar).add_child(*slot);
    }
    let inventory_grid = commands.spawn((
        Node {
            border: UiRect::all(Val::Px(2.0)),
            width: Val::Px(472.0),
            height: Val::Px(164.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(60.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-234.0)), // центрування
            justify_content: JustifyContent::Center,
            display: Display::Grid,
            grid_template_columns: vec![RepeatedGridTrack::px(9, 48.0)], // 9 колонок
            grid_template_rows: vec![RepeatedGridTrack::px(3, 48.0)],    // 3 ряди
            column_gap: Val::Px(4.0),
            row_gap: Val::Px(4.0),
            padding: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        BorderColor::all(Color::srgb(1.0, 1.0, 1.0)),
        InventoryGrid,
    )).id();
    let slice_for_inv_grid = &slots[9..];
    for slot in slice_for_inv_grid {
        commands.entity(inventory_grid).add_child(*slot);
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





