mod enemy_setup;
mod ai;
mod pathfinding;

use super::*;
use enemy_setup::*;
use ai::*;
use pathfinding::*;
use crate::colision_manager::{Colider, ColiderShape};
use crate::components::{Speed, Health, FacingDirection, Facing,
    ActorState, ActorStateType, MovementIntent};


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemy.after(find_empty_cells));
        app.add_systems(Startup, generate_trial.after(setup_enemy));
        app.add_systems(Update, (update_hp_on_marker, apply_pathfinding_to_ai));
        app.add_systems(FixedUpdate, ai_movement);
    }
}



#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Marker;