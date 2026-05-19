mod world_systems;
mod functions;


pub use world_systems::*;
use bevy::prelude::*;
pub use functions::*;
use crate::messages::{MapChanged, RebuildGrid};
use std::collections::HashMap;
use super::*;
use std::sync::{Arc, RwLock};
use crate::map_setup::{TILE_SIZE, Wall, ChunkGrid};
use crate::enemy::Position;
use std::collections::HashSet;


pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldGrid {
            cells: HashMap::new(),
        });
        app.insert_resource(EmptyCellsWorldPos {
            cells: Vec::new(),
        });
        app.insert_resource(SharedBounds(Arc::new(RwLock::new(HashSet::new()))));
        // app.add_systems(Startup, insert_entities.after(map_setup));
        // app.add_systems(Startup, find_empty_cells.after(insert_entities));
        // app.add_systems(Startup, setup_bounds.after(find_empty_cells));
        // app.add_systems(Update, modify_grid);
        app.add_systems(Update, (update_empty_cells, update_worldgird, find_empty_cells));
        app.add_systems(Update, update_bounds.after(update_empty_cells));
        // app.add_systems(FixedUpdate, modify_grid);
        // app.add_systems(Update, check_grid);

        
    }
}


#[derive(Resource, PartialEq, Clone,)]
pub struct WorldGrid {
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
}


#[derive(Resource, PartialEq, Clone,)]
pub struct EmptyCellsWorldPos {
    pub cells: Vec<Vec2>,
}


pub const CELL_SIZE: f32 = TILE_SIZE as f32;


#[derive(Resource)]
pub struct SharedBounds(pub Arc<RwLock<HashSet<Position>>>);


























