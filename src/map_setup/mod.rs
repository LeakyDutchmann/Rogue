mod setup;
mod cave_generating;
mod components;
mod systems;
mod functions;

use bevy::prelude::*;
use rand::Rng;
use std::collections::{HashMap, HashSet};
pub use setup::*;
pub use components::*;
use cave_generating::*;
use crate::components::Health;
use crate::colision_manager::{Colider, ColiderShape};
use crate::messages::{MapChanged, PrepareChunk, DisableChunk, SaveChunk, DirectChunkSpawnRequest, SpawnChunk};
use crate::player::PlayerTransform;
use rand::rngs::StdRng;
use rand::SeedableRng;
pub use systems::*;
pub use functions::*;
use noise::{NoiseFn, Perlin, Seedable};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use std::sync::{Arc};
use bevy::math::USizeVec2;
use crate::world::{CELL_SIZE, get_cells_3x3, get_entities_in_cells, WorldGrid};


pub struct MapSetupPlugin;

impl Plugin for MapSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapAtlases {
            atlases: HashMap::new(),
        });
        app.insert_resource(GlobalSeed {
            value: 897991921,
        });
        app.insert_resource(ChunkGrid {
            chunks: HashMap::new(),
            pending_chunks: HashSet::new(),
        });
        app.insert_resource(PlayerChunk {
            position: IVec2::new(0, 0),
        });
        app.insert_resource( SavedChunks {
            chunks: HashMap::new(),
        });
        app.add_systems(Startup, (setup_atlas).chain());
        app.add_systems(Update, (track_chunks, chunk_handler, 
            prepare_chunk, poll_pending_chunks, spawn_chunk, despawn_chunk, save_chunk, poll_saving_chunks));
        app.add_systems(Update, update_map);
    }
}


#[derive(Resource)]
pub struct PlayerChunk {
    pub position: IVec2,
}


#[derive(Component)]
pub struct ParrentChunk {
    pub position: IVec2,
}


#[derive(Resource)]
pub struct GlobalSeed {
    pub value: u64,
}


#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Chunk {
    pub map: Vec<TileType>,
    pub position: IVec2,
    pub changed: bool,
}


#[derive(Resource)]
pub struct ChunkGrid {
    pub chunks: HashMap<IVec2, Chunk>,
    pub pending_chunks: HashSet<IVec2>,
}


#[derive(Resource)]
pub struct SavedChunks {
    pub chunks: HashMap<IVec2, ChunkSpawnData>,
}

#[derive(Resource)]
pub struct MapAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}


#[derive(Resource)]
pub struct GameMap {
    tiles: Vec<TileType>,
}


#[derive(Resource)]
pub struct MapAtlases {
    pub atlases: HashMap<TileMaterial, MapAtlas>,
}


pub const CHUNK_HEIGHT: usize = 16;
pub const CHUNK_WIDTH: usize = 16;
pub const TILE_SIZE: f32 = 32.0;
pub const MAX_Y: f32 = ((CHUNK_HEIGHT / 2) * TILE_SIZE as usize) as f32;
