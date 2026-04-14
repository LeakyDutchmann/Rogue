use super::*;

pub fn toggle_building_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut building_mode: ResMut<BuildingMode>,
    mut cursor: Query<&mut CursorStructureCarrier>,
) {
    if let Ok(mut cursor) = cursor.single_mut() {
        if keys.just_pressed(KeyCode::KeyB) {
            if building_mode.state == BuildingState::Off {
                building_mode.state = BuildingState::On;
            } else if building_mode.state == BuildingState::On {
                building_mode.state = BuildingState::Off;
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
            }
        }
    } 
}

pub fn set_building_ui_visibility(
    building_mode: Res<BuildingMode>,
    mut ui_node: Query<&mut Visibility, With<BuildingRootUiNode>>,
    mut inventory_open: ResMut<InventoryOpen>,
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
    can_place: Res<CanPlaceStruct>,
    mut writer_remove_from_inv: MessageWriter<RemoveFromInventory>,
    recipe_reg: Res<RecipeRegistry>,
    inventory: Query<&Inventory, With<Player>>,
) {
    if building_mode.state == BuildingState::Placing {
        for msg in reader.read() {
            let now = time.elapsed_secs_f64();
            if now - ui_click.last < 0.5 {
                continue;
            }
            if let Ok(mut cursor) = cursor.single_mut() {
                match msg {
                    MouseClickEvent::LeftClick(position) => {
                        if let Some(structure) = &cursor.structure {
                            if can_place.state == true {
                                if let Ok(inventory) = inventory.single() {
                                    if let Some(recipe) = recipe_reg.recipes.get(structure) {
                                        let mut ingredients: Vec<(String, i32)> = Vec::new();
                                        let mut missing_ingreients: Vec<(String, i32)> = Vec::new();
                                        for (item, quantity) in &recipe.ingredients {
                                            if check_if_inventory_has_item(inventory, item, quantity.clone()) {
                                                ingredients.push((item.clone(), quantity.clone()));
                                            } else {
                                                missing_ingreients.push((item.clone(), quantity.clone()));
                                                println!("missing ingredient: {} ({})", item, quantity);
                                            }
                                        }
                                        if missing_ingreients.is_empty() {
                                            for (item, quantity) in ingredients {
                                                writer_remove_from_inv.write(RemoveFromInventory {
                                                    quantity: quantity.clone(),
                                                    item: item.clone(),
                                                });
                                            }
                                            let chunk = get_chunk_pos(position.clone());
                                            writer.write(SpawnStructureRequest {
                                                position: position.clone(),
                                                item_id: structure.clone(),
                                                chunk_position: chunk,
                                            });
                                            cursor.structure = None;
                                            building_mode.state = BuildingState::On;
                                        }
                                    }
                                    
                                }
                            }
                        }
                    }
                    MouseClickEvent::RightClick(_position) => {
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
    mut writer: MessageWriter<MapChanged>,
    structure_reg: Res<StructureRegistry>,
    mut chunkgrid: ResMut<ChunkGrid>,
) {
    for msg in reader.read() {
        if let Some(def) = structure_reg.structures.get(&msg.item_id) {
            let structure = assemble_structure(&def, &mut commands, &msg.item_id);
            if let Some(chunk) = chunkgrid.chunks.get_mut(&msg.chunk_position) {
                chunk.changed = true;
                commands.entity(structure).insert(ParrentChunk { position: msg.chunk_position });
                commands.entity(structure).insert(Transform::from_translation(msg.position.extend((- msg.position.y + 1.0) * 0.001)));
            }
           
        }
    }
}

pub fn check_colision_static(pos1: Vec2, pos2: Vec2, size1: (f32, f32), size2: (f32, f32)) -> bool {
    let left_1 = pos1.x - size1.0 / 2.0;
    let right_1 = pos1.x + size1.0 / 2.0;
    let top_1 = pos1.y + size1.1 / 2.0;
    let bottom_1 = pos1.y - size1.1 / 2.0;
    
    let left_2 = pos2.x - size2.0 / 2.0;
    let right_2 = pos2.x + size2.0 / 2.0;
    let top_2 = pos2.y + size2.1 / 2.0;
    let bottom_2 = pos2.y - size2.1 / 2.0;
    
    let mut intersects = false;
    if right_1 < left_2 || left_1 > right_2 || top_1 < bottom_2 || bottom_1 > top_2 {
        intersects = false;
    } else {
        intersects = true;
    }
    intersects
}

pub fn can_place_structure(
    cursor: Query<&CursorStructureCarrier>,
    building_mode: Res<BuildingMode>,
    structure_reg: Res<StructureRegistry>,
    mut cursor_pos: Res<CursorWorldPos>,
    coliders: Query<(&Colider, &Transform)>,
    worldgrid: Res<WorldGrid>,
    mut can_place: ResMut<CanPlaceStruct>,
) {
    if building_mode.state == BuildingState::Placing {
        if let Ok(cursor) = cursor.single() {
            if let Some(structure) = &cursor.structure {
                if let Some(def) = structure_reg.structures.get(structure) {
                    if let Some(position) = cursor_pos.0 {
                        let width_1 = match def.width {
                            Some(w) => w,
                            None => 0.0,
                        };
                        let height_1 = match def.height {
                            Some(h) => h,
                            None => 0.0,
                        };
                        let cell_x = (position.x / CELL_SIZE ).floor() as i32;
                        let cell_y = (position.y / CELL_SIZE ).floor() as i32;
                        let central_cell = (cell_x, cell_y);
                        let cells = get_cells_3x3(central_cell);
                        let entities = get_entities_in_cells(cells, &worldgrid);
                        let mut intersected_any = false;
                        for entity in entities {
                            if let Ok((colider, tf)) = coliders.get(entity) {
                                match colider.shape {
                                    ColiderShape::Rectangle { width, height } => {
                                        if check_colision_static(position, tf.translation.truncate(), (width_1, height_1), (width, height)) {
                                            intersected_any = true;
                                            break;
                                        }
                                    },
                                    ColiderShape::Circle { radius } => {
                                        continue;
                                    },
                                }
                            }
                        }
                        if intersected_any {
                            can_place.state = false;
                        } else {
                            can_place.state = true;
                        }
                    }
                }
            }
        }
    }
    
}