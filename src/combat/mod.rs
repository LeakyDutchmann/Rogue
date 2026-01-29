mod damage_applying;
mod hit_detections;

use bevy::prelude::*;
use hit_detections::*;
use damage_applying::*;

use crate::map_setup::{MapTile, Wall, TileType, world_pos_to_tile_pos};
use crate::world::{WorldGrid, CELL_SIZE};
use crate::components::Health;
use crate::messages::{HitMessage, ApplyDamage, MapChanged};


pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (hit_detection_system, damage_execution_system));
    }
}


#[derive(Component)]
pub struct AttackAnimation {
    pub progress: f32,      // 0..1
    pub duration: f32,      // seconds
    pub max_angle: f32,     // radians
    pub hit_triggered: bool,
    pub target: Option<Vec2>,
    pub item: Option<Entity>,
}


