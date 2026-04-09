use super::*;

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



