use super::*;

pub fn update_map(
    mut reader: MessageReader<MapChanged>,
    mut query: Query<(&Transform, &MapTile, &mut Sprite), Without<Floor>>,
    tile_identifier: Query<(Entity, &MapTile), With<Wall>>,
    mut chunkgrid: ResMut<ChunkGrid>,
    worldgrid: Res<WorldGrid>,
) {
    for msg in reader.read() {
        if let Some(chunk) = chunkgrid.chunks.get_mut(&msg.chunk_pos) {
            let changed_idx = xy_idx(msg.local_pos.x as usize, msg.local_pos.y as usize);
            chunk.map[changed_idx] = TileType::Empty;
            let world_pos = tile_pos_to_world_pos(msg.local_pos, msg.chunk_pos);
            let cell_x = (world_pos.x / CELL_SIZE).round() as i32;
            let cell_y = (world_pos.y / CELL_SIZE).round() as i32;
            let cells_to_update = get_cells_3x3((cell_x, cell_y));
            let root_entities = get_entities_in_cells(cells_to_update, &worldgrid);
            for entity in root_entities {
                if let Ok((tf, map_tile, mut sprite)) = query.get_mut(entity) {
                    let translation = tf.translation.truncate();
                    let mut has_wall_north = false;
                    let mut has_wall_south = false;
                    let mut has_wall_east = false;
                    let mut has_wall_west = false;
                    let cell_x = (translation.x / CELL_SIZE).round() as i32;
                    let cell_y = (translation.y / CELL_SIZE).round() as i32;
                    if let Some(entities) = worldgrid.cells.get(&(cell_x, cell_y + 1)) {
                        for entity in entities {
                            if let Ok(_) = tile_identifier.get(*entity) {
                                has_wall_north = true;
                                break;
                            }
                        }
                    }
                    if let Some(entities) = worldgrid.cells.get(&(cell_x, cell_y - 1)) {
                        for entity in entities {
                            if let Ok(_) = tile_identifier.get(*entity) {
                                has_wall_south = true;
                                break;
                            }
                        }
                    }
                    if let Some(entities) = worldgrid.cells.get(&(cell_x - 1, cell_y)) {
                        for entity in entities {
                            if let Ok(_) = tile_identifier.get(*entity) {
                                has_wall_west = true;
                                break;
                            }
                        }
                    }
                    if let Some(entities) = worldgrid.cells.get(&(cell_x + 1, cell_y)) {
                        for entity in entities {
                            if let Ok(_) = tile_identifier.get(*entity) {
                                has_wall_east = true;
                                break;
                            }
                        }
                    }
                    let tile_idx = xy_idx(map_tile.local_pos.x, map_tile.local_pos.y);
                    let tile_type = pick_tile_type_in_world((has_wall_south, has_wall_north, has_wall_west, has_wall_east));
                    chunk.map[tile_idx] = tile_type;
                    chunk.changed = true;
                    let index = tile_type.tile_type_to_index();
                    if let Some(atlas) = sprite.texture_atlas.as_mut() {
                        atlas.index = index;
                    }
                }
            }
        } 
    }
}