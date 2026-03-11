mod items_systems;

use bevy::prelude::*;
use items_systems::*;
use rand::Rng;

use crate::player::{Player};
use crate::messages::{ItemDropped};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (pick_up_item, update_dropped_items));
    }
}


#[derive(Component)]
pub struct Item {
    pub image: Handle<Image>,
    pub name: String,
}


#[derive(PartialEq, Clone, Copy)]
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


#[derive(Component)]
pub struct Inventory {
    pub capacity: u32,
    pub items: Vec<Option<Entity>>,
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
pub struct _Usable;


#[derive(Component)]
pub struct Durability {
    pub durability: f32,
}

#[derive(Component)]
pub struct HeldItem {
    pub held: Option<Entity>,
    pub last_held: Option<Entity>,
}