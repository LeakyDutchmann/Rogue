mod map;
mod cave_generating;
mod components;
mod systems;
mod functions;

use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
pub use map::*;
pub use components::*;
use cave_generating::*;
use crate::components::Health;
use crate::colision_manager::{Colider, ColiderShape};
use crate::messages::{MapChanged, SpawnChunk};
use crate::player::PlayerTransform;
use systems::*;
use functions::*;



pub struct MapSetupPlugin;

impl Plugin for MapSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapAtlases {
            atlases: HashMap::new(),
        });
        app.insert_resource(GlobalSeed {
            value: 12343253,
        });
        app.insert_resource(ChunkGrid {
            chunks: HashMap::new(),
        });
        app.insert_resource(PlayerChunk {
            position: IVec2::new(0, 0),
        });
        app.add_systems(Startup, (setup_atlas, map_setup).chain());
        app.add_systems(Update, update_map);
        app.add_systems(Update, (chunk_handler, spawn_chunk, track_chunks));
    }
}


#[derive(Resource)]
pub struct PlayerChunk {
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
}


#[derive(Resource)]
pub struct ChunkGrid {
    pub chunks: HashMap<IVec2, Chunk>,
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


pub const CHUNK_HEIGHT: usize = 4;
pub const CHUNK_WIDTH: usize = 4;
pub const TILE_SIZE: f32 = 32.0;
pub const MAX_Y: f32 = ((CHUNK_HEIGHT / 2) * TILE_SIZE as usize) as f32;
