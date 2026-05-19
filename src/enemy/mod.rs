mod enemy_setup;
mod ai;
mod pathfinding;
mod spawner;

use super::*;
use enemy_setup::*;
pub use ai::*;
pub use pathfinding::*;
pub use spawner::*;
use crate::messages::EnemySpawnRequest;
use crate::colision_manager::{Colider, ColiderShape};
use crate::components::{Speed, Health, FacingDirection, Facing,
    ActorState, ActorStateType, MovementIntent};
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
        app.add_systems(Update, generate_trial);
        app.add_systems(Update, (update_enemy_state, apply_pathfinding_to_ai));
        app.add_systems(FixedUpdate, ai_movement.after(follow_path));
        app.add_systems(FixedUpdate, follow_path);
        app.add_systems(FixedUpdate, ai_steering.after(ai_movement));
        app.add_systems(Update, ai_initialize_attack);
        app.add_systems(Update, (tick_spawner_system, spawn_enemy_system));
    }
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





