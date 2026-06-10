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
    let cells = vec![
        (cell_x + 1, cell_y),
        (cell_x - 1, cell_y),
        (cell_x, cell_y + 1),
        (cell_x, cell_y - 1),
    ];
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
        surrounding_slots.slots.push(cell);
    }
}

pub fn modify_slots_near(
    mut gizmos: Gizmos,
    surrounding_slots: Res<SlotsForSurrounding>,
) {
    for pos_i32 in surrounding_slots.slots.iter() {
        let pos = Vec2::new(pos_i32.0 as f32 * CELL_SIZE, pos_i32.1 as f32 * CELL_SIZE);
        gizmos.circle_2d(pos, 10.0, Color::srgba(0.0, 1.0, 0.0, 0.2));       
    }
}



#[derive(Component)]
pub struct Surrounding;

pub fn trim_slots_behind(enemy_cell: (i32, i32), player_cell: (i32, i32), slots: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    //creating Vec's 2 to calculate dot products later.
    let e_cell = Vec2::new(enemy_cell.0 as f32, enemy_cell.1 as f32);
    let p_cell = Vec2::new(player_cell.0 as f32, player_cell.1 as f32);
    let mut slots_to_proceed = Vec::new();
    for pos in slots.iter() {
        let pos_cell = Vec2::new(pos.0 as f32, pos.1 as f32);
        let to_slot = (pos_cell - p_cell).normalize();
        let to_enemy = (e_cell - p_cell).normalize();
        if to_slot.dot(to_enemy) >= 0.0 {
            slots_to_proceed.push(pos.clone(),);
        }
    }
    slots_to_proceed
}

pub fn pick_best_surrounding_slot(
    player_cell: (i32, i32),
    enemy_cell: (i32, i32),
    slots: &Vec<(i32, i32)>
) -> Option<(i32, i32)> {
    let mut value = f32::MIN;
    let mut best_slot: Option<(i32, i32)> = None;
    let p_pos = Vec2::new(player_cell.0 as f32, player_cell.1 as f32);
    let e_pos = Vec2::new(enemy_cell.0 as f32, enemy_cell.1 as f32);
    for pos in slots.iter() {
        let pos_f32 = Vec2::new(pos.0 as f32, pos.1 as f32);
        let distance = p_pos.distance(pos_f32);
        let distance_2 = e_pos.distance(pos_f32);
        let new_value = (1.0 / distance ) +  (1.0 / distance_2);
        if new_value > value {
            value = new_value;
            best_slot = Some(pos.clone());
        }
    }
    best_slot
}

pub fn start_surrounding(
    mut commands: Commands,
    swarm_buff: Res<SwarmBuffState>,
    player_tf: Res<PlayerTransform>,
    enemies: Query<(Entity, &Transform, &EnemyState), (With<Enemy>, Without<Surrounding>, Without<AiPath>)>,
    mut surrounding_slots: ResMut<SlotsForSurrounding>,
    mut writer: MessageWriter<FindPath>,
) {
    let player_pos = player_tf.0.translation.truncate();
    let p_cell_x = (player_pos.x / CELL_SIZE).round() as i32;
    let p_cell_y = (player_pos.y / CELL_SIZE).round() as i32;
    for (entity, tf, state) in enemies.iter() {
        if state.current != EnemyStateType::Surrounding {
            continue;
        }
        let enemy_pos = tf.translation.truncate();
        let e_cell_x = (enemy_pos.x / CELL_SIZE).round() as i32;
        let e_cell_y = (enemy_pos.y / CELL_SIZE).round() as i32;
        let slots = trim_slots_behind((e_cell_x, e_cell_y), (p_cell_x, p_cell_y), &surrounding_slots.slots);
        if let Some(best_slot) = pick_best_surrounding_slot((p_cell_x, p_cell_y), (e_cell_x, e_cell_y), &slots) {
            let slot_pos = Vec2::new(best_slot.0 as f32 * CELL_SIZE, best_slot.1 as f32 * CELL_SIZE);
            writer.write(FindPath {
                seeker: entity,
                seeker_pos: enemy_pos,
                target_pos: slot_pos,
            });
        }
    }
}
