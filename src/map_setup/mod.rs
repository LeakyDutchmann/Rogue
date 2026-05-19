mod setup;
mod generation;
mod components;
mod systems;
mod functions;
mod spawning;
mod tasks;
mod persistence;
mod map_manager;
mod types;



pub use setup::*;
pub use components::*;
use tasks::*;
pub use types::*;
use map_manager::*;
use generation::*;
use persistence::*;
pub use systems::*;
pub use functions::*;
use spawning::*;

use bevy::prelude::*;
use rand::Rng;
use bevy::math::IVec2;
use std::fs;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use rand::rngs::StdRng;
use rand::SeedableRng;
use noise::{NoiseFn, Perlin};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use bevy::math::USizeVec2;

use crate::colision_manager::{Colider, ColiderShape};
use crate::messages::{MapChanged, PrepareChunk, DisableChunk, SaveChunk,
    LoadChunk, UpdateTile, SpawnStructureRequest, RebuildGrid};
use crate::player::PlayerTransform;
use crate::structures::StructureId;
use crate::world::{CELL_SIZE, WorldGrid};
use crate::components::Health;


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
            chunks: HashSet::new(),
            saving_chunks: HashSet::new(),
        });
        app.add_systems(Startup, (setup_atlas, setup_world_dir).chain());
        app.add_systems(Update, (track_chunks, chunk_handler, 
            prepare_chunk, chunk_loader, poll_pending_chunks, poll_chunk_loading,
            spawn_chunk, update_map, update_tiles, save_chunk, poll_saving_chunks, despawn_chunk).chain());
        app.add_systems(Update, track_of_saved_chunks);
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
    pub chunks: HashSet<IVec2>,
    pub saving_chunks: HashSet<IVec2>,
}


#[derive(Resource)]
pub struct MapAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}


#[derive(Resource)]
pub struct MapAtlases {
    pub atlases: HashMap<TileMaterial, MapAtlas>,
}


pub const CHUNK_HEIGHT: usize = 16;
pub const CHUNK_WIDTH: usize = 16;
pub const TILE_SIZE: f32 = 32.0;