use crate::map_setup::*;

pub fn floor_setup(
    mut commands: Commands,
    atlases: Res<MapAtlases>,
) {
    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            let tile_type = TileType::Floor;
            let mut rng = rand::rng();
            let sprite_index = rng.random_range(0..3);
            let pos_x = (x as f32 - CHUNK_WIDTH as f32 / 2.0) * TILE_SIZE;
            let pos_y = (y as f32 - CHUNK_HEIGHT as f32 / 2.0) * TILE_SIZE;
            let position = IVec2::new(x as i32, y as i32);
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
                ));
            }
        }
    }
}   



pub fn setup_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut map_atlases: ResMut<MapAtlases>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 8, 8, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);
    map_atlases.atlases.insert(TileMaterial::Structurix, MapAtlas {
        texture: asset_server.load("tiles/Structurix.png"),
        layout: layout_handle.clone(),
    });
    map_atlases.atlases.insert(TileMaterial::Mechanae, MapAtlas {
        texture: asset_server.load("tiles/Mechanae.png"),
        layout: layout_handle.clone(),
    });
    map_atlases.atlases.insert(TileMaterial::Secturix, MapAtlas {
        texture: asset_server.load("tiles/Secturix.png"),
        layout: layout_handle.clone(),
    });
}

pub fn map_setup(
    mut commands: Commands,
    atlases: Res<MapAtlases>,
    global_seed: Res<GlobalSeed>,
    mut chunkgrid: ResMut<ChunkGrid>,
    mut writer: MessageWriter<SpawnChunk>,
) {
    commands.insert_resource(GameMap {
        tiles: Vec::new(),
    });
    // writer.write(SpawnChunk {position: IVec2::new(0, 0) });
    // writer.write(SpawnChunk { position: IVec2::new(0, 1) });
    // writer.write(SpawnChunk { position: IVec2::new(0, -1) });
    // writer.write(SpawnChunk { position: IVec2::new(1, 0) });
    // writer.write(SpawnChunk { position: IVec2::new(-1, 0) });
    // writer.write(SpawnChunk { position: IVec2::new(1, 1) });
    // writer.write(SpawnChunk { position: IVec2::new(-1, -1) });
    // writer.write(SpawnChunk { position: IVec2::new(1, -1) });
    // writer.write(SpawnChunk { position: IVec2::new(-1, 1) });
    
}

// pub fn old_map_setup(´__
//     mut commands: Commands,
//     atlases: Res<MapAtlases>,
//     global_seed: Res<GlobalSeed>,
//     mut chunkgrid: ResMut<ChunkGrid>,
// ) {
//     let map = generate_chunk(global_seed.value as u32);
//     commands.insert_resource(GameMap {
//         tiles: map.clone(),
//     });
//     let mut chunk_map: Vec<TileType> = map.clone();
//     for y in 0..CHUNK_HEIGHT {
//         for x in 0..CHUNK_WIDTH {
//             let idx = xy_idx(x, y);
//             let mut tile_type = map[idx];
//             if tile_type != TileType::Empty {
//                 tile_type = pick_tile_type(&map, x, y);
//             }
//             chunk_map[idx] = tile_type;
//             // Select sprite index from atlas
//             let sprite_index = tile_type.tile_type_to_index();

//             // Calculate position (centered on screen)
//             let pos_x = (x as f32 - CHUNK_WIDTH as f32 / 2.0) * TILE_SIZE;
//             let pos_y = (y as f32 - CHUNK_HEIGHT as f32 / 2.0) * TILE_SIZE;
//             let position = IVec2::new(x as i32, y as i32);
//             if tile_type != TileType::Empty {
//                 // let material = TileMaterial::pick_tile_material();
//                 if let Some(atlas) = atlases.atlases.get(&material) {
//                     let texture = atlas.texture.clone();
//                     commands.spawn((
//                     Sprite::from_atlas_image(
//                                 texture,
//                                 TextureAtlas {
//                                     layout: atlas.layout.clone(),
//                                     index: sprite_index,
//                                 },
//                             ),
//                     Transform::from_xyz(pos_x, pos_y, (MAX_Y - pos_y + 1.0) * 0.001),
//                     MapTile { 
//                         position,
//                         tile_type,
//                         material: material,
//                     },
//                     Wall,
//                     Colider {
//                         shape: ColiderShape::Rectangle {
//                             width: TILE_SIZE,
//                             height: TILE_SIZE,
//                         },
//                         _offsety: 0.0,
//                         _sensor: true,
//                     },
//                     Health(100),
//                 ));
//                 }    
//             }
//         }    
//     }
// }

pub fn xy_idx(x: usize, y: usize) -> usize {
    (y as usize * CHUNK_WIDTH) + x as usize
}

