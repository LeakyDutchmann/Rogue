mod enemy_setup;
mod ai;

use super::*;
use enemy_setup::*;
use ai::*;
use crate::colision_manager::{Colider, ColiderShape};
use crate::components::{Speed, Health, FacingDirection, Facing,
    ActorState, ActorStateType, MovementIntent};


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemy);
        app.add_systems(Update, update_hp_on_marker);
        app.add_systems(FixedUpdate, ai_movement);
    }
}



#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Marker;