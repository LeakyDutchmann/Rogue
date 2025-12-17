use bevy::math::Vec2;
use bevy::prelude::Component;
use crate::map::TileType;

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct MapTile {
    pub x: usize,
    pub y: usize,
    pub tile_type: TileType,
}

