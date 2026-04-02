use crate::map_setup::*;
use noise::{NoiseFn, Perlin, Seedable};

pub fn generate_cave() -> Vec<TileType> {
    let mut map = vec![TileType::Empty; MAP_WIDTH * MAP_HEIGHT];
    let mut seed: u32 = 32726;
    let perlin = Perlin::new(seed);
    let scale = 0.1;
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let val = perlin.get([x as f64 * scale, y as f64 * scale]);
            if val > -0.4 {
                let idx = xy_idx(x, y);
                map[idx] = TileType::WallCentre;
            }
        }
    }
    map
}


fn cave_condition(map: &Vec<TileType>, x: usize, y: usize)-> bool {
    let idx = xy_idx(x, y);
    let tiletype = map[idx];
    if tiletype == TileType::WallCentre {
        return true;
    } 
    let conditions = [
        has_wall_below(map, x, y),
        has_wall_above(map, x, y),
        has_wall_left(map, x, y),
        has_wall_right(map, x, y),
        has_wall_diagonal_upper_right(map, x, y),
        has_wall_diagonal_upper_left(map, x, y),
        has_wall_diagonal_bottom_left(map, x, y),
        has_wall_diagonal_bottom_right(map, x, y),
    ];
    let count = conditions.iter().filter(|&&b| b).count();
    count >= 3
}

fn has_wall_below(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if y == 0 {
        return false;
    }
    
    let below_idx = xy_idx(x, y - 1);
    let below_tile_type = map[below_idx];

    below_tile_type == TileType::WallCentre
}
fn has_wall_above(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if y + 1 >=  MAP_HEIGHT {
        return false;
    }

    let above_idx = xy_idx(x, y + 1);
    let above_tile_type = map[above_idx];

    above_tile_type == TileType::WallCentre
}
fn has_wall_left(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x == 0 {
        return false;
    }

    let left_idx = xy_idx(x - 1, y);
    let left_tile_type = map[left_idx];

    left_tile_type == TileType::WallCentre
}
fn has_wall_right(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x + 1 >= MAP_WIDTH {
        return false;
    }

    let right_idx = xy_idx(x + 1, y);
    let right_tile_type = map[right_idx];

    right_tile_type == TileType::WallCentre
}
fn has_wall_diagonal_upper_right(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x + 1 >= MAP_WIDTH || y + 1 >= MAP_HEIGHT {
        return false;
    }

    let diagonal_upper_righ_idx = xy_idx(x + 1, y + 1);
    let diagonal_upper_righ_type = map[diagonal_upper_righ_idx];

    diagonal_upper_righ_type == TileType::WallCentre
}
fn has_wall_diagonal_upper_left(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x == 0 || y + 1 >= MAP_HEIGHT {
        return false;
    }


    let iagonal_upper_left_idx = xy_idx(x - 1, y + 1);
    let iagonal_upper_left_type = map[iagonal_upper_left_idx];

    iagonal_upper_left_type == TileType::WallCentre
}
fn has_wall_diagonal_bottom_left(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 {
        return false;
    }


    let diagonal_bottom_left_idx = xy_idx(x - 1, y - 1);
    let diagonal_bottom_left_tile_type = map[diagonal_bottom_left_idx];

    diagonal_bottom_left_tile_type == TileType::WallCentre
}
fn has_wall_diagonal_bottom_right(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x + 1 >= MAP_WIDTH || y == 0 {
        return false;
    }

    let diagonal_bottom_right_idx = xy_idx(x + 1, y - 1);
    let diagonal_bottom_right_type = map[diagonal_bottom_right_idx];

    diagonal_bottom_right_type == TileType::WallCentre
}

