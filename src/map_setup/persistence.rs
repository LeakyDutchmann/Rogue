use super::*;

pub fn save_chunk(
    structures: Query<(&StructureId, &Transform, &ParrentChunk, &Health)>,
    mut commands: Commands,
    mut reader: MessageReader<SaveChunk>,
    global_seed: Res<GlobalSeed>,
    chunkgrid: Res<ChunkGrid>,
) {
    for msg in reader.read() {
        if let Some(chunk) = chunkgrid.chunks.get(&msg.position) {
            let task_pool = AsyncComputeTaskPool::get();
            let chunk_pos = msg.position;
            let map = chunk.map.clone();
            let seed_value = global_seed.value;
            let mut structures_in_chunk: Vec<StructureSpawnData> = Vec::new();
            for (struct_id, transform, parent_chunk, hp) in structures.iter() {
                if parent_chunk.position == chunk_pos {
                    let structure = StructureSpawnData {
                        id: struct_id.id.clone(),
                        pos: transform.translation.truncate(),
                        hp: hp.0,
                    };
                    structures_in_chunk.push(structure);
                }
            }
            let task = task_pool.spawn(async move {
                let seed_u64 = get_seed(seed_value, chunk_pos.x, chunk_pos.y);
                let mut rng = StdRng::seed_from_u64(seed_u64);
                let mut tiles: Vec<TileSpawnData> = Vec::new();
                for local_x in 0..CHUNK_WIDTH {
                    for local_y in 0..CHUNK_HEIGHT {
                        let idx = xy_idx(local_x, local_y);
                        let tile_type = map[idx];
                        
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
                let saved_chunk = ChunkSpawnData {
                    position: chunk_pos,
                    tiles,
                    map: map,
                    structures: structures_in_chunk,
                };
                if let Ok(serialized) = bincode::serialize(&saved_chunk) {
                    let path = chunk_path_from_pos(chunk_pos);
                    if let Err(e) = std::fs::write(&path, serialized) {
                        println!("FAILED WRITE {:?}: {:?}", path, e);
                    }
                } else {
                    println!("FAILED SERIALIZE chunk {:?}", chunk_pos);
                }
            });
            commands.spawn(
                SavingPendingChunk {
                    pos: chunk_pos,
                    task: task
                }
            );
        }
        
    }
}

pub fn chunk_loader(
    mut commands: Commands,
    mut reader: MessageReader<LoadChunk>,
    mut chunkgrid: ResMut<ChunkGrid>,
) {
    for msg in reader.read() {
        let chunk_pos = msg.position;
        let task_pool = AsyncComputeTaskPool::get();
        let task = task_pool.spawn(async move {
            let path = chunk_path_from_pos(chunk_pos);
            if let Ok(bytes) = std::fs::read(path) {
                if let Ok(chunk) = bincode::deserialize::<ChunkSpawnData>(&bytes) {
                    return Some(chunk);
                }
            }
            None
        });
        commands.spawn(
            LoadingPendingChunk {
                chunk: task,
            }
        );
        chunkgrid.pending_chunks.insert(chunk_pos);
        
    } 
}