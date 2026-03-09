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
    mut slots: Query<(Entity, &mut SlotIcon)>, item_query: Query<&Item>,
) {
    if let Ok(inventory) = inventory.single() {
        for (slot_e, slot_icon) in slots.iter_mut() {
            let slot_idx = slot_icon.index;
            if let Some(Some(item_entity)) = inventory.items.get(slot_idx) {
                let texture = item_query.get(*item_entity).unwrap();
                let image = texture.image.clone();
                commands.entity(slot_e).insert(ImageNode::new(image));
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

pub fn draw_helditem(
    mut commands: Commands,
    mut held_item: Query<(Entity, &mut HeldItem), (With<HeldItem>, Without<AttackAnimation>)>,
    item_query: Query<&Item>,
) {
    for (actor_hand, mut held_item) in held_item.iter_mut() {
        if let Some(held) = held_item.held {
            if let Some(last_held) = held_item.last_held {
                if held != last_held {
                    if let Ok(item) = item_query.get(held) {
                        commands.entity(actor_hand).insert(Sprite::from_image(item.image.clone()));
                        held_item.last_held = Some(held);
                    }
                }
            } else {
                if let Ok(item) = item_query.get(held) {
                    commands.entity(actor_hand).insert(Sprite::from_image(item.image.clone()));
                    held_item.last_held = Some(held);
                }
                
            }
             
        } else {
            commands.entity(actor_hand).remove::<Sprite>();
        }
        
    }
}

pub fn sync_player_held_item(
    mut held_item: Query<(&mut HeldItem, &ChildOf),
        (With<HeldItem>, Without<AttackAnimation>,)>,
    inventory_qr: Query<(&Inventory, &ActiveSlot), With<Player>>,
) {
    for (mut held, childof) in held_item.iter_mut() {
        if let Ok((inventory, active)) = inventory_qr.get(childof.0) {
            if let Some(Some(item_e)) = inventory.items.get(active.index) {
                if held.held != Some(*item_e) {
                    held.held = Some(*item_e);
                }
            } else {
                if held.held.is_some() {
                    held.held = None;
                }
            }
        }
    }
}


pub fn update_held_item_dir(
    mut held_item: Query<(&mut Transform, &ChildOf, &mut Sprite), (With<HeldItem>, Without<AttackAnimation>)>,
    facing_qr: Query<&FacingDirection>,
) {
    for (mut hand_pos, childof, mut sprite) in held_item.iter_mut() {
        if let Ok(facing) = facing_qr.get(childof.0) {
            match facing.facing {
                Facing::Up => {
                    hand_pos.translation = Vec3::new(0.0, 0.0, -1.0);
                    hand_pos.rotation = Quat::from_rotation_z((30.0_f32).to_radians());
                },
                Facing::Down => {
                    hand_pos.translation = Vec3::new(0.0, 0.0, 1.0);
                    hand_pos.rotation = Quat::from_rotation_z((30.0_f32).to_radians());
                },
                Facing::Left => {
                    hand_pos.translation = Vec3::new(0.0, 0.0, 1.0);
                    hand_pos.rotation = Quat::from_rotation_z(-(30.0_f32).to_radians());
                    sprite.flip_x = true;
                },
                Facing::Right => {
                    hand_pos.translation = Vec3::new(0.0, 0.0, 1.0);
                    hand_pos.rotation = Quat::from_rotation_z((30.0_f32).to_radians());
                },
            }
        }
    }
}





