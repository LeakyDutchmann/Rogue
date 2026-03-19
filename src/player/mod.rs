mod player_setup;
mod player_movement;
mod player_inventory;
mod player_keyboard;
mod player_combat;

use crate::components::{Speed, MovementIntent, Facing, FacingDirection, ActorState, ActorStateType};
use crate::mouse::CursorWorldPos;
use player_setup::*;
pub use player_movement::*;
use player_inventory::*;
use player_keyboard::*;
use bevy::prelude::*;
pub use player_combat::*;
use bevy::prelude::Component;
use crate::animations::{ActiveAnimation, AnimationId, AnimationTimer};
use crate::colision_manager::{Colider, ColiderShape};
use crate::items::{Inventory, HeldItem, ItemStack, ItemRegistry, ItemId};
use crate::messages::{MouseClickEvent, GetFromInventory, KeyPressed,
    SpawnItemRequest, ScrollMessage, ScrollDir, SlotClicked, InsertToInventory, DropFromCursor, ClickType};
use crate::combat::{AttackAnimation, HurtBox, HurtTimer, FractionType};
use bevy::window::PrimaryWindow;
use super::FieldOfView;
use crate::enemy::{ai_steering};


pub struct PlayerSetupPlugin;

impl Plugin for PlayerSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InventoryOpen(false));
        app.insert_resource(UiClickTrack {
            last: 0.0,
        });
        app.add_systems(Startup, (player_setup, setup_inventory));
        app.add_systems(FixedUpdate, move_player.after(ai_steering));
        app.add_systems(Update, (player_idle_direction, sync_player_inventory,
            pick_active_slot, show_active_slot, keyboard_input_system, sync_player_held_item,
            drop_item, show_inventory, toggle_inventory, pick_active_slot_scroll, ui_cursor_update));
        app.add_systems(Update, initialize_attack);
        app.add_systems(Update, (inventory_interactions, background_interactions, item_click_handler, item_take_handler, item_put_handler, update_item_count).chain());
    }
}

#[derive(Resource)]
pub struct UiClickTrack {
    pub last: f64
}


#[derive(Resource)]
pub struct InventoryOpen(pub bool);


#[derive(Component)]
pub struct UiBackground;


#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct SlotCounter;


#[derive(Component)]
pub struct Slot {
    pub index: usize,
}


#[derive(Component)]
pub struct CursorCarrier {
    pub item: Option<ItemId>,
    pub quantity: i32,
}


#[derive(Component)]
pub struct SlotIcon {
    pub index: usize,
}


#[derive(Component)]
pub struct ActiveSlot {
    pub index: i32,
}






