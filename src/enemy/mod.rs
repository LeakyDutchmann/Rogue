mod enemy_setup;
mod ai;
mod pathfinding;
mod spawner;
mod functions;
mod data;

use super::*;
use enemy_setup::*;
use data::*;
use functions::*;
pub use ai::*;
pub use pathfinding::*;
pub use spawner::*;
use crate::messages::EnemySpawnRequest;
use crate::colision_manager::{Colider, ColiderShape};
use crate::components::{Speed, Health, FacingDirection, Facing,
    ActorState, ActorStateType, MovementIntent};
use serde::Deserialize;
use crate::raycasting::{EnemyAwareness, AwarenessType};
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
        app.add_systems(Startup, setup_enemy_registry);
        app.add_systems(Update, generate_trial);
        app.add_systems(Update, (update_enemy_state, apply_pathfinding_to_ai));
        app.add_systems(FixedUpdate, ai_movement.after(follow_path));
        app.add_systems(FixedUpdate, follow_path);
        app.add_systems(FixedUpdate, ai_steering.after(ai_movement));
        app.add_systems(Update, ai_initialize_attack);
        app.add_systems(Update, (tick_spawner_system, spawn_enemy_system));
    }
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





