use super::*;



pub fn track_player_pos(
    res: Res<PlayerTransform>,
    mut chunkgrid: ResMut<ChunkGrid>,
) {
    println!("player pos: {:?}", res.0);
}

pub struct TileSpawnData {
    pub position: Vec2,
    pub local_pos: USizeVec2,
    pub tile_type: TileType,
    pub material: TileMaterial,
    pub sprite_index: usize,
    pub floor_index: usize,
}

pub struct ChunkSpawnData {
    pub position: IVec2,
    pub tiles: Vec<TileSpawnData>,
    pub map: Vec<TileType>,
}

#[derive(Component)]
pub struct PendingChunk {
    pub task: Task<ChunkSpawnData>
}

pub fn prepare_chunk(
    mut commands: Commands,
    mut reader: MessageReader<SpawnChunk>,
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
            };
            chunk
        });
        commands.spawn(PendingChunk { task: task });
        chunk_grid.pending_chunks.insert(chunk_pos);
        
    }
}

pub fn spawn_chunk(
    mut chunkgrid: ResMut<ChunkGrid>,
    atlases: Res<MapAtlases>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PendingChunk)>,
) {
    for (entity, mut pending_chunk) in query.iter_mut() {
        if let Some(chunk_data) = future::block_on(future::poll_once(&mut pending_chunk.task)) {
            commands.entity(entity).despawn();
            for tile in chunk_data.tiles {
                if let Some(atlas) = atlases.atlases.get(&tile.material) {
                    commands.spawn((
                        Sprite::from_atlas_image(
                                    atlas.texture.clone(),
                                    TextureAtlas {
                                        layout: atlas.layout.clone(),
                                        index: tile.floor_index,
                                    },
                                ),
                        Transform::from_xyz(tile.position.x, tile.position.y, -1.0),
                        MapTile { 
                            local_pos: tile.local_pos,
                            tile_type: TileType::Floor,
                            material: TileMaterial::None,
                        },
                        ParrentChunk { position: chunk_data.position },
                    )); 
                    if tile.tile_type != TileType::Empty {
                        commands.spawn((
                            Sprite::from_atlas_image(
                                        atlas.texture.clone(),
                                        TextureAtlas {
                                            layout: atlas.layout.clone(),
                                            index: tile.sprite_index,
                                        },
                                    ),
                            Transform::from_xyz(tile.position.x, tile.position.y, (MAX_Y - tile.position.y + 1.0) * 0.001),
                            MapTile { 
                                local_pos: tile.local_pos,
                                tile_type: tile.tile_type,
                                material: tile.material,
                            },
                            Wall,
                            Colider {
                                shape: ColiderShape::Rectangle {
                                    width: TILE_SIZE,
                                    height: TILE_SIZE,
                                },
                                _offsety: 0.0,
                                _sensor: true,
                            },
                            Health(100),
                            ParrentChunk { position: chunk_data.position },
                        )); 
                    }
                }
            } 
            chunkgrid.chunks.insert(
                chunk_data.position,
                Chunk {
                    position: chunk_data.position,
                    map: chunk_data.map,
                },
            );
            chunkgrid.pending_chunks.remove(&chunk_data.position);
        }
    }
}

pub fn despawn_chunk(
    mut commands: Commands,
    mut reader: MessageReader<DisableChunk>,
    mut chunkgrid: ResMut<ChunkGrid>,
    query: Query<(Entity, &ParrentChunk)>,
) {
    for msg in reader.read() {
        for (entity, parrent_chunk) in query.iter() {
            if msg.position == parrent_chunk.position {
                commands.entity(entity).despawn();
            }
        }
        chunkgrid.chunks.remove(&msg.position);
    }
}

pub fn track_chunks(
    player_tf: Res<PlayerTransform>,
    mut chunkgrid: ResMut<ChunkGrid>,
    mut player_chunk: ResMut<PlayerChunk>,
) {
    let player_pos = player_tf.0.translation.truncate();
    let center_chunk_x = (player_pos.x / (CHUNK_WIDTH as f32 * TILE_SIZE)).round() as i32;
    let center_chunk_y = (player_pos.y / (CHUNK_HEIGHT as f32 * TILE_SIZE)).round() as i32;
    if player_chunk.position != IVec2::new(center_chunk_x, center_chunk_y) {
        player_chunk.position = IVec2::new(center_chunk_x, center_chunk_y);
    }
    
}

pub fn chunk_handler(
    chunkgrid: Res<ChunkGrid>,
    player_chunk: Res<PlayerChunk>,
    mut writer: MessageWriter<SpawnChunk>,
    mut disable_writer: MessageWriter<DisableChunk>,
) {
    let active_chunks = vec![
        player_chunk.position,
        player_chunk.position + IVec2::new(1, 0),
        player_chunk.position + IVec2::new(0, 1),
        player_chunk.position + IVec2::new(1, 1),
        player_chunk.position + IVec2::new(1, -1),
        player_chunk.position + IVec2::new(-1, 0),
        player_chunk.position + IVec2::new(-1, -1),
        player_chunk.position + IVec2::new(-1, 1),
        player_chunk.position + IVec2::new(0, -1),
        player_chunk.position + IVec2::new(0, 1),
    ];
    for chunk_pos in &active_chunks {
        if !chunkgrid.chunks.contains_key(&chunk_pos) && !chunkgrid.pending_chunks.contains(&chunk_pos) {
            writer.write(SpawnChunk { position: chunk_pos.clone() });
            println!("spawning chunk: ({}, {})", chunk_pos.x, chunk_pos.y);
        }
    }
    for (pos, _chunk) in chunkgrid.chunks.iter() {
        if !active_chunks.contains(pos) {
            disable_writer.write(DisableChunk { position: pos.clone() });
        }
    }
}

pub fn update_map(
    mut reader: MessageReader<MapChanged>,
) {
    for msg in reader.read() {
        println!("map changed: local_pos: ({}, {}), chunk_pos: ({}, {})",
            msg.local_pos.x, msg.local_pos.y, msg.chunk_pos.x, msg.chunk_pos.y);
    }
}
