use crate::map_setup::*;


pub fn generate_cave() -> Vec<TileType> {
    let mut map = vec![TileType::Empty; MAP_WIDTH * MAP_HEIGHT];
    for x in 0..MAP_WIDTH {
        map[xy_idx(x, 0)] = TileType::WallCentre;
        map[xy_idx(x, 49)] = TileType::WallCentre;
    }
    for y in 0..MAP_HEIGHT {
        map[xy_idx(0, y)] = TileType::WallCentre;
        map[xy_idx(79, y)] = TileType::WallCentre;
    }
    
    let mut rng = rand::rng();
    for _i in 0..400 {
        let x = rng.random_range(1..79);
        let y = rng.random_range(1..49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::WallCentre;
        }
        
    }
    for _ in 0..5 {
        let old_map = map.clone();
    
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let idx = xy_idx(x, y);
    
                if cave_condition(&old_map, x, y) {
                    map[idx] = TileType::WallCentre;
                } else {
                    map[idx] = TileType::Empty;
                }
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

    let idx = xy_idx(x, y);

    let below_idx = xy_idx(x, y - 1);
    let below_tile_type = map[below_idx];

    below_tile_type == TileType::WallCentre
}
fn has_wall_above(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if y + 1 >=  MAP_HEIGHT {
        return false;
    }

    let idx = xy_idx(x, y);

    let below_idx = xy_idx(x, y + 1);
    let below_tile_type = map[below_idx];

    below_tile_type == TileType::WallCentre
}
fn has_wall_left(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x == 0 {
        return false;
    }

    let idx = xy_idx(x, y);

    let below_idx = xy_idx(x - 1, y);
    let below_tile_type = map[below_idx];

    below_tile_type == TileType::WallCentre
}
fn has_wall_right(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x + 1 >= MAP_WIDTH {
        return false;
    }

    let idx = xy_idx(x, y);

    let below_idx = xy_idx(x + 1, y);
    let below_tile_type = map[below_idx];

    below_tile_type == TileType::WallCentre
}
fn has_wall_diagonal_upper_right(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x + 1 >= MAP_WIDTH || y + 1 >= MAP_HEIGHT {
        return false;
    }

    let idx = xy_idx(x, y);

    let below_idx = xy_idx(x + 1, y + 1);
    let below_tile_type = map[below_idx];

    below_tile_type == TileType::WallCentre
}
fn has_wall_diagonal_upper_left(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x == 0 || y + 1 >= MAP_HEIGHT {
        return false;
    }

    let idx = xy_idx(x, y);

    let below_idx = xy_idx(x - 1, y + 1);
    let below_tile_type = map[below_idx];

    below_tile_type == TileType::WallCentre
}
fn has_wall_diagonal_bottom_left(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 {
        return false;
    }

    let idx = xy_idx(x, y);

    let below_idx = xy_idx(x - 1, y - 1);
    let below_tile_type = map[below_idx];

    below_tile_type == TileType::WallCentre
}
fn has_wall_diagonal_bottom_right(map: &Vec<TileType>, x: usize, y: usize) -> bool {
    if x + 1 >= MAP_WIDTH || y == 0 {
        return false;
    }

    let idx = xy_idx(x, y);
    let tile_type = map[idx];

    let below_idx = xy_idx(x + 1, y - 1);
    let below_tile_type = map[below_idx];

    below_tile_type == TileType::WallCentre
}

