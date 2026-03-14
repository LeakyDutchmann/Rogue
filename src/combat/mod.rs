mod damage_applying;
mod hit_detections;
mod attack_progression;

use bevy::prelude::*;
use hit_detections::*;
pub use damage_applying::*;
use attack_progression::*;

use crate::map_setup::{MapTile, Wall, world_pos_to_tile_pos};
use crate::world::{WorldGrid, CELL_SIZE, get_cells_in_radius, get_entities_in_cells};
use crate::components::{Health, ActorState, ActorStateType, FacingDirection, DeathTimer, Speed,};
use crate::messages::{ApplyDamage, MapChanged, CalculateDamage, DamageType, SpawnItemRequest};
use crate::items::{ItemId, WeaponStats, ToolStats, AnimationStyle, ItemRegistry};
use crate::player::{initialize_attack};
use crate::animations::*;
use crate::colision_manager::Colider;
use crate::{components::MovementIntent, raycasting::EnemyAwareness};


pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (attack_progression_system, hit_detection_system, calculate_damage,
            damage_execution_system, despawn_used_hitboxes).chain().after(initialize_attack));
        app.add_systems(Update, tick_hurt_timers);
        app.add_systems(Update, dead_actor_processing);
        app.add_systems(Update, tick_cooldown);
    }
}


#[derive(Component)]
pub struct AttackAnimation {
    pub anim_pattern: AnimationStyle,
    pub progress: f32,      // 0..1
    pub duration: f32,      // seconds
    pub max_angle: f32,     // radians
    pub hit_triggered: bool,
    pub cursor_pos: Vec2,
    pub item: ItemId,
    pub item_radius: f32,
}


#[derive(PartialEq, Clone)]
pub enum FractionType {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct HitBox {
    pub owner: Entity,
    pub item_used: ItemId,
    pub radius: f32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub aim: Vec2,
    pub fraction: FractionType,
}


#[derive(Component)]
pub struct HurtBox {
    pub radius: f32,
    pub fraction: FractionType,
}


#[derive(Component)]
pub struct HurtTimer {
 pub timer: Timer,
 
}


#[derive(Component)]
pub struct KnockBack {
 pub from: Vec2,
 pub to: Vec2,
}


#[derive(Component)]
pub struct HitBoxUsed;


#[derive(Component)]
pub struct CoolDown {
    pub timer: Timer,
}