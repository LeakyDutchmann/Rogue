pub mod player_setup;
mod player_movement;
mod player_keyboard;
mod player_combat;

use crate::components::{Speed, MovementIntent, Facing, FacingDirection, ActorState, ActorStateType};
use crate::mouse::CursorWorldPos;
use player_setup::*;
pub use player_movement::*;
use player_keyboard::*;
use bevy::prelude::*;
pub use player_combat::*;
use bevy::prelude::Component;
use crate::animations::{ActiveAnimation, AnimationId, AnimationTimer};
use crate::colision_manager::{Colider, ColiderShape};
use crate::items::{HeldItem, ItemRegistry};
use crate::messages::{MouseClickEvent, KeyPressed};
use crate::combat::{AttackAnimation, HurtBox, HurtTimer, FractionType};
use super::FieldOfView;
use crate::enemy::{ai_steering};
use crate::inventory::{Inventory, ItemStack, ActiveSlot};


pub struct PlayerSetupPlugin;

impl Plugin for PlayerSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(FixedUpdate, move_player.after(ai_steering));
        app.add_systems(Update, (player_idle_direction, keyboard_input_system, update_player_transform));
        app.add_systems(Update, initialize_attack);
    }
}

#[derive(Resource)]
pub struct PlayerTransform(pub Transform);

#[derive(Component)]
pub struct Player;