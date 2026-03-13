mod items_systems;

use bevy::prelude::*;
use items_systems::*;
use rand::Rng;

use crate::player::{Player};
use crate::messages::{ItemDropped};
use serde::{Deserialize};
use std::collections::HashMap;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemRegistry {
            items: HashMap::new(),
        });
        app.add_systems(Startup, setup_items);
    }
}


#[derive(Deserialize, Debug, Hash)]
pub struct CombatStatsRaw {
    pub attack_speed: i32,
    pub swing_angle: i32,
    pub radius: i32,
}


#[derive(Deserialize, Debug, Hash)]
pub struct WeaponStatsRaw {
    pub enemy_damage: i32,
}


#[derive(Deserialize, Debug, Hash)]
pub struct ToolStatsRaw {
    pub structure_damage: i32,
}


#[derive(Deserialize, Hash, Debug, Clone, PartialEq, Eq, Component)]
pub enum ItemId {
    Sword,
    PickAxe,
}


#[derive(Deserialize, Debug)]
pub struct ItemDefinitionRaw {
    pub name: ItemId,
    pub icon: String,
    pub sprite: String,
    pub durability: Option<i32>,
    pub usable: bool,
    pub animation_style: AnimationStyle,
    pub combat_stats: Option<CombatStatsRaw>,
    pub weapon_stats: Option<WeaponStatsRaw>,
    pub tool_stats: Option<ToolStatsRaw>,
}


#[derive(Hash)]
pub struct ItemDefinition {
    pub icon: Handle<Image>,
    pub sprite: Handle<Image>,
    pub durability: Option<i32>,
    pub usable: bool,
    pub animation_style: AnimationStyle,
    pub combat_stats: Option<CombatStatsRaw>,
    pub weapon_stats: Option<WeaponStatsRaw>,
    pub tool_stats: Option<ToolStatsRaw>,
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
pub struct AnimationPattern {
    pub pattern: AnimationStyle,
}


#[derive(Component)]
pub struct OnGround;


#[derive(Component)]
pub struct InInventory;

#[derive(Clone)]
pub struct ItemStack {
    pub item_stored: Option<ItemId>,
    pub quantity: u32,
}


#[derive(Component)]
pub struct Inventory {
    pub capacity: usize,
    pub items: Vec<ItemStack>,
}


#[derive(Component)]
pub struct IntentPickingUp {
    pub target: Entity,
}

#[derive(Component)]
pub struct CombatStats {
    pub attack_speed: f32,
    pub swing_angle: f32,
    pub radius: f32,
}

#[derive(Component)]
pub struct WeaponStats {
    pub enemy_damage: i32,
}


#[derive(Component)]
pub struct ToolStats {
    pub structure_damage: i32,
}


#[derive(Component)]
pub struct Usable;


#[derive(Component)]
pub struct Durability {
    pub durability: i32,
}

#[derive(Component)]
pub struct HeldItem {
    pub held: Option<Entity>,
    pub last_held: Option<Entity>,
}