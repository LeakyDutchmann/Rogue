use crate::map_setup::*;

pub fn floor_setup(
    mut commands: Commands,
    atlas: Res<MapAtlas>,
) {
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let tile_type = TileType::Floor;
            let mut rng = rand::rng();
            let sprite_index = rng.random_range(0..3);
            let pos_x = (x as f32 - MAP_WIDTH as f32 / 2.0) * TILE_SIZE;
            let pos_y = (y as f32 - MAP_HEIGHT as f32 / 2.0) * TILE_SIZE;
            let position = IVec2::new(x as i32, y as i32);

            commands.spawn((
                Sprite::from_atlas_image(
                            atlas.texture.clone(),
                            TextureAtlas {
                                layout: atlas.layout.clone(),
                                index: sprite_index,
                            },
                        ),
                Transform::from_xyz(pos_x, pos_y, -1.0),
                MapTile { position, tile_type },
            ));
            
            
        }
    }
}   


pub fn setup_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Sprites.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 8, 8, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.insert_resource(MapAtlas {
        texture,
        layout: layout_handle,
    });
}



pub fn map_setup(
    mut commands: Commands,
    atlas: Res<MapAtlas>,
) {
    let map = generate_cave();
    commands.insert_resource(GameMap {
        tiles: map.clone(),
    });
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let idx = xy_idx(x, y);
            let mut tile_type = map[idx];
            if tile_type != TileType::Empty {
                tile_type = pick_tile_type(&map, x, y);
            }
            // Select sprite index from atlas
            let sprite_index = tile_type_to_index(tile_type);

            // Calculate position (centered on screen)
            let pos_x = (x as f32 - MAP_WIDTH as f32 / 2.0) * TILE_SIZE;
            let pos_y = (y as f32 - MAP_HEIGHT as f32 / 2.0) * TILE_SIZE;
            let position = IVec2::new(x as i32, y as i32);
            if tile_type != TileType::Empty {
                commands.spawn((
                Sprite::from_atlas_image(
                            atlas.texture.clone(),
                            TextureAtlas {
                                layout: atlas.layout.clone(),
                                index: sprite_index,
                            },
                        ),
                Transform::from_xyz(pos_x, pos_y, 0.0),
                MapTile { position, tile_type },
                Wall,
                Colider {
                    shape: ColiderShape::Rectangle {
                        width: TILE_SIZE,
                        height: TILE_SIZE,
                    },
                    offsety: 0.0,
                    sensor: true,
                },
            ));
            }
        }
    }
}


pub fn xy_idx(x: usize, y: usize) -> usize {
    (y as usize * MAP_WIDTH) + x as usize
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
    if y + 1 >=  MAP_HEIGHT {
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
    if x + 1 >= MAP_WIDTH {
        return false;
    }

    let right_idx = xy_idx(x + 1, y);
    let right_tile_type = map[right_idx];
    
    right_tile_type != TileType::Empty
}

fn tile_type_to_index(tile_type: TileType) -> usize {
    let sprite_index = match tile_type {
        TileType::Floor => 0, // First tile in spritesheet, takes first 3 spirtes.
        TileType::Empty => 19,
        TileType::WallSideEast => 4,
        TileType::WallSideWest => 3,
        TileType::WallSideSouth => 5,
        TileType::WallSideNorth => 6,
        TileType::WallCentre => 7,
        TileType::WallCornerSE => 8,
        TileType::WallCornerSW => 9,
        TileType::WallCornerNE => 10,
        TileType::WallCornerNW => 11,
        TileType::WallEndEast => 13,
        TileType::WallEndWest => 12,
        TileType::WallEndNorth => 14,
        TileType::WallEndSouth => 15,
        TileType::WallSingle => 16,
        TileType::WallNorthSouth => 18,
        TileType::WallEastWest => 17,
    };
    sprite_index
}


pub fn update_map(
    mut map: ResMut<GameMap>,
    mut query: Query<(&mut Sprite, &mut MapTile), With<Wall>>,
    mut reader: MessageReader<MapChanged>,
) {
    let mut copied = map.tiles.clone();
    for changes in reader.read() {
        println!("msg read");
        let changed_x = changes.position.x;
        let changed_y = changes.position.y;
        let changed_idx = xy_idx(changed_x as usize, changed_y as usize);
        copied[changed_idx] = TileType::Empty;
        for x in changed_x.saturating_sub(2)..=(changed_x + 2)  {
            if x as usize >= MAP_WIDTH { continue; }
            for y in changed_y.saturating_sub(2)..=(changed_y + 2) {
                if y as usize >= MAP_HEIGHT { continue; }
                
                let idx = xy_idx(x as usize, y as usize);
                let mut tile_type = copied[idx];
                println!("Now Saw tile with type: {:?} at ({}, {})", tile_type, x, y);
                tile_type = pick_tile_type(&copied, x as usize, y as usize);
                println!("changed tile type to {:?}", tile_type);
                map.tiles[idx] = tile_type;
                
                for (mut sprite, mut tile) in query.iter_mut() {
                    if tile.position.x == x && tile.position.y == y {
                        let tile_idx = xy_idx(tile.position.x as usize, tile.position.y as usize);
                        tile.tile_type = map.tiles[tile_idx];
                        if let Some(atlas) = sprite.texture_atlas.as_mut() {
                            atlas.index = tile_type_to_index(tile.tile_type);
                        }
                    } 
                }
            }
        }
    }
}

fn pick_tile_type(map: &Vec<TileType>, x: usize, y: usize) -> TileType {
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

