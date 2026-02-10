mod damage_applying;
mod hit_detections;
mod attack_progression;

use bevy::prelude::*;
use hit_detections::*;
use damage_applying::*;
use attack_progression::*;

use crate::map_setup::{MapTile, Wall, TileType, world_pos_to_tile_pos};
use crate::world::{WorldGrid, CELL_SIZE, get_cells_in_radius, get_entities_in_cells};
use crate::components::Health;
use crate::messages::{HitMessage, ApplyDamage, MapChanged, CalculateDamage, DamageType};
use crate::items::{WeaponStats, ToolStats, AnimationStyle};
use crate::enemy::Enemy;


pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (hit_detection_system, calculate_damage, damage_execution_system, attack_progression_system));
    }
}


#[derive(Component)]
pub struct AttackAnimation {
    pub anim_pattern: AnimationStyle,
    pub progress: f32,      // 0..1
    pub duration: f32,      // seconds
    pub max_angle: f32,     // radians
    pub hit_triggered: bool,
    pub target: Option<Vec2>,
    pub item: Option<Entity>,
    pub item_radius: f32,
    pub item_pos: Vec2,
}


