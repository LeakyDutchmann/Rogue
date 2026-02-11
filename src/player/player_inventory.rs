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
        // println!("foundi nventory");
        for (slot_e, slot_icon) in slots.iter_mut() {
            let slot_idx = slot_icon.index;
            // println!("found idx");
            if let Some(Some(item_entity)) = inventory.items.get(slot_idx) {
                let texture = item_query.get(*item_entity).unwrap();
                let image = texture.image.clone();
                commands.entity(slot_e).insert(ImageNode::new(image));
                // println!("inserted img");
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
    mut held_item: Query<(Entity, &mut HeldItem, &mut Transform), (With<HeldItem>, Without<AttackAnimation>)>,
    inventory: Query<(&Inventory, &ActiveSlot), With<Player>>,
    item_query: Query<&Item>,
) {
    if let Ok((player_hand, mut held_item, mut hand_pos)) = held_item.single_mut() {
        if let Ok((inventory, active_slot)) = inventory.single() {
            if let Some(Some(item_entity)) = inventory.items.get(active_slot.index) {
                // Only update if we're holding a different item (or None)
                if held_item.last_held != Some(*item_entity) {
                    let texture = item_query.get(*item_entity).unwrap();
                    let image = texture.image.clone();
                    commands.entity(player_hand).insert(Sprite::from_image(image));
                    held_item.last_held = Some(*item_entity);
                }
            } else {
                // No item in active slot - remove sprite if we were holding something
                if held_item.last_held.is_some() {
                    commands.entity(player_hand).remove::<Sprite>();
                    held_item.last_held = None;
                }
            }
            hand_pos.translation = Vec3::new(0.0, 0.0, 1.0);
        }
    }
}


pub fn update_held_item_dir(
    mut held_item: Query<(&mut Transform, &mut Sprite), (With<HeldItem>, Without<AttackAnimation>)>,
    player: Query<(&Player, &FacingDirection)>,
) {
    let Ok((player, facing_dir)) = player.single() else { return };
    let Ok((mut transform, mut sprite)) = held_item.single_mut() else { return };
    sprite.flip_x = matches!(facing_dir.facing, Facing::Left);
    if facing_dir.facing == Facing::Right {
        transform.rotation = Quat::from_rotation_z((30.0_f32).to_radians());
    } else if facing_dir.facing == Facing::Left {
        transform.rotation = Quat::from_rotation_z(-(30.0_f32).to_radians());
        sprite.flip_x = true;
    }
}