fn has_tile_below(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if y == 0 {
        return false;
    }
    let below_idx = xy_idx(x, y - 1);
    let below_tile_type = map[below_idx];
    
    below_tile_type != TileType::Empty 

}

fn has_tile_above(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if y + 1 >=  CHUNK_HEIGHT {
        return false;
    }

    let above_idx = xy_idx(x, y + 1);
    let above_tile_type = map[above_idx];
    
    above_tile_type != TileType::Empty 
}

fn has_tile_left(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x == 0 {
        return false;
    }

    let left_idx = xy_idx(x - 1, y);
    let left_tile_type = map[left_idx];
    
    left_tile_type != TileType::Empty 

}

fn has_tile_right(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x + 1 >= CHUNK_WIDTH {
        return false;
    }

    let right_idx = xy_idx(x + 1, y);
    let right_tile_type = map[right_idx];
    
    right_tile_type != TileType::Empty
}

pub fn update_map(
    mut map: ResMut<GameMap>,
    mut query: Query<(&mut Sprite, &mut MapTile), With<Wall>>,
    mut reader: MessageReader<MapChanged>,
) {
    let mut copied = map.tiles.clone();
    for changes in reader.read() {
        let changed_x = changes.position.x;
        let changed_y = changes.position.y;
        let changed_idx = xy_idx(changed_x as usize, changed_y as usize);
        copied[changed_idx] = TileType::Empty;
        for x in changed_x.saturating_sub(2)..=(changed_x + 2)  {
            if x as usize >= CHUNK_WIDTH {
                continue; 
            }
            for y in changed_y.saturating_sub(2)..=(changed_y + 2) {
                if y as usize >= CHUNK_HEIGHT {
                    continue; 
                }
                let idx = xy_idx(x as usize, y as usize);
                let mut tile_type = copied[idx];
                //come back and fix this unreaded tile_type
                tile_type = pick_tile_type(&copied, x as usize, y as usize);
                map.tiles[idx] = tile_type;
                for (mut sprite, mut tile) in query.iter_mut() {
                    if tile.position.x == x && tile.position.y == y {
                        let tile_idx = xy_idx(tile.position.x as usize, tile.position.y as usize);
                        tile.tile_type = map.tiles[tile_idx];
                        if let Some(atlas) = sprite.texture_atlas.as_mut() {
                            atlas.index = tile.tile_type.tile_type_to_index();
                        }
                    } 
                }
            }
        }
    }
}

pub fn pick_tile_type(map: &Vec<TileType>, x: usize, y: usize) -> TileType {
    let idx = xy_idx(x, y);
    let mut tile_type = map[idx];
    if tile_type != TileType::Empty {
        let condition = (
            has_tile_below(&map, x, y),
            has_tile_above(&map, x, y),
            has_tile_left(&map, x, y),
            has_tile_right(&map, x, y)
        );
        match condition {
            (true, true, true, true) => tile_type = TileType::WallCentre,
            (true, true, true, false) => tile_type = TileType::WallSideEast,
            (false, false, true, true) => tile_type = TileType::WallNorthSouth,
            (true, false, false, false) => tile_type = TileType::WallEndSouth,
            (false, false, false, false) => tile_type = TileType::WallSingle,
            (true, true, false, true) => tile_type = TileType::WallSideWest,
            (true, false, true, false) => tile_type = TileType::WallCornerNW,
            (false, false, true, false) => tile_type = TileType::WallEndWest,  
            (true, false, true, true) => tile_type = TileType::WallSideNorth,
            (false, false, false, true) => tile_type = TileType::WallEndEast,                    
            (false, true, true, true) => tile_type = TileType::WallSideSouth,                            
            (true, true, false, false) => tile_type = TileType::WallEastWest,
            (false, true, false, true) => tile_type = TileType::WallCornerSE,
            (false, true, true, false) => tile_type = TileType::WallCornerSW,
            (true, false, false, true) => tile_type = TileType::WallCornerNE,
            (false, true, false, false) => tile_type = TileType::WallEndNorth,
        }
    }
    tile_type
}


pub fn world_pos_to_tile_pos(world: Vec2) -> IVec2 {
    let x = ((world.x + (CHUNK_WIDTH as f32 / 2.0) * TILE_SIZE) / TILE_SIZE).round() as i32;
    let y = ((world.y + (CHUNK_HEIGHT as f32 / 2.0) * TILE_SIZE) / TILE_SIZE).round() as i32;
    IVec2::new(x, y)
}

pub fn tile_pos_to_world_pos(tile: IVec2) -> Vec2 {
    let x = tile.x as f32 * TILE_SIZE
        - (CHUNK_WIDTH as f32 / 2.0) * TILE_SIZE;

    let y = tile.y as f32 * TILE_SIZE
        - (CHUNK_HEIGHT as f32 / 2.0) * TILE_SIZE;

    Vec2::new(x, y)
}