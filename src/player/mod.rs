mod player_setup;
mod player_movement;

use player_setup::*;
use player_movement::*;
use bevy::prelude::*;


pub struct PlayerSetupPlugin;

impl Plugin for PlayerSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(FixedUpdate, move_player);
    }
}
