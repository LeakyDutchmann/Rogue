mod enemy_setup;
mod ai_movement;
mod pathfinding_data;
mod swarm_behavior;
mod spawner;
mod functions;
mod vision;
mod surrounding;
mod data;
mod brain;
mod pathfinding;

use super::*;
use enemy_setup::*;
use data::*;
use functions::*;
use swarm_behavior::*;
use brain::*;
use vision::*;
pub use surrounding::*;
pub use ai_movement::*;
pub use pathfinding_data::*;
pub use pathfinding::*;
pub use spawner::*;
use crate::messages::{EnemySpawnRequest};
use crate::colision_manager::{Colider, ColiderShape};
use crate::components::{Speed, Health, FacingDirection, Facing,
    ActorState, ActorStateType, MovementIntent};
use serde::Deserialize;
use std::collections::{HashMap, HashSet, VecDeque};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use rand::seq::IndexedRandom;
use std::sync::{Arc, RwLock};
use std::collections::BinaryHeap;
use std::cmp::Ordering;


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnerTimer{
            timer: Timer::from_seconds(5.0, TimerMode::Repeating)
        });
        app.insert_resource(EnemyRegistry {
            definitions: HashMap::new(),
        });
        app.insert_resource(SlotsForSurrounding {
            slots: Vec::new(),
        });
        app.insert_resource(SwarmBuffState(false));
        app.add_systems(Startup, setup_enemy_registry);
        app.add_systems(Update, generate_trial);
        app.add_systems(Update, (update_enemy_state, apply_pathfinding_to_ai));
        app.add_systems(Update, ai_brain_system);
        app.add_systems(FixedUpdate, ai_pursuing_system.after(follow_path));
        app.add_systems(Update, find_path_ai_system.after(ai_brain_system));
        app.add_systems(Update, start_surrounding.after(ai_brain_system));
        app.add_systems(FixedUpdate, follow_path);
        app.add_systems(FixedUpdate, ai_steering.after(ai_pursuing_system));
        app.add_systems(FixedUpdate, ai_cosmetics_steering.after(ai_steering));
        app.add_systems(Update, ai_initialize_attack);
        app.add_systems(Update, apply_swarn_buff_system);
        app.add_systems(Update, track_enemies_near_player);
        app.add_systems(Update, (tick_spawner_system, spawn_enemy_system));
        app.add_systems(Update, (calculate_slots_around_player, modify_slots_near).chain());
        app.add_systems(Update, (enemy_vision_system, tick_awareness_timer, awareness_state_system, show_enemy_state));
        // app.add_systems(Update, (start_surrounding, remove_surrounding_marker));
    }
}

//here true means occupied, false unoccupied
#[derive(Resource)]
pub struct SlotsForSurrounding {
    slots: Vec<(i32, i32)>,
}


#[derive(Resource)]
pub struct SwarmBuffState(pub bool);


#[derive(Component)]
pub struct Buffed;


#[derive(Component)]
pub struct BuffVisualMarker;


#[derive(Component)]
pub struct EnemyId {
    pub id: String,
}


#[derive(Hash, Deserialize)]
pub enum ColiderShapeRaw {
    Circle { radius: i32 },
    Rectangle { width: i32, height: i32 },
}


#[derive(Hash, Deserialize)]
pub struct ColiderRaw {
    pub shape: ColiderShapeRaw,
    pub offset: IVec2,
    pub mass: i32,
}


#[derive(Hash)]
pub struct EnemyDefinition {
    pub hp: i32,
    pub sprite_sheet: String,
    pub dead_sprite: String,
    pub swarm_buff: Option<SwarmBuffRaw>,
    pub kind: EnemyKind,
    pub speed: i32,
    pub awareness_range: i32,
    pub pursuit_time: i32,
    pub colider: ColiderRaw,
    pub hurt_radius: i32,
    pub fraction: FractionType,
    pub held_item: Option<String>,
}

#[derive(Resource)]
pub struct EnemyRegistry {
    pub definitions: HashMap<String, EnemyDefinition>,
}


#[derive(Resource)]
pub struct EnemySpawnerTimer {
    pub timer: Timer,
}



#[derive(Component)]
pub struct Enemy;


#[derive(Component)]
pub struct Marker;


#[derive(Component)]
pub struct PathfindingTask(Task<Result<Vec<Position>, PathfindingError>>);


#[derive(Component)]
pub struct AiPath {
    pub steps: VecDeque<Vec2>,
}


#[derive(Copy, Clone, PartialEq)]
pub enum EnemyStateType{
    Idle,
    Patroling,
    Pursuing,
    Surrounding,
    Retreating,
    Pathfinding,
}


#[derive(Component)]
pub struct EnemyState {
    pub current: EnemyStateType,
    pub previous: EnemyStateType,
}

impl EnemyState {
    pub fn set(&mut self, new: EnemyStateType) {
        if new != self.current {
            self.previous = self.current;
            self.current = new;
        }
        
    }
}


pub enum AwarenessType {
    Unaware,
    Direct,
    Indirect,
}

#[derive(Component)]
pub struct EnemyAwareness {
    pub state: AwarenessType,
    pub radius: f32,
    pub awareness_timer: Timer,
}

#[derive(Component)]
pub struct EnemyEyes {
    pub sees_player: bool,
    pub last_seen_pos: Option<Vec2>,
}



