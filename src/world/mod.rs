mod world_systems;


pub use world_systems::*;
use bevy::prelude::*;
use std::collections::HashMap;
use crate::map_setup::{TILE_SIZE, MAP_WIDTH, MAP_HEIGHT, Wall};
use crate::map_setup::map_setup;






pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldGrid {
            cells: HashMap::new(),
        });
        app.insert_resource(EmptyCellsWorldPos {
            cells: Vec::new(),
        });
        app.add_systems(Startup, insert_entities.after(map_setup));
        app.add_systems(Startup, find_empty_cells.after(insert_entities));
        // app.add_systems(Startup, modify_empty.after(find_empty_cells));
        // app.add_systems(Update, check_grid.after(apply_movement));
        
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



























