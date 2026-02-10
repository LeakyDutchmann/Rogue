mod enemy_setup;

use super::*;
use enemy_setup::*;
use crate::colision_manager::{Colider, ColiderShape};
use crate::components::{Speed, Health, FacingDirection, Facing, ActorState, ActorStateType};


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemy);
    }
}



#[derive(Component)]
pub struct Enemy;