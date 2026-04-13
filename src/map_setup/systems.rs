use super::*;

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
                        Transform::from_xyz(tile.position.x, tile.position.y, -tile.position.y * 0.001 -10.0),
                        MapTile { 
                            local_pos: tile.local_pos,
                            tile_type: TileType::Floor,
                            material: TileMaterial::None,
                        },
                        ParrentChunk { position: chunk_data.position },
                        Floor,
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
                            Transform::from_xyz(tile.position.x, tile.position.y, -tile.position.y * 0.001),
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
                    changed: false,
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
    let player_chunk_pos = get_chunk_pos(player_pos);
    if player_chunk.position != player_chunk_pos {
        player_chunk.position = player_chunk_pos;
    }
    
}

pub fn chunk_handler(
    chunkgrid: Res<ChunkGrid>,
    player_chunk: Res<PlayerChunk>,
    saved: Res<SavedChunks>,
    mut writer: MessageWriter<SpawnChunk>,
    mut disable_writer: MessageWriter<DisableChunk>,
    mut save_writer: MessageWriter<SaveChunk>,
) {
    let active_chunks = vec![
        player_chunk.position,
        player_chunk.position + IVec2::new(0, 1),
        player_chunk.position + IVec2::new(1, 1),
        player_chunk.position + IVec2::new(1, 0),
        player_chunk.position + IVec2::new(1, -1),
        player_chunk.position + IVec2::new(0, -1),
        player_chunk.position + IVec2::new(-1, -1),
        player_chunk.position + IVec2::new(-1, 0),
        player_chunk.position + IVec2::new(-1, 1),
    ];
    for chunk_pos in &active_chunks {
        if !chunkgrid.chunks.contains_key(&chunk_pos) && !chunkgrid.pending_chunks.contains(&chunk_pos) {
            writer.write(SpawnChunk { position: chunk_pos.clone() });
        }
    }
    for (pos, chunk) in chunkgrid.chunks.iter() {
        if !active_chunks.contains(pos) {
            if chunk.changed {
                if !saved.chunks.contains_key(pos) {
                    save_writer.write(SaveChunk { position: pos.clone() });
                    continue;
                }
            }
            disable_writer.write(DisableChunk { position: pos.clone() });
        }
    }
}

pub fn update_map(
    mut commands: Commands,
    mut reader: MessageReader<MapChanged>,
    mut query: Query<(Entity, &Transform, &MapTile, &mut Sprite), Without<Floor>>,
    tile_identifier: Query<Entity, With<Wall>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                if let Ok((entity, tf, map_tile, mut sprite)) = query.get_mut(entity) {
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

pub fn save_chunk(
    mut commands: Commands,
    mut reader: MessageReader<SaveChunk>,
    mut disable_writer: MessageWriter<DisableChunk>,
    mut saved: ResMut<SavedChunks>,
    global_seed: Res<GlobalSeed>,
    chunkgrid: Res<ChunkGrid>,
) {
    for msg in reader.read() {
        if let Some(chunk) = chunkgrid.chunks.get(&msg.position) {
            let task_pool = AsyncComputeTaskPool::get();
            let chunk_pos = msg.position;
            let map_pointer = Arc::new(&chunk.map);
            let map = (*map_pointer).clone();
            let seed_value = global_seed.value;
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
                };
                saved_chunk
            });
            commands.spawn(
                SavingPendingChunk {
                    task: task
                }
            );
        }
        
    }
}

pub fn poll_saving_chunks(
    mut commands: Commands,
    mut chunks: Query<(Entity, &mut SavingPendingChunk)>,
    mut saved: ResMut<SavedChunks>,
    mut disable_writer: MessageWriter<DisableChunk>,
) {
    for (entity, mut pending) in chunks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut pending.task)) {
            let pos = result.position.clone();
            saved.chunks.insert(pos, result);
            println!("chunk saveed {:?}", pos);
            disable_writer.write(DisableChunk { position: pos });
            commands.entity(entity).despawn();
        }
    }
}