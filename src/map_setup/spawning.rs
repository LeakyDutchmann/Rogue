use super::*;

pub fn spawn_chunk(
    mut chunkgrid: ResMut<ChunkGrid>,
    atlases: Res<MapAtlases>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PendingChunk)>,
    struct_reg: Res<StructureRegistry>,
    mut writer: MessageWriter<SpawnStructureRequest>,
) {
    for (entity, mut pending_chunk) in query.iter_mut() {
        let tiles = std::mem::take(&mut pending_chunk.chunk.tiles);
        let map = std::mem::take(&mut pending_chunk.chunk.map);
        let structures = std::mem::take(&mut pending_chunk.chunk.structures);
        
        commands.entity(entity).despawn();
        for tile in tiles {
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
                    ParrentChunk { position: pending_chunk.chunk.position },
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
                        ParrentChunk { position: pending_chunk.chunk.position },
                    )); 
                }
            }
        } 
        for structure in structures {
            writer.write(SpawnStructureRequest {
                position: structure.pos,
                item_id: structure.id,
                chunk_position: pending_chunk.chunk.position,
            });
        }
        chunkgrid.chunks.insert(
            pending_chunk.chunk.position,
            Chunk {
                position: pending_chunk.chunk.position,
                map: map,
                changed: false,
            },
        );
        chunkgrid.pending_chunks.remove(&pending_chunk.chunk.position);
    }
}

pub fn despawn_chunk(
    mut commands: Commands,
    mut reader: MessageReader<DisableChunk>,
    mut chunkgrid: ResMut<ChunkGrid>,
    query: Query<(Entity, &ParrentChunk, &Transform)>,
    mut worldgrid: ResMut<WorldGrid>,
) {
    for msg in reader.read() {
        for (entity, parrent_chunk, transform) in query.iter() {
            let pos = transform.translation;
            let cell_x = (pos.x / CELL_SIZE).floor() as i32;
            let cell_y = (pos.y / CELL_SIZE).floor() as i32;
            if msg.position == parrent_chunk.position {
                if let Some(entities) = worldgrid.cells.get_mut(&(cell_x, cell_y)) {
                    entities.retain(|&e| e != entity);
                }
                commands.entity(entity).despawn();          
            }
        }
        chunkgrid.chunks.remove(&msg.position);
        println!("chunk despawned {:?}", msg.position);
    }
}