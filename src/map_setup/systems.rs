use super::*;

pub fn track_player_pos(
    res: Res<PlayerTransform>,
    mut chunkgrid: ResMut<ChunkGrid>,
) {
    println!("player pos: {:?}", res.0);
}

pub fn spawn_chunk(
    mut commands: Commands,
    mut reader: MessageReader<SpawnChunk>,
    atlases: Res<MapAtlases>,
    global_seed: Res<GlobalSeed>,
    mut chunkgrid: ResMut<ChunkGrid>,
) {
    for msg in reader.read() {
        let seed_u64 = get_chunk_seed(global_seed.value, msg.position.x, msg.position.y);
        let seed = (seed_u64 ^ (seed_u64 >> 32)) as u32;
        let mut chunk_map = generate_chunk(seed);
        for local_x in 0..CHUNK_WIDTH {
            for local_y in 0..CHUNK_HEIGHT {
                let idx = xy_idx(local_x, local_y);
                let mut tile_type = chunk_map[idx];
                if tile_type != TileType::Empty {
                    tile_type = pick_tile_type(&chunk_map, local_x, local_y);
                }
                chunk_map[idx] = tile_type;
                let pos_x = (msg.position.x as f32 * CHUNK_WIDTH as f32 * TILE_SIZE
                    + local_x as f32 * TILE_SIZE)
                    - CHUNK_WIDTH as f32 * TILE_SIZE / 2.0
                    + TILE_SIZE / 2.0;
                let pos_y = (msg.position.y as f32 * CHUNK_HEIGHT as f32 * TILE_SIZE
                    + local_y as f32 * TILE_SIZE)
                    - CHUNK_HEIGHT as f32 * TILE_SIZE / 2.0
                    + TILE_SIZE / 2.0;
                let position = IVec2::new(pos_x as i32, pos_y as i32);
                let mut rng = rand::rng();
                let sprite_index = rng.random_range(0..3);
                if let Some(atlas) = atlases.atlases.get(&TileMaterial::Structurix) {
                    commands.spawn((
                        Sprite::from_atlas_image(
                                    atlas.texture.clone(),
                                    TextureAtlas {
                                        layout: atlas.layout.clone(),
                                        index: sprite_index,
                                    },
                                ),
                        Transform::from_xyz(pos_x, pos_y, -1.0),
                        MapTile { position, tile_type, material: TileMaterial::None},
                        ParrentChunk { position: msg.position },
                    ));
                }
            }
        }
        let chunk = Chunk {
            position: msg.position,
            map: chunk_map,
        };
        chunkgrid.chunks.insert(chunk.position, chunk);
        println!("spawned chunk at {:?}", msg.position);
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
        println!("despawned chunk at {:?}", msg.position);
    }
}

pub fn call_spawn_chunk(
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: MessageWriter<SpawnChunk>,
) {
    if keys.just_pressed(KeyCode::KeyJ) {
        writer.write(SpawnChunk { position: IVec2::new(-1, 0) });
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
        println!("player chunk: ({}, {})", center_chunk_x, center_chunk_y);
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
        if !chunkgrid.chunks.contains_key(&chunk_pos) {
            writer.write(SpawnChunk { position: chunk_pos.clone() });
        }
    }
    for (pos, _chunk) in chunkgrid.chunks.iter() {
        if !active_chunks.contains(pos) {
            disable_writer.write(DisableChunk { position: pos.clone() });
            println!("disabling chunk: ({}, {})", pos.x, pos.y);
        }
    }
}