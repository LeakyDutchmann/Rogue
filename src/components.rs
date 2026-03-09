use bevy::math::{Vec2};
use bevy::prelude::*;


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

impl Facing {
    pub fn from_direction(direction: Vec2) -> Facing {
        if direction.x.abs() >= direction.y.abs() {
                if direction.x > 0.0 { Facing::Right } else { Facing::Left }
            } else {
                if direction.y > 0.0 { Facing::Up } else { Facing::Down }
            } 
    }
}

#[derive(Component)]
pub struct FacingDirection {
    pub facing: Facing,
}

#[derive(PartialEq)]
pub enum ActorStateType {
    Idle,
    Walking,
    Hurt,
    Dead,
}

#[derive(Component)]
pub struct ActorState {
    pub state: ActorStateType,
}


#[derive(Component)]
pub struct DeathTimer {
    pub timer: Timer,
}



