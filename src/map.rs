
use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::cave_generating::*;


#[derive(Resource)]
pub struct MapAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}


#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Floor, // atlas index = 1..3 **
    WallSideEast,
    WallSideWest,
    WallSideSouth,
    WallSideNorth,
    WallCentre,
    WallCornerSE,
    WallCornerSW,
    WallCornerNE,
    WallCornerNW,
    WallEndEast,
    WallEndWest,
    WallEndNorth,
    WallEndSouth,
    WallEastWest,
    WallNorthSouth,
    WallSingle,
    Empty, //no tile here. Going to use for corridors
}


pub const MAP_HEIGHT: usize = 50;
pub const MAP_WIDTH: usize = 80;
pub const TILE_SIZE: f32 = 32.0;


pub fn floor_setup(
    mut commands: Commands,
    atlas: Res<MapAtlas>,
) {
    let mut map = vec![TileType::Floor; 80 * 50];
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let idx = xy_idx(x, y);
            let tile_type = TileType::Floor;
            let mut rng = rand::rng();
            let sprite_index = rng.random_range(0..3);
            let pos_x = (x as f32 - MAP_WIDTH as f32 / 2.0) * TILE_SIZE;
            let pos_y = (y as f32 - MAP_HEIGHT as f32 / 2.0) * TILE_SIZE;

            commands.spawn((
                Sprite::from_atlas_image(
                            atlas.texture.clone(),
                            TextureAtlas {
                                layout: atlas.layout.clone(),
                                index: sprite_index,
                            },
                        ),
                Transform::from_xyz(pos_x, pos_y, 0.0),
                MapTile { x, y, tile_type },
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
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
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
                    _ => tile_type = TileType::Empty,
                }
                
            }
            // Select sprite index from atlas
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

            // Calculate position (centered on screen)
            let pos_x = (x as f32 - MAP_WIDTH as f32 / 2.0) * TILE_SIZE;
            let pos_y = (y as f32 - MAP_HEIGHT as f32 / 2.0) * TILE_SIZE;

            commands.spawn((
                Sprite::from_atlas_image(
                            atlas.texture.clone(),
                            TextureAtlas {
                                layout: atlas.layout.clone(),
                                index: sprite_index,
                            },
                        ),
                Transform::from_xyz(pos_x, pos_y, 0.0),
                MapTile { x, y, tile_type },
            ));
        }
    }
}


pub fn xy_idx(x: usize, y: usize) -> usize {
    (y * MAP_WIDTH) + x
}


fn has_tile_below(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if y == 0 {
        return false;
    }

    let idx = xy_idx(x, y);
    let tile_type = map[idx];

    let below_idx = xy_idx(x, y - 1);
    let below_tile_type = map[below_idx];

    below_tile_type == tile_type
}
fn has_tile_above(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if y + 1 >=  MAP_HEIGHT {
        return false;
    }

    let idx = xy_idx(x, y);
    let tile_type = map[idx];

    let below_idx = xy_idx(x, y + 1);
    let below_tile_type = map[below_idx];

    below_tile_type == tile_type
}
fn has_tile_left(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x == 0 {
        return false;
    }

    let idx = xy_idx(x, y);
    let tile_type = map[idx];

    let below_idx = xy_idx(x - 1, y);
    let below_tile_type = map[below_idx];

    below_tile_type == tile_type
}
fn has_tile_right(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x + 1 >= MAP_WIDTH {
        return false;
    }

    let idx = xy_idx(x, y);
    let tile_type = map[idx];

    let below_idx = xy_idx(x + 1, y);
    let below_tile_type = map[below_idx];

    below_tile_type == tile_type
}

//Functions bellow are unneeded for now, but you could use them later. If so, delete this line first. ***

// fn has_diagonal_upper_right(map: &Vec<TileType>, x: usize, y: usize) -> bool {
//     if x + 1 >= MAP_WIDTH || y + 1 >= MAP_HEIGHT {
//         return false;
//     }

//     let idx = xy_idx(x, y);
//     let tile_type = map[idx];

//     let below_idx = xy_idx(x + 1, y + 1);
//     let below_tile_type = map[below_idx];

//     below_tile_type == tile_type
// }
// fn has_diagonal_upper_left(map: &Vec<TileType>, x: usize, y: usize) -> bool {
//     if x == 0 || y + 1 >= MAP_HEIGHT {
//         return false;
//     }

//     let idx = xy_idx(x, y);
//     let tile_type = map[idx];

//     let below_idx = xy_idx(x - 1, y + 1);
//     let below_tile_type = map[below_idx];

//     below_tile_type == tile_type
// }
// fn has_diagonal_bottom_left(map: &Vec<TileType>, x: usize, y: usize) -> bool {
//     if x == 0 || y == 0 {
//         return false;
//     }

//     let idx = xy_idx(x, y);
//     let tile_type = map[idx];

//     let below_idx = xy_idx(x - 1, y - 1);
//     let below_tile_type = map[below_idx];

//     below_tile_type == tile_type
// }
// fn has_diagonal_bottom_right(map: &Vec<TileType>, x: usize, y: usize) -> bool {
//     if x + 1 >= MAP_WIDTH || y == 0 {
//         return false;
//     }

//     let idx = xy_idx(x, y);
//     let tile_type = map[idx];

//     let below_idx = xy_idx(x + 1, y - 1);
//     let below_tile_type = map[below_idx];

//     below_tile_type == tile_type
// }


