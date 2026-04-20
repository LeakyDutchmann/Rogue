use crate::world;

use super::*;

struct Condition {
    has_wall_north: bool,
    has_wall_south: bool,
    has_wall_east: bool,
    has_wall_west: bool,
}

pub fn update_map(
    mut reader: MessageReader<MapChanged>,
    mut chunkgrid: ResMut<ChunkGrid>,
    worldgrid: Res<WorldGrid>,
    mut writer: MessageWriter<UpdateTile>,
) {
    for msg in reader.read() {
        let pos = msg.pos;
        let chunk_pos = get_chunk_pos(pos);
        let root_local = world_pos_to_tile_pos(pos, chunk_pos);
        if let Some(chunk) = chunkgrid.chunks.get_mut(&chunk_pos) {
            let idx = xy_idx(root_local.x as usize, root_local.y as usize);
            chunk.map[idx] = TileType::Empty;
            chunk.changed = true;
        }
        let tiles_to_update = vec![
            Vec2::new(pos.x, pos.y + TILE_SIZE),
            Vec2::new(pos.x, pos.y - TILE_SIZE, ),
            Vec2::new(pos.x + TILE_SIZE, pos.y),
            Vec2::new(pos.x - TILE_SIZE, pos.y),  
        ];
        let mut to_update: Vec<(IVec2, IVec2, TileType, Vec2)> = Vec::new();
        for (i, tile_pos) in tiles_to_update.iter().enumerate() {
            let tile_pos = *tile_pos;
            let chunk_pos_recursive = get_chunk_pos(tile_pos);
            if let Some(chunk) = chunkgrid.chunks.get(&chunk_pos_recursive) {
                let tile_local = world_pos_to_tile_pos(tile_pos, chunk_pos_recursive);
                let tile_idx = xy_idx(tile_local.x as usize, tile_local.y as usize);
                let tile_type = chunk.map[tile_idx];
                if tile_type == TileType::Empty || tile_type == TileType::Floor{
                    continue;
                }
                let mut condition = Condition {
                    has_wall_north: false,
                    has_wall_south: false,
                    has_wall_east: false,
                    has_wall_west: false,
                };
                let direction = [
                    Vec2::new(0.0, TILE_SIZE),
                    Vec2::new(0.0, -TILE_SIZE),
                    Vec2::new(TILE_SIZE, 0.0),
                    Vec2::new(-TILE_SIZE, 0.0),
                ];
                for (i, dir) in direction.iter().enumerate() {
                    let pos = tile_pos + dir;
                    let chunk_pos_rec_2 = get_chunk_pos(pos);
                    if let Some(chunk_recursive) = chunkgrid.chunks.get(&chunk_pos_rec_2) {
                        let local_idx_rec = world_pos_to_tile_pos(pos, chunk_pos_rec_2);
                        let tile_idx_rec = xy_idx(local_idx_rec.x as usize, local_idx_rec.y as usize);
                        let tile_type_rec = chunk_recursive.map[tile_idx_rec];
                        if tile_type_rec != TileType::Empty {
                            match i {
                                0 => condition.has_wall_north = true,
                                1 => condition.has_wall_south = true,
                                2 => condition.has_wall_east = true,
                                3 => condition.has_wall_west = true,
                                _ => {}
                            }
                        }
                    }
                }
                let final_type = pick_tile_type_in_world((
                    condition.has_wall_south,
                    condition.has_wall_north,
                    condition.has_wall_west,
                    condition.has_wall_east,
                ));
                to_update.push((chunk_pos_recursive, tile_local, final_type, tile_pos));
            }
        }
        for (chunk_pos, local_pos, final_type, tile_world_pos) in to_update {
            if let Some(chunk) = chunkgrid.chunks.get_mut(&chunk_pos) {
                let changed_idx = xy_idx(local_pos.x as usize, local_pos.y as usize);
                chunk.map[changed_idx] = final_type;
                chunk.changed = true;
                if final_type != TileType::Empty {
                    writer.write( UpdateTile { 
                        tile_position: tile_world_pos, 
                        tile_type: final_type, 
                    });
                }
            }
        }
    }
}

