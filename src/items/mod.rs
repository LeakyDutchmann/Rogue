mod items_systems;
mod item_functions;

use bevy::prelude::*;
use items_systems::*;
pub use item_functions::*;
use rand::Rng;

use crate::player::{Player};
use crate::messages::SpawnItemRequest;
use serde::{Deserialize};
use std::collections::HashMap;
use std::fs;
use crate::inventory::Inventory;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemRegistry {
            items: HashMap::new(),
        });
        app.add_systems(Startup, setup_items);
        app.add_systems(Update, (pick_up_near_item, item_spawn_system));
    }
}


#[derive(Deserialize, Debug, Hash, Clone, Copy)]
pub struct CombatStatsRaw {
    pub attack_speed: i32,
    pub swing_angle: i32,
    pub radius: i32,
}


#[derive(Deserialize, Debug, Hash, Clone, Copy)]
pub struct WeaponStatsRaw {
    pub enemy_damage: i32,
}


#[derive(Deserialize, Debug, Hash, Clone, Copy)]
pub struct ToolStatsRaw {
    pub structure_damage: i32,
}


#[derive(Deserialize, Hash, Debug, Clone, PartialEq, Eq, Component, Copy)]
pub enum ItemId {
    Sword,
    PickAxe,
    Inferium,
}


#[derive(Deserialize, Debug)]
pub struct ItemDefinitionRaw {
    pub name: ItemId,
    pub icon: String,
    pub sprite: String,
    pub durability: Option<i32>,
    pub usable: bool,
    pub animation_style: Option<AnimationStyle>,
    pub combat_stats: Option<CombatStatsRaw>,
    pub weapon_stats: Option<WeaponStatsRaw>,
    pub tool_stats: Option<ToolStatsRaw>,
    pub max_stack: usize,
}


#[derive(Hash)]
pub struct ItemDefinition {
    pub icon: Handle<Image>,
    pub sprite: Handle<Image>,
    pub durability: Option<i32>,
    pub usable: bool,
    pub animation_style: Option<AnimationStyle>,
    pub combat_stats: Option<CombatStatsRaw>,
    pub weapon_stats: Option<WeaponStatsRaw>,
    pub tool_stats: Option<ToolStatsRaw>,
    pub max_stack: usize,
}


#[derive(Resource)]
pub struct ItemRegistry {
    pub items: HashMap<ItemId, ItemDefinition>,
}


#[derive(PartialEq, Clone, Copy, Deserialize, Debug, Hash)]
pub enum AnimationStyle {
    PickAxe,
    Sword,
}


#[derive(Component)]
pub struct OnGround;


#[derive(Component)]
pub struct Usable;




#[derive(Component)]
pub struct HeldItem {
    pub held: Option<ItemId>,
    pub last_held: Option<ItemId>,
}