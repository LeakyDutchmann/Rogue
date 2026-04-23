use super::*;

pub fn interact_with_structure(
    structure_identifier: Query<(Entity,&StructureId, &Transform), With<Interactable>>,
    keys: Res<ButtonInput<KeyCode>>,
    player_transform: Res<PlayerTransform>,
    worldgrid: Res<WorldGrid>,
    mut interaction_state: ResMut<InteractionState>,
    mut console: ResMut<Console>,
) {
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
            interaction_state.interacting = true;
            interaction_state.object = Some(*entity);
            console.log(format!("Interacting with structure: {:?}", structure_id));        
        } else {
            console.log(format!("No interactable structure found near the player."));
        }
    }
}

