use bevy::{mesh::CircularSegmentMeshBuilder, transform::commands, ui::FocusPolicy};

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
            Interaction::None,
            ZIndex(3),
            FocusPolicy::Block,
        )).with_children(|parent| {
            parent.spawn((
                SlotIcon {
                    index: i,
                },
            ));
            parent.spawn((
                Text::new(""),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent((50.0)),
                    ..Default::default()
                },
                SlotCounter,
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
            ZIndex(2),
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
        ZIndex(2),
    )).id();
    let slice_for_inv_grid = &slots[9..];
    for slot in slice_for_inv_grid {
        commands.entity(inventory_grid).add_child(*slot);
    }
    commands.spawn((
        Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        CursorCarrier {
            item: None,
            quantity: 0,
        },
        // BackgroundColor(Color::WHITE),
        ZIndex(10000),
    ));
    
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            bottom: Val::Px(0.0),
            ..default()
        },
        UiBackground,
        ZIndex(0),
        // BackgroundColor(Color::WHITE),
        Interaction::None,
        FocusPolicy::Block,
    ));
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
            if slot.index == active_slot.index as usize {
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
            let item_stack = inventory.items.get(active_slot.index as usize).unwrap();
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
                    
                    println!("Item dropped");
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

pub fn pick_active_slot(
    mut reader: MessageReader<KeyPressed>,
    mut active_slot: Query<&mut ActiveSlot, With<Player>>,
) {
    if let Ok(mut slot) = active_slot.single_mut() {
        for msg in reader.read() {
            let mut active = None;
            match msg.key {
                KeyCode::Digit1 => active = Some(0),
                KeyCode::Digit2 => active = Some(1),
                KeyCode::Digit3 => active = Some(2),
                KeyCode::Digit4 => active = Some(3),
                KeyCode::Digit5 => active = Some(4),
                KeyCode::Digit6 => active = Some(5),
                KeyCode::Digit7 => active = Some(6),
                KeyCode::Digit8 => active = Some(7),
                KeyCode::Digit9 => active = Some(8),
                _ => {}
            }
            if let Some(active) = active {
                slot.index = active;
                println!("Active slot changed to {}", active);
            }
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

pub fn inventory_interactions(
    keys: Res<ButtonInput<KeyCode>>,
    mut reader_click: MessageReader<MouseClickEvent>,
    mut slots: Query<(Entity, &mut BorderColor, &Children, &Interaction), Changed<Interaction>>,
    mut slot: Query<&SlotIcon>,
    mut writer: MessageWriter<SlotClicked>,
    mut writer_outside: MessageWriter<DropFromCursor>,
    mut ui_click_track: ResMut<UiClickTrack>,
    time: Res<Time>,
) {
    for (entity, mut border, children, interaction) in slots.iter_mut() {
        if *interaction == Interaction::Pressed {
            let now = time.elapsed_secs_f64();
            println!("pressed");
            for child in children.iter() {
                if let Ok(slot) = slot.get_mut(child) {
                    if now - ui_click_track.last >= 0.2 {
                        if keys.pressed(KeyCode::ShiftLeft) {
                            writer.write(SlotClicked {
                                click_type: ClickType::ShiftLeftSingle,
                                entity: entity,
                                slot_index: slot.index,
                            });
                            println!("shift clicked slot: {}", slot.index);
                            ui_click_track.last = now;
                            break;
                        } else if keys.pressed(KeyCode::ControlLeft) {
                            writer.write(SlotClicked {
                                click_type: ClickType::CtrlLeftSingle,
                                entity: entity,
                                slot_index: slot.index,
                            });
                            println!("ctrl clicked slot: {}", slot.index);
                            ui_click_track.last = now;
                            break;
                        } else {
                            writer.write(SlotClicked {
                                click_type: ClickType::LeftSingle,
                                entity: entity,
                                slot_index: slot.index,
                            });
                            println!("clicked slot: {}", slot.index);
                            ui_click_track.last = now;
                            break;
                        }
                        
                    } else  {
                        writer.write(SlotClicked {
                            click_type: ClickType::LeftDouble,
                            entity: entity,
                            slot_index: slot.index,
                        });
                        println!("double clicked slot: {}", slot.index);
                        ui_click_track.last = now;
                        break;
                    }    
                }
            }
        }
    }
} 


pub fn background_interactions(
    mut query: Query<&Interaction, (Changed<Interaction>, With<UiBackground>)>,
    mut writer: MessageWriter<DropFromCursor>,
) {
    for interaction in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            writer.write(DropFromCursor);
            println!("clicked outside");
        }
    }
}

pub fn item_click_handler(
    mut reader: MessageReader<SlotClicked>,
    mut inventory: Query<&mut Inventory, With<Player>>,
    mut cursor: Query<(Entity, &mut CursorCarrier)>,
    mut writer: MessageWriter<InsertToInventory>,
    mut writer_get: MessageWriter<GetFromInventory>,
    mut reader_drop: MessageReader<DropFromCursor>,
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
                    ClickType::LeftDouble => {
                        quantity = ItemQuantity::Max;
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
                            let quantity = msg.quantity.match_quantity(def.max_stack as i32, item_stack.quantity);
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
                            println!("ok5");
                            if let Some(mut item_stack) = inventory.items.get_mut(slot) {
                                println!("ok6");
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
                                    println!("ok3");
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

pub fn ui_cursor_update(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &mut Node, &mut CursorCarrier)>,
    image_node: Query<&mut ImageNode>,
    registry: Res<ItemRegistry>
) {
    if let Some(position) = window.cursor_position() {
        for ((entity, mut node, cursor)) in query.iter_mut() {
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

//testing 

pub fn insert_item_in_inventory(
    mut commands: Commands,
    mut inventory: Query<&mut Inventory, With<Player>>,
    item_registry: Res<ItemRegistry>,
) {
    for mut inventory in inventory.iter_mut() {
        for (i, item_stack) in &mut inventory.items.iter_mut().enumerate() {
            if i == 0 {
                item_stack.item_stored = Some(ItemId::Sword);
                if let Some(def) = item_registry.items.get(&ItemId::Sword) {
                     item_stack.quantity = def.max_stack as i32;
                }
                continue;
            }
            if i == 1 {
                item_stack.item_stored = Some(ItemId::PickAxe);
                if let Some(def) = item_registry.items.get(&ItemId::PickAxe) {
                     item_stack.quantity = def.max_stack as i32;
                }
                continue;
            }
            item_stack.item_stored = Some(ItemId::Inferium);
            if let Some(def) = item_registry.items.get(&ItemId::Inferium) {
                 item_stack.quantity = def.max_stack as i32;
            }
           
        }
    }
}