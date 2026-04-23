use super::*;

pub fn setup_building_mode_ui(
    mut commands: Commands,
) {
    let mut slots = Vec::new();
    for _i in 0..40 {
        slots.push(commands.spawn((
            Node {
                border: UiRect::all(Val::Px(2.0)),
                width: Val::Px(48.0),
                height: Val::Px(48.0),
                ..default()
            },
            BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
            Interaction::None,
            ZIndex(2),
            BuildingUiNode,
            FocusPolicy::Block,
        )).with_children(|parent| {
            parent.spawn((
                BuildingUiSlot {
                    structure: None,
                },
            ));
        }).id());
    }
    let root =commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(256.0),
            height: Val::Percent(100.0),
            left: Val::Px(32.0),
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
            width: Val::Px(296.0),
            height: Val::Percent(80.0),
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
        BuildingRootUiNode,
        ZIndex(1),
    )).id();
    for slot in slots {
        commands.entity(actual).add_child(slot);
    }
    commands.entity(root).add_child(actual);
    
    //below is cursor, maybe shall make separated system to initialize it
    commands.spawn((
        Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        CursorStructureCarrier {
            structure: None,
        },
        Text::new(""),
        // BackgroundColor(Color::WHITE),
        ZIndex(10000),
    ));
}

pub fn load_structures(
    mut struct_registry: ResMut<StructureRegistry>,
    mut recipe_registry: ResMut<RecipeRegistry>,
    assest_server: Res<AssetServer>,
) { 
    let path = "./data/structures";
    if let Ok(structures) = load_definitions_for::<StructureDefinitionRaw>(path) {
        let mut count = 0;
        for structure in structures {
            let sprite = assest_server.load(&structure.sprite_path);
            let icon = assest_server.load(&structure.icon_path);
            if let Some(recipe) = structure.recipe {
                recipe_registry.recipes.insert(structure.name.clone(), recipe);
            }
            let definition = StructureDefinition {
                sprite,
                icon,
                width: structure.width,
                height: structure.height,
                radius: structure.radius,
                interaction: structure.interaction,
            };
            struct_registry.structures.insert(structure.name.clone(), definition);
            count += 1;
        }
        println!("Loaded {} structures", count);
    }
}

pub fn setup_building_ui_nodes(
    mut commands: Commands,
    mut nodes: Query<(Entity, &mut BuildingUiSlot)>,
    structure_reg: Res<StructureRegistry>,
) {
    let mut added: Vec<String> = Vec::new();
    for (enitity, mut slot) in nodes.iter_mut() {
        let structures: Vec<(String, StructureDefinition)> = structure_reg.structures.iter()
            .map(|(name, def)| (name.clone(), def.clone()))
            .collect();
        for (name, def) in structures {
            if slot.structure.is_none() {
                if added.contains(&name) {
                    continue;
                }
                slot.structure = Some(name.clone());
                commands.entity(enitity).insert(ImageNode{ 
                    image: def.sprite, 
                    ..Default::default() 
                });
                added.push(name);
            }
        }
    }
}