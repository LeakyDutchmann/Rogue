mod map;
mod cave_generating;
mod components;

use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
pub use map::*;
pub use components::*;
use cave_generating::*;
use crate::components::Health;
use crate::colision_manager::{Colider, ColiderShape};
use crate::messages::MapChanged;



pub struct MapSetupPlugin;

impl Plugin for MapSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapAtlases {
            atlases: HashMap::new(),
        });
        app.add_systems(Startup, (setup_atlas, floor_setup, map_setup).chain());
        app.add_systems(Update, update_map);
    }
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


pub const MAP_HEIGHT: usize = 50;
pub const MAP_WIDTH: usize = 80;
pub const TILE_SIZE: f32 = 32.0;
pub const MAX_Y: f32 = ((MAP_HEIGHT / 2) * TILE_SIZE as usize) as f32;
