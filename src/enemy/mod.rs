mod enemy_setup;

use super::*;
use enemy_setup::*;


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemy);
    }
}