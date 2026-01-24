mod world_systems;


pub use world_systems::*;
use bevy::prelude::*;
use std::collections::HashMap;
use crate::map_setup::*;
use crate::map_setup::map_setup;






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
    // Using a 2d array would be faster and more memory efficient. But with
    // HashMap, you are kinda not limited by fixed size (you still are, but
    // by your RAM, which you can eat uncontrollably here)
    //
    // Perhaps you don't want to maintain "one entity per tile" like classic
    // turn-based roguelikes do. Then you may want to look at
    // https://en.wikipedia.org/wiki/Quadtree, e.g.
    // https://github.com/alexpyattaev/spatialtree or
    // https://crates.io/crates/kdtree
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
}

//components


//enums



//constants 

pub const CELL_SIZE: f32 = TILE_SIZE as f32;



























