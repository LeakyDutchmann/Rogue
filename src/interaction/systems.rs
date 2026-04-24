use super::*;

pub fn interact_with_structure(
    structure_identifier: Query<(Entity,&StructureId, &Transform), With<Interactable>>,
    keys: Res<ButtonInput<KeyCode>>,
    player_transform: Res<PlayerTransform>,
    worldgrid: Res<WorldGrid>,
    mut interaction_state: ResMut<InteractionState>,
    mut console: ResMut<Console>,
    struct_reg: Res<StructureRegistry>,
) {
    if interaction_state.interacting == InteractionStage::Idle {
        if keys.just_pressed(KeyCode::KeyF) {
            let player_pos = player_transform.0.translation.truncate();
            let cell_x = (player_pos.x / CELL_SIZE).floor() as i32;
            let cell_y = (player_pos.y / CELL_SIZE).floor() as i32;
            let cells = get_cells_3x3((cell_x, cell_y));
            let entities = get_entities_in_cells(cells, &worldgrid);
            let mut near_structs: Vec<(String, Vec2, Entity)> = Vec::new();
            for entity in entities {
                if let Ok((entity, structure_id, tf)) = structure_identifier.get(entity) {
                    let pos = tf.translation.truncate();
                    near_structs.push((structure_id.id.clone(), pos, entity));
                }
            }
            let mut nearest_struct: Option<(String, Vec2, Entity)> = None;
            for (structure_id, pos, entity) in near_structs {
                if let Some((_struct_id, position, nearest_entity)) = &nearest_struct {
                    if player_pos.distance(pos) < player_pos.distance(position.clone()) {
                        nearest_struct = Some((structure_id, pos, entity));
                    }
                } else {
                    nearest_struct = Some((structure_id, pos, entity));
                }
            }
            if let Some((structure_id, position, entity)) = &nearest_struct {
                if let Some(definition) = struct_reg.structures.get(structure_id) {
                    interaction_state.interaction_type = definition.interaction.clone();
                }
                interaction_state.interacting = InteractionStage::Intializing;
                interaction_state.entity = Some(*entity);
                console.log(format!("Interacting with structure: {:?}", structure_id));        
            } else {
                console.log(format!("No interactable structure found near the player."));
            }
        } 
    } else {
        if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::KeyF) {
            interaction_state.interacting = InteractionStage::Idle;
            interaction_state.entity = None;
            interaction_state.interaction_type = InteractionType::None;
        }
    }
    
    
}

pub fn show_structure_window(
    mut commands: Commands,
    mut interaction_state: ResMut<InteractionState>,
    mut console: ResMut<Console>,
    mut writer: MessageWriter<SpawnWindowRequest>,
    mut close_writer: MessageWriter<CloseWindowRequest>,
) {
    if interaction_state.interacting == InteractionStage::Intializing {
        console.log(format!("Interacting on {:?}", interaction_state.interaction_type));
        match interaction_state.interaction_type {
            InteractionType::None => {}
            InteractionType::WorkBench => {
                console.log(format!("Work bench is not implemented"));
            }
            InteractionType::BasicOven => {
                writer.write(SpawnWindowRequest {
                    window_type: WindowType::BasicOven,
                });
                interaction_state.interacting = InteractionStage::Interacting;
            }
        }
    } else if interaction_state.interacting == InteractionStage::Idle {
        close_writer.write(CloseWindowRequest);
    }
}

pub fn close_window(
    mut commands: Commands,
    mut window: Query<Entity, With<UiStructureWindow>>,
    mut reader: MessageReader<CloseWindowRequest>,
) {
    for msg in reader.read() {
        for entity in window.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}
