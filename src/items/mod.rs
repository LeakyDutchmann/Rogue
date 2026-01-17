mod items_systems;

use bevy::prelude::*;
use items_systems::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pick_up_item);
    }
}




//components

#[derive(Component)]
pub struct Item {
    pub image: Handle<Image>,
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





