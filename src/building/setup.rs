use super::*;

pub fn setup_building_mode_ui(
    mut commands: Commands,
) {
    let mut slots = Vec::new();
    for i in 0..40 {
        slots.push(commands.spawn((
            Node {
                border: UiRect::all(Val::Px(2.0)),
                width: Val::Px(48.0),
                height: Val::Px(48.0),
                ..default()
            },
            BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
            Interaction::None,
            ZIndex(3),
            // FocusPolicy::Block,
        )).id());
    }
    let root =commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px((256.0)),
            height: Val::Percent((100.0)),
            left: Val::Px((32.0)),
            // top: Val::Percent((50.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        // BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        // BorderColor::all(Color::srgb(1.0, 1.0, 1.0)),
    )).id();
    let actual = commands.spawn((
        Node {
            border: UiRect::all(Val::Px(2.0)),
            position_type: PositionType::Absolute,
            width: Val::Px((296.0)),
            height: Val::Percent((80.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            display: Display::Grid,
            grid_template_columns: vec![RepeatedGridTrack::px(5, 48.0)],
            // grid_template_rows: vec![RepeatedGridTrack::px(3, 48.0)], 
            column_gap: Val::Px(4.0),
            row_gap: Val::Px(4.0),
            padding: UiRect::all(Val::Px(4.0)),
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        BorderColor::all(Color::srgb(1.0, 1.0, 1.0)),
        BuildingUiNode,
    )).id();
    for slot in slots {
        commands.entity(actual).add_child(slot);
    }
    commands.entity(root).add_child(actual);
}

pub fn load_structures(
    mut struct_registry: ResMut<StructureRegistry>,
    assest_server: Res<AssetServer>,
) { 
    let path = "./data/structures";
    if let Ok(structures) = load_definitions_for::<StructureDefinitionRaw>(path) {
        let mut count = 0;
        for structure in structures {
            let sprite = assest_server.load(&structure.sprite_path);
            let icon = assest_server.load(&structure.icon_path);
            let definition = StructureDefinition {
                sprite,
                icon,
            };
            struct_registry.structures.insert(structure.name.clone(), definition);
            count += 1;
        }
        println!("Loaded {} structures", count);
    }
}