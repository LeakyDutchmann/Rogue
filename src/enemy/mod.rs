mod enemy_setup;
mod ai;
mod pathfinding;

use super::*;
use enemy_setup::*;
pub use ai::*;
pub use pathfinding::*;
use crate::colision_manager::{Colider, ColiderShape};
use crate::components::{Speed, Health, FacingDirection, Facing,
    ActorState, ActorStateType, MovementIntent};
use crate::raycasting::{EnemyAwareness, AwarenessType};
use std::collections::{HashMap, HashSet, VecDeque};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use std::sync::{Arc, RwLock};


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemy.after(find_empty_cells));
        app.add_systems(Update, generate_trial.after(setup_enemy));
        app.add_systems(Update, (update_enemy_state, apply_pathfinding_to_ai));
        app.add_systems(FixedUpdate, ai_movement.after(follow_path));
        app.add_systems(FixedUpdate, follow_path);
        app.add_systems(FixedUpdate, ai_steering.after(ai_movement));
    }
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





