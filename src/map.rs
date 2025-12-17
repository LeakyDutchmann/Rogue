
use bevy::prelude::*;
use rand::Rng;
use std::cmp::{max, min};
use crate::components::*;


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


const MAP_HEIGHT: usize = 50;
const MAP_WIDTH: usize = 80;
const TILE_SIZE: f32 = 32.0;


pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}


    
)
pub fn map_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Sprites.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 8, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let map = new_map_rooms_and_corridors();
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
                TileType::WallEndEast => 12,
                TileType::WallEndWest => 13,
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
                    texture.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: sprite_index,
                    },
                ),
                Transform::from_xyz(pos_x, pos_y, 0.0),
                MapTile { x, y, tile_type },
            ));
        }
    }
}


fn xy_idx(x: usize, y: usize) -> usize {
    (y * MAP_WIDTH) + x
}

fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::WallCentre; 80 * 50];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = rand::rng();

    for _ in 0..MAX_ROOMS {
        let w = rng.random_range(MIN_SIZE..MAX_SIZE);
        let h = rng.random_range(MIN_SIZE..MAX_SIZE);
        let x = rng.random_range(0..(80 - w - 1));
        let y = rng.random_range(0..(50 - h - 1));
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.random_range(0..2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }
    map
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x as usize, y as usize)] = TileType::Empty;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x as usize, y as usize);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Empty;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x as usize, y as usize);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Empty;
        }
    }
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
