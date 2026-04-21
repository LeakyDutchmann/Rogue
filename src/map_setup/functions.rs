use super::*;


pub fn get_seed(global_seed: u64, pos_x: i32, pos_y: i32) -> u64 {
    let mut h = global_seed;
    h ^= (pos_x as u64)
        .wrapping_add(0x9e3779b97f4a7c15)
        .wrapping_add(h << 6)
        .wrapping_add(h >> 2);
    h ^= (pos_y as u64)
        .wrapping_add(0x9e3779b97f4a7c15)
        .wrapping_add(h << 6)
        .wrapping_add(h >> 2);
    h
}

pub fn is_wall(perlin: &Perlin, world_x: i32, world_y: i32) -> bool {
    let scale = 0.01;
    let val = perlin.get([
        world_x as f64 * scale,
        world_y as f64 * scale,
    ]);

    val > 0.0
}

pub fn get_chunk_pos(world_pos: Vec2) -> IVec2 {
    let half_chunk_width = CHUNK_WIDTH as f32 * TILE_SIZE / 2.0;
    let chunk_x = ((world_pos.x + half_chunk_width) / (CHUNK_WIDTH as f32 * TILE_SIZE)).floor() as i32;
    let chunk_y = ((world_pos.y + half_chunk_width) / (CHUNK_HEIGHT as f32 * TILE_SIZE)).floor() as i32;
    IVec2::new(chunk_x, chunk_y)
}

pub fn tile_pos_to_world_pos(local: IVec2, chunk_pos: IVec2) -> Vec2 {
    let pos_x = (chunk_pos.x as f32 * CHUNK_WIDTH as f32 * TILE_SIZE
        + local.x as f32 * TILE_SIZE)
        - CHUNK_WIDTH as f32 * TILE_SIZE / 2.0;
    let pos_y = (chunk_pos.y as f32 * CHUNK_HEIGHT as f32 * TILE_SIZE
        + local.y as f32 * TILE_SIZE)
        - CHUNK_HEIGHT as f32 * TILE_SIZE / 2.0;
    Vec2::new(pos_x, pos_y)
}

pub fn world_pos_to_tile_pos(world: Vec2, chunk_pos: IVec2) -> IVec2 {
    let local_x = (((world.x
                    - CHUNK_WIDTH as f32 * TILE_SIZE * chunk_pos.x as f32
                    + CHUNK_WIDTH as f32 * TILE_SIZE / 2.0)
                   / TILE_SIZE)
                  .floor() as i32)
                  .rem_euclid(CHUNK_WIDTH as i32) as u32;
    let local_y = (((world.y
                    - CHUNK_HEIGHT as f32 * TILE_SIZE * chunk_pos.y as f32
                    + CHUNK_HEIGHT as f32 * TILE_SIZE / 2.0)
                   / TILE_SIZE)
                  .floor() as i32)
                  .rem_euclid(CHUNK_HEIGHT as i32) as u32;
    IVec2::new(local_x as i32, local_y as i32)
}


pub fn xy_idx(x: usize, y: usize) -> usize {
    (y as usize * CHUNK_WIDTH) + x as usize
}

pub fn pick_tile_type_in_world(conditions: (bool, bool, bool, bool)) -> TileType {
    let tile_type =  match conditions {
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
    tile_type
}

pub fn chunk_path_from_pos(pos: IVec2) -> String {
    format!("world/{}_{}.chunk", pos.x, pos.y)
}

pub fn get_positions_of_saved_chunks() -> Option<Vec<IVec2>> {
    let mut items = Vec::new();
    let entries = fs::read_dir("world").ok()?;
    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();
        let file_name = match path.file_stem().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => continue,
        };
        let mut parts = file_name.split('_');
        let x = match parts.next()?.parse::<i32>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let y = match parts.next()?.parse::<i32>() {
            Ok(v) => v,
            Err(_) => continue,
        };

        items.push(IVec2::new(x, y));
    }
    Some(items)
}

