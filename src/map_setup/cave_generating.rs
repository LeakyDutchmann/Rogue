use crate::map_setup::*;
use noise::{NoiseFn, Perlin, Seedable};

pub fn generate_chunk(seed: u32) -> Vec<TileType> {
    let mut map = vec![TileType::Empty; CHUNK_WIDTH * CHUNK_HEIGHT];
    let perlin = Perlin::new(seed);
    let scale = 0.1;
    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            let val = perlin.get([x as f64 * scale, y as f64 * scale]);
            if val > -0.4 {
                let idx = xy_idx(x, y);
                map[idx] = TileType::WallCentre;
            }
        }
    }
    map
}