pub fn update_tiles(
    mut reader: MessageReader<UpdateTile>,
    mut query: Query<(&mut MapTile, &mut Sprite), Without<Floor>>,
    worldgrid: Res<WorldGrid>,
) {
    for msg in reader.read() {
        println!("updating tile at {:?} with type {:?}", msg.tile_position, msg.tile_type);
        let index = TileType::tile_type_to_index(msg.tile_type);
        let cell_x = (msg.tile_position.x / CELL_SIZE).round() as i32;
        let cell_y = (msg.tile_position.y / CELL_SIZE).round() as i32;
        if let Some(entities) = worldgrid.cells.get(&(cell_x, cell_y)) {
            for entity in entities {
                if let Ok((mut map_tile, mut sprite)) = query.get_mut(*entity) {
                    map_tile.tile_type = msg.tile_type;
                    if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
                        texture_atlas.index = index;
                        println!("updated tile");
                    }
                } else {
                }
            }
        }
    }
}

// pub fn update_map(
//     mut reader: MessageReader<MapChanged>,
//     mut query: Query<(&Transform, &mut MapTile, &mut Sprite), Without<Floor>>,
//     tile_identifier: Query<(Entity, &MapTile), With<Wall>>,
//     mut chunkgrid: ResMut<ChunkGrid>,
//     worldgrid: Res<WorldGrid>,
// ) {
//     for msg in reader.read() {
//         if let Some(chunk) = chunkgrid.chunks.get_mut(&msg.chunk_pos) {
//             let changed_idx = xy_idx(msg.local_pos.x as usize, msg.local_pos.y as usize);
//             chunk.map[changed_idx] = TileType::Empty;
//             let world_pos = tile_pos_to_world_pos(msg.local_pos, msg.chunk_pos);
//             let cell_x = (world_pos.x / CELL_SIZE).round() as i32;
//             let cell_y = (world_pos.y / CELL_SIZE).round() as i32;
//             let cells_to_update = get_cells_3x3((cell_x, cell_y));
//             let root_entities = get_entities_in_cells(cells_to_update, &worldgrid);
//             for entity in root_entities {
//                 if let Ok((tf, mut map_tile, mut sprite)) = query.get_mut(entity) {
//                     let translation = tf.translation.truncate();
//                     let mut has_wall_north = false;
//                     let mut has_wall_south = false;
//                     let mut has_wall_east = false;
//                     let mut has_wall_west = false;
//                     let cell_x = (translation.x / CELL_SIZE).round() as i32;
//                     let cell_y = (translation.y / CELL_SIZE).round() as i32;
//                     if let Some(entities) = worldgrid.cells.get(&(cell_x, cell_y + 1)) {
//                         for entity in entities {
//                             if let Ok(_) = tile_identifier.get(*entity) {
//                                 has_wall_north = true;
//                                 break;
//                             }
//                         }
//                     }
//                     if let Some(entities) = worldgrid.cells.get(&(cell_x, cell_y - 1)) {
//                         for entity in entities {
//                             if let Ok(_) = tile_identifier.get(*entity) {
//                                 has_wall_south = true;
//                                 break;
//                             }
//                         }
//                     }
//                     if let Some(entities) = worldgrid.cells.get(&(cell_x - 1, cell_y)) {
//                         for entity in entities {
//                             if let Ok(_) = tile_identifier.get(*entity) {
//                                 has_wall_west = true;
//                                 break;
//                             }
//                         }
//                     }
//                     if let Some(entities) = worldgrid.cells.get(&(cell_x + 1, cell_y)) {
//                         for entity in entities {
//                             if let Ok(_) = tile_identifier.get(*entity) {
//                                 has_wall_east = true;
//                                 break;
//                             }
//                         }
//                     }
//                     let tile_idx = xy_idx(map_tile.local_pos.x, map_tile.local_pos.y);
//                     let tile_type = pick_tile_type_in_world((has_wall_south, has_wall_north, has_wall_west, has_wall_east));
//                     map_tile.tile_type = tile_type;
//                     chunk.map[tile_idx] = tile_type;
//                     chunk.changed = true;
//                     let index = tile_type.tile_type_to_index();
//                     if let Some(atlas) = sprite.texture_atlas.as_mut() {
//                         atlas.index = index;
//                     }
//                 }
//             }
//         } 
//     }
// }