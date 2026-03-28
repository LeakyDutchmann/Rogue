use super::*;

pub fn toggle_building_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut building_mode: ResMut<BuildingMode>,
    mut inventory_open: ResMut<InventoryOpen>,
    mut cursor: Query<&mut CursorStructureCarrier>,
) {
    if let Ok(mut cursor) = cursor.single_mut() {
        if keys.just_pressed(KeyCode::KeyB) {
            if building_mode.state == BuildingState::Off {
                building_mode.state = BuildingState::On;
                inventory_open.0 = true;
            } else if building_mode.state == BuildingState::On {
                building_mode.state = BuildingState::Off;
                inventory_open.0 = false;
            } 
        } 
        if keys.just_pressed(KeyCode::Escape) {
            if building_mode.state == BuildingState::Placing {
                building_mode.state = BuildingState::On;
                cursor.structure = None;
            }
            if building_mode.state == BuildingState::On {
                building_mode.state = BuildingState::Off;
                cursor.structure = None;
                inventory_open.0 = false;
            }
        }
    } 
}

pub fn set_building_ui_visibility(
    building_mode: Res<BuildingMode>,
    mut ui_node: Query<&mut Visibility, With<BuildingRootUiNode>>,
) {
    for mut visibility in ui_node.iter_mut() {
        *visibility = if building_mode.state == BuildingState::On {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub fn cursor_structure_carrier_update(
    mut commands: Commands,
    mut cursor: Query<(Entity, &mut CursorStructureCarrier, &mut Node)>,
    window: Single<&Window, With<PrimaryWindow>>,
    structure_reg: Res<StructureRegistry>,
    image_node: Query<&ImageNode>,
    placing_state: Res<PlacingState>,
) {
    if let Some(position) = window.cursor_position() {
        for (entity, mut carrier, mut node) in cursor.iter_mut() {
            node.left = Val::Px(position.x);
            node.top = Val::Px(position.y);
            if let Some(structure) = &carrier.structure {
                if let Some(def) = structure_reg.structures.get(structure) {
                    if image_node.get(entity).is_err() {
                        commands.entity(entity).insert(ImageNode {
                            image: def.icon.clone(),
                            ..Default::default()
                        });
                        println!("got crsr");
                    }
                }
            } else {
                commands.entity(entity).remove::<ImageNode>();
            }
        }
    }
}

pub fn build_structure(
    mut commands: Commands,
    mut cursor: Query<(&mut CursorStructureCarrier)>,
    mut reader: MessageReader<MouseClickEvent>,
    mut writer: MessageWriter<SpawnStructureRequest>,
    structure_reg: Res<StructureRegistry>,
    mut building_mode: ResMut<BuildingMode>,
    ui_click: Res<UiClickTrack>,
    time: Res<Time>,
) {
    if building_mode.state == BuildingState::Placing {
        for msg in reader.read() {
            if let MouseClickEvent::LeftClick(position) = msg {
                let now = time.elapsed_secs_f64();
                if now - ui_click.last < 0.5 {
                    continue;
                }
                if let Ok(mut cursor) = cursor.single_mut() {
                    if let Some(structure) = &cursor.structure {
                        writer.write(SpawnStructureRequest {
                            position: *position,
                            item_id: structure.clone(),
                        });
                        cursor.structure = None;
                        building_mode.state = BuildingState::On;
                    }
                }
            }
        }
    }
}

pub fn spawn_structure(
    mut commands: Commands,
    mut reader: MessageReader<SpawnStructureRequest>,
    structure_reg: Res<StructureRegistry>,
) {
    for msg in reader.read() {
        if let Some(def) = structure_reg.structures.get(&msg.item_id) {
            let structure = assemble_structure(&def, &mut commands, &msg.item_id);
            commands.entity(structure).insert(Transform::from_translation(msg.position.extend((MAX_Y - msg.position.y + 1.0) * 0.001)));
        }
    }
}