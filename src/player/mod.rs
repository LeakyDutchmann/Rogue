mod player_setup;
mod player_movement;

use crate::components::*;
use crate::mouse::*;
use player_setup::*;
use player_movement::*;
use bevy::prelude::*;
use bevy::prelude::Component;
use crate::animations::*;
use crate::colision_manager::*;
use crate::items::*;


pub struct PlayerSetupPlugin;

impl Plugin for PlayerSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(FixedUpdate, move_player);
        app.add_systems(Update, player_idle_direction);
    }
}


//resources




//components


#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Player {
    pub state: PlayerState,
    pub facing: Facing,
}



//enums
// 
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PlayerState {
    Idle,
    Walking,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}




