use bevy::math::{IVec2, Vec2};
use bevy::prelude::Component;


#[derive(Component)]
pub struct Speed(pub f32);


#[derive(Component)]
pub struct Wall;


#[derive(Component)]
pub struct MovementIntent {
    pub direction: Vec2,
}


#[derive(Component)]
pub struct MovementResolved{
    pub direction: Vec2,
}



