use bevy::math::{Vec2};
use bevy::prelude::Component;


#[derive(Component)]
pub struct Speed(pub f32);


#[derive(Component)]
pub struct Health(pub i32);


#[derive(Component)]
pub struct MovementIntent {
    pub direction: Vec2,
}


#[derive(Component)]
pub struct MovementResolved{
    pub direction: Vec2,
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct FacingDirection {
    pub facing: Facing,
}

#[derive(PartialEq)]
pub enum ActorStateType {
    Idle,
    Walking,
}

#[derive(Component)]
pub struct ActorState {
    pub state: ActorStateType,
}


