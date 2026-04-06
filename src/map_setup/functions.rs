use super::*;


pub fn get_seed(global_seed: u64, pos_x: i32, pos_y: i32) -> u64 {
    let mut h = global_seed;
    h ^= (pos_x as u64)
        .wrapping_add(0x9e3779b97f4a7c15)
        .wrapping_add(h << 6)
        .wrapping_add(h >> 2);
    h ^= (pos_y as u64)
        .wrapping_add(0x9e3779b97f4a7c15)
        .wrapping_add(h << 6)
        .wrapping_add(h >> 2);
    h
}

pub fn is_wall(perlin: &Perlin, world_x: i32, world_y: i32) -> bool {
    let scale = 0.01;
    let val = perlin.get([
        world_x as f64 * scale,
        world_y as f64 * scale,
    ]);

    val > 0.0
}