use super::*;

mod input;
mod components;
mod sync;
mod systems;
mod ui;
mod setup;
mod functions;

use input::*;
pub use components::*;
use sync::*;
use systems::*;
use ui::*;
use setup::*;
pub use functions::*;

use crate::items::{HeldItem, ItemRegistry};
use crate::messages::{GetFromInventory, KeyPressed,
    SpawnItemRequest, ScrollMessage, InsertToInventory,
    DropFromCursor, ItemQuantity, RemoveFromInventory, QuickMoveFromContainer};
use bevy::ui::FocusPolicy;
use crate::interaction::{InteractionState, InteractionType};
use crate::player::player_setup::player_setup;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InventoryOpen(false));
        app.insert_resource(UiClickTrack {
            last: 0.0,
        });
        app.add_systems(Startup, (setup_inventory, insert_item_in_inventory.after(setup_inventory)).after(player_setup));
        app.add_systems(Update, (toggle_inventory, pick_active_slot_scroll, pick_active_slot));
        app.add_systems(Update, (background_interactions, drop_item, drop_cursor_item, ui_slot_click_handler, quick_move_from_container, double_click_handler).chain());
        app.add_systems(Update, (sync_player_inventory, sync_player_held_item));
        app.add_systems(Update, (show_active_slot, show_inventory, ui_cursor_update, update_item_count));
        app.add_systems(Update, remove_from_inventory);
    }
}


#[derive(Resource)]
pub struct UiClickTrack {
    pub last: f64
}


#[derive(Resource)]
pub struct InventoryOpen(pub bool);
