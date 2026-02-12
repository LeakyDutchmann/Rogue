mod world_systems;


pub use world_systems::*;
use bevy::prelude::*;
use std::collections::HashMap;
use crate::map_setup::{TILE_SIZE, MapTile, MAP_HEIGHT, MAP_WIDTH, map_setup, TileType};






pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldGrid {
            cells: HashMap::new(),
        });
        app.insert_resource(EmptyCells {
            position: Vec::new(),
        });
        app.add_systems(Startup, insert_entities.after(map_setup));
        app.add_systems(Startup, find_empty_space.after(insert_entities));
        // app.add_systems(Update, check_grid.after(apply_movement));
        // app.add_systems(Startup, check_empty.after(find_empty_space));
        
    }
}


#[derive(Resource, PartialEq, Clone,)]
pub struct WorldGrid {
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
}

#[derive(Resource, PartialEq, Clone,)]
pub struct EmptyCells {
    pub position: Vec<(i32, i32)>,
}



pub const CELL_SIZE: f32 = TILE_SIZE as f32;



























