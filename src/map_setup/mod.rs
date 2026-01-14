mod map;
mod cave_generating;

use bevy::prelude::*;
use rand::Rng;
use map::*;
use cave_generating::*;
use crate::components::*;
use crate::mouse::*;
use crate::colision_manager::*;


pub struct MapSetupPlugin;

impl Plugin for MapSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_atlas, floor_setup, map_setup).chain());
        app.add_systems(Update, update_map);
    }
}

//Resources HERE!!!

#[derive(Resource)]
pub struct MapAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}


#[derive(Resource)]
pub struct GameMap {
    tiles: Vec<TileType>,
}


//Acosiated Enums and components here!!


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    Floor, // atlas index = 1..3 **
    WallSideEast,
    WallSideWest,
    WallSideSouth,
    WallSideNorth,
    WallCentre,
    WallCornerSE,
    WallCornerSW,
    WallCornerNE,
    WallCornerNW,
    WallEndEast,
    WallEndWest,
    WallEndNorth,
    WallEndSouth,
    WallEastWest,
    WallNorthSouth,
    WallSingle,
    Empty, //no tile here. Going to use for corridors
}


#[derive(Component)]
pub struct MapTile {
    pub position: IVec2,
    pub tile_type: TileType,
}


//Constants Here!!

pub const MAP_HEIGHT: usize = 50;
pub const MAP_WIDTH: usize = 80;
pub const TILE_SIZE: f32 = 32.0;
