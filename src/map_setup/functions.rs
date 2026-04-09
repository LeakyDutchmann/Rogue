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

pub fn get_chunk_pos(world_pos: Vec2) -> IVec2 {
    let chunk_x = (world_pos.x / (CHUNK_WIDTH as f32 * TILE_SIZE)).round() as i32;
    let chunk_y = (world_pos.y / (CHUNK_HEIGHT as f32 * TILE_SIZE)).round() as i32;
    IVec2::new(chunk_x, chunk_y)
}

pub fn tile_pos_to_world_pos(local: IVec2, chunk_pos: IVec2) -> Vec2 {
    let pos_x = (chunk_pos.x as f32 * CHUNK_WIDTH as f32 * TILE_SIZE
        + local.x as f32 * TILE_SIZE)
        - CHUNK_WIDTH as f32 * TILE_SIZE / 2.0;
    let pos_y = (chunk_pos.y as f32 * CHUNK_HEIGHT as f32 * TILE_SIZE
        + local.y as f32 * TILE_SIZE)
        - CHUNK_HEIGHT as f32 * TILE_SIZE / 2.0;
    Vec2::new(pos_x, pos_y)
}

pub fn world_pos_to_tile_pos(world: Vec2, chunk_pos: IVec2) -> IVec2 {
    let local_x = (
        (world.x
        - (CHUNK_WIDTH as f32 * TILE_SIZE * chunk_pos.x as f32)
        + (CHUNK_WIDTH as f32 * TILE_SIZE / 2.0)
        ) / TILE_SIZE) as i32;
    let local_y = (
        (world.y
        - (CHUNK_HEIGHT as f32 * TILE_SIZE * chunk_pos.y as f32)
        + (CHUNK_HEIGHT as f32 * TILE_SIZE / 2.0)
        ) / TILE_SIZE) as i32;
    IVec2::new(local_x, local_y)
}

pub fn xy_idx(x: usize, y: usize) -> usize {
    (y as usize * CHUNK_WIDTH) + x as usize
}