use super::*;


pub fn hit_detection_system(
    world: Res<WorldGrid>,
    query: Query<&mut MapTile, With<Wall>>,
    mut reader: MessageReader<HitMessage>,
    mut writer: MessageWriter<ApplyDamage>,
) {
    for impact in reader.read() {
        let impact_pos = impact.target.unwrap();
        let _item_used = impact.item;
        let cell_x = (impact_pos.x / CELL_SIZE).round() as i32;
        let cell_y = (impact_pos.y / CELL_SIZE).round() as i32;
        if let Some(cell_vec) = world.cells.get(&(cell_x, cell_y)) {
            for entity in cell_vec.iter() {
                if let Ok(tile) = query.get(*entity) {
                    if tile.tile_type != TileType::Floor && tile.tile_type != TileType::Empty {
                        println!("hitted tile");
                        writer.write(ApplyDamage {
                            entity: *entity,
                            position: world_pos_to_tile_pos(impact_pos),
                            damage: 30,
                        });
                    }
                }

            }
        }
        
    }
}



