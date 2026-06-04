use super::*;

pub fn calculate_slots_around_player(
    player_tf: Res<PlayerTransform>,
    world_grid: Res<WorldGrid>,
    mut surrounding_slots: ResMut<SlotsForSurrounding>,
    wall: Query<&Wall>,
) {
    let player_pos = player_tf.0.translation.truncate();
    let cell_x = (player_pos.x / CELL_SIZE).round() as i32;
    let cell_y = (player_pos.y / CELL_SIZE).round() as i32;
    let cells = get_cells_in_radius((cell_x, cell_y), 64.0);
    let mut empty_cells = Vec::new();
    for cell in cells {
        if let Some(entities) = world_grid.cells.get(&cell) {
            let mut wall_found = false;
            for entity in entities {
                if let Ok(_) = wall.get(*entity) {
                    wall_found = true;
                    break;
                }
            }
            if !wall_found {
                empty_cells.push(cell);
            }
        }
    }
    surrounding_slots.slots.clear();
    for cell in empty_cells {
        if cell == (cell_x, cell_y) {
            continue;
        }
        surrounding_slots.slots.insert(cell, false);
    }
}

pub fn modify_slots_near(
    mut gizmos: Gizmos,
    surrounding_slots: Res<SlotsForSurrounding>,
) {
    for (pos_i32, state) in surrounding_slots.slots.iter() {
        let pos = Vec2::new(pos_i32.0 as f32 * CELL_SIZE, pos_i32.1 as f32 * CELL_SIZE);
        if *state {
            gizmos.circle_2d(pos, 10.0, Color::srgba(1.0, 0.0, 0.0, 0.2));
        } else {
            gizmos.circle_2d(pos, 10.0, Color::srgba(0.0, 1.0, 0.0, 0.2));
        }
        
    }
}

pub fn track_surrounding_slots_accesibility(
    world_grid: Res<WorldGrid>,
    mut surrounding_slots: ResMut<SlotsForSurrounding>,
    enemy: Query<&ActorState, With<Enemy>>,
) {
    for (pos_i32, mut state) in surrounding_slots.slots.iter_mut() {
        if let Some(entities) = world_grid.cells.get(pos_i32) {
            for entity in entities {
                if let Ok(actor_state) = enemy.get(*entity) {
                    if actor_state.state != ActorStateType::Dead {
                        *state = true;
                        break;
                    }
                    
                }
            }
        }
    }
}