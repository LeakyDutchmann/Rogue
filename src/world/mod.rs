mod world_systems;


pub use world_systems::*;
use bevy::prelude::*;
use std::collections::HashMap;
use crate::map_setup::*;
use crate::player::*;
use crate::map_setup::map_setup;
use crate::movement::*;






pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldGrid {
            cells: HashMap::new(),
        });
        app.add_systems(Startup, insert_entities.after(map_setup));
        // app.add_systems(Update, check_grid.after(apply_movement));
        
    }
}

//resources

#[derive(Resource, PartialEq, Clone,)]
pub struct WorldGrid {
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
}

//components


//enums



//constants 

pub const CELL_SIZE: f32 = TILE_SIZE as f32;



























