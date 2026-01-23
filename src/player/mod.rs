mod player_setup;
mod player_movement;
mod player_inventory;
mod player_keyboard;
mod player_animations;

use crate::components::*;
use crate::mouse::*;
use player_setup::*;
use player_movement::*;
use player_inventory::*;
use player_keyboard::*;
use player_animations::*;
use bevy::prelude::*;
use bevy::prelude::Component;
use crate::animations::*;
use crate::colision_manager::*;
use crate::items::*;


pub struct PlayerSetupPlugin;

impl Plugin for PlayerSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ItemDropped>();
        app.add_systems(Startup, (player_setup, setup_inventory));
        app.add_systems(FixedUpdate, move_player);
        app.add_systems(Update, (player_idle_direction, sync_player_inventory,
            pick_active_slot, show_active_slot, drop_item, draw_helditem, update_held_item_dir, animate_kick, start_kick));
        
    }
}


//resources




//components


#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Player {
    pub state: PlayerState,
    pub facing: Facing,
}


#[derive(Component)]
pub struct Slot {
    pub index: usize,
}


#[derive(Component)]
pub struct SlotIcon {
    pub index: usize,
}


#[derive(Component)]
pub struct ActiveSlot {
    pub index: usize,
}


#[derive(Component)]
pub struct HeldItem {
    pub last_held: Option<Entity>,
}


#[derive(Component)]
struct KickAnimation {
    progress: f32,      // 0..1
    duration: f32,      // seconds
    max_angle: f32,     // radians
    active: bool,
    impact_triggered: bool,
    target: Option<Vec2>,
    item: Option<Entity>,
}


//enums

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

//messages

#[derive(Message)]
pub struct ItemDropped {
    pub item: Option<Entity>,
}
