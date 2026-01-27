mod items_systems;

use bevy::prelude::*;
use items_systems::*;
use rand::Rng;

use crate::player::*;
use crate::messages::{ItemDropped};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (pick_up_item, update_dropped_items, sync_player_inventory));
    }
}




//components

#[derive(Component)]
pub struct Item {
    pub image: Handle<Image>,
    pub name: String,
}


#[derive(Component)]
pub struct OnGround;


#[derive(Component)]
pub struct InInventory;


#[derive(Component)]
pub struct Inventory {
    pub capacity: u32,
    pub items: Vec<Option<Entity>>,
}

#[derive(Component)]
pub struct IntentPickingUp {
    pub target: Entity,
}


//resources


//enums





