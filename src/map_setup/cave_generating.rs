use crate::map_setup::*;


pub fn generate_chunk(global_seed: u32, chunk_pos: IVec2) -> Vec<TileType> {
    let mut map = vec![TileType::Empty; CHUNK_WIDTH * CHUNK_HEIGHT];
    let perlin = Perlin::new(global_seed);

    for local_x in 0..CHUNK_WIDTH {
        for local_y in 0..CHUNK_HEIGHT {
            let world_x = (chunk_pos.x as i32 * CHUNK_WIDTH as i32 * TILE_SIZE as i32
                + local_x as i32 * TILE_SIZE as i32)
                - CHUNK_WIDTH as i32 * TILE_SIZE as i32 / 2
                + TILE_SIZE as i32 / 2;
            let world_y = (chunk_pos.y as i32 * CHUNK_HEIGHT as i32 * TILE_SIZE as i32
                + local_y as i32 * TILE_SIZE as i32)
                - CHUNK_HEIGHT as i32 * TILE_SIZE as i32 / 2
                + TILE_SIZE as i32 / 2;

            if !is_wall(&perlin, world_x , world_y) {
                continue;
            }
            
            let step = TILE_SIZE as i32;
            let wall_below = is_wall(&perlin, world_x, world_y - step);
            let wall_above = is_wall(&perlin, world_x, world_y + step);
            let wall_left = is_wall(&perlin, world_x - step, world_y);
            let wall_right = is_wall(&perlin, world_x + step, world_y);

            let tile_type = match (wall_below, wall_above, wall_left, wall_right) {
                (true, true, true, true) => TileType::WallCentre,
                (true, true, true, false) => TileType::WallSideEast,
                (false, false, true, true) => TileType::WallNorthSouth,
                (true, false, false, false) => TileType::WallEndSouth,
                (false, false, false, false) => TileType::WallSingle,
                (true, true, false, true) => TileType::WallSideWest,
                (true, false, true, false) => TileType::WallCornerNW,
                (false, false, true, false) => TileType::WallEndWest,  
                (true, false, true, true) => TileType::WallSideNorth,
                (false, false, false, true) => TileType::WallEndEast,                    
                (false, true, true, true) => TileType::WallSideSouth,                            
                (true, true, false, false) => TileType::WallEastWest,
                (false, true, false, true) => TileType::WallCornerSE,
                (false, true, true, false) => TileType::WallCornerSW,
                (true, false, false, true) => TileType::WallCornerNE,
                (false, true, false, false) => TileType::WallEndNorth,
            };

            map[xy_idx(local_x, local_y)] = tile_type;
        }
    }

    map
}




