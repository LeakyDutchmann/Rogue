mod player_setup;
mod player_movement;
mod player_inventory;
mod player_keyboard;
mod player_combat;

use crate::components::{Speed, MovementIntent, Facing, FacingDirection, ActorState, ActorStateType};
use crate::mouse::CursorWorldPos;
use player_setup::*;
use player_movement::*;
use player_inventory::*;
use player_keyboard::*;
use bevy::prelude::*;
use player_combat::*;
use bevy::prelude::Component;
use crate::animations::{ActiveAnimation, AnimationId, AnimationTimer};
use crate::colision_manager::{Colider, ColiderShape};
use crate::items::{Item, Inventory,CombatStats, AnimationPattern};
use crate::messages::{MouseClickEvent, ItemDropped};
use crate::combat::AttackAnimation;
use super::FieldOfView;


pub struct PlayerSetupPlugin;

impl Plugin for PlayerSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup, setup_inventory));
        app.add_systems(FixedUpdate, move_player);
        app.add_systems(Update, (player_idle_direction, sync_player_inventory,
            pick_active_slot, show_active_slot, drop_item, draw_helditem, update_held_item_dir));
        app.add_systems(Update, initialize_attack);
    }
}







#[derive(Component)]
pub struct Player;


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



