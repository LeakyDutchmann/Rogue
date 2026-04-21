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

pub fn prepare_chunk(
    mut commands: Commands,
    mut reader: MessageReader<PrepareChunk>,
    global_seed: Res<GlobalSeed>,
    mut chunk_grid: ResMut<ChunkGrid>,
) {
    let task_pool = AsyncComputeTaskPool::get();
    for msg in reader.read() {
        let seed_value = global_seed.value;
        let chunk_pos = msg.position;
        let task = task_pool.spawn(async move {
            let seed_u64 = get_seed(seed_value, chunk_pos.x, chunk_pos.y);
            let mut rng = StdRng::seed_from_u64(seed_u64);
            let chunk_map = generate_chunk(seed_value as u32, chunk_pos);
            
            let mut tiles: Vec<TileSpawnData> = Vec::with_capacity(CHUNK_WIDTH * CHUNK_HEIGHT);
            
            for local_x in 0..CHUNK_WIDTH {
                for local_y in 0..CHUNK_HEIGHT {
                    let idx = xy_idx(local_x, local_y);
                    let tile_type = chunk_map[idx];
                    
                    let position = tile_pos_to_world_pos(IVec2::new(local_x as i32, local_y as i32), chunk_pos);
                    let sprite_index = tile_type.tile_type_to_index();
                    let floor_index = rng.random_range(0..3);
                    let material = TileMaterial::pick_tile_material(&mut rng);
                    let tile = TileSpawnData {
                        position,
                        local_pos: USizeVec2::new(local_x, local_y),
                        tile_type,
                        material,
                        sprite_index,
                        floor_index,
                    };
                    tiles.push(tile);
                }
            }
            let chunk = ChunkSpawnData {
                position: chunk_pos,
                tiles,
                map: chunk_map,
                structures: Vec::new()
            };
            chunk
        });
        commands.spawn(PendingTaskChunk { task: task });
        chunk_grid.pending_chunks.insert(chunk_pos);
        
    }
}
