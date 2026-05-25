use bevy::ecs::entity;

use super::*;

#[derive(Component)]
pub struct SectorBuff {
    pub hp: Option<i32>,
    pub speed: Option<i32>,
}


pub fn apply_sector_buff_system(
    mut health: Query<&mut Health>,
    mut speed: Query<&mut Speed>,
    enemy_reg: Res<EnemyRegistry>,
) {
 
}

pub fn track_enemies_near_player(
    enemy: Query<&Enemy>,
    player_tf: Res<PlayerTransform>,
    world: Res<WorldGrid>,
    mut console: ResMut<Console>,
) {
    let pos = player_tf.0.translation.truncate();
    let cell_x = (pos.x / CELL_SIZE).round() as i32;
    let cell_y = (pos.y / CELL_SIZE).round() as i32;
    let cells = get_cells_in_radius((cell_x, cell_y), 200.0);
    let entities = get_entities_in_cells(cells, &world);
    let mut count = 0;
    for entity in entities {
        if let Ok(_) = enemy.get(entity) {
            count += 1;
        }
    }
    console.log(format!("{:?} enemy near", count))
} 

