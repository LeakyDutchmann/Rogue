use super::*;


pub fn detect_hit_system(
    world: Res<WorldGrid>,
    query: Query<&mut MapTile, With<Wall>>,
    mut reader: MessageReader<ImpactTrigger>,
    mut writer: MessageWriter<ApplyDestruction>,
) {
    for impact in reader.read() {
        let impact_pos = impact.target.unwrap();
        let item_used = impact.item;
        let cell_x = (impact_pos.x / CELL_SIZE).round() as i32;
        let cell_y = (impact_pos.y / CELL_SIZE).round() as i32;
        if let Some(cell_vec) = world.cells.get(&(cell_x, cell_y)) {
            for entity in cell_vec.iter() {
                if let Ok(tile) = query.get(*entity) {
                    if tile.tile_type != TileType::Floor && tile.tile_type != TileType::Empty {
                        println!("hit!!!");
                    }
                }

            }
        }
        
    }
}



