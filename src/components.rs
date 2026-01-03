use bevy::math::{IVec2, Vec2};
use bevy::prelude::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Wall;

