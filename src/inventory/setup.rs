use super::*;

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
                    top: Val::Percent(50.0),
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
                margin: UiRect::left(Val::Px(-216.0)),
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
            margin: UiRect::left(Val::Px(-234.0)),
            justify_content: JustifyContent::Center,
            display: Display::Grid,
            grid_template_columns: vec![RepeatedGridTrack::px(9, 48.0)],
            grid_template_rows: vec![RepeatedGridTrack::px(3, 48.0)], 
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
        Text::new(""),
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

pub fn insert_item_in_inventory(
    mut inventory: Query<&mut Inventory, With<Player>>,
    item_registry: Res<ItemRegistry>,
) {
    for mut inventory in inventory.iter_mut() {
        for (i, item_stack) in &mut inventory.items.iter_mut().enumerate() {
            if i == 0 {
                let sword = "Sword".to_string();
                if let Some(def) = item_registry.items.get(&sword) {
                     item_stack.quantity = def.max_stack as i32;
                     item_stack.item_stored = Some(sword);
                }
                continue;
            }
            if i == 1 {
                let pickaxe = "PickAxe".to_string();
                if let Some(def) = item_registry.items.get(&pickaxe) {
                     item_stack.quantity = def.max_stack as i32;
                     item_stack.item_stored = Some(pickaxe);
                }
                continue;
            }
            if i == 14 {
                break
            }
            let inferium = "Inferium".to_string();
            if let Some(def) = item_registry.items.get(&inferium) {
                 item_stack.quantity = def.max_stack as i32;
                 item_stack.item_stored = Some(inferium);
            }
            
            
           
        }
    }
}