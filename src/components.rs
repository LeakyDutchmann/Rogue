use bevy::math::{IVec2, Vec2};
use bevy::prelude::Component;
use crate::map::TileType;

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct MapTile {
    pub position: IVec2,
    pub tile_type: TileType,
}


#[derive(Component)]
pub struct Wall;

