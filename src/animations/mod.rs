mod animations_setup; 
mod components;


use animations_setup::*;
pub use components::*;
use crate::components::{Facing, ActorState, ActorStateType, FacingDirection};
use std::collections::HashMap;
use crate::items::{AnimationStyle, HeldItem};
use crate::combat::{AttackAnimation, damage_execution_system};
use crate::map_setup::MAX_Y;

use bevy::prelude::*;


pub struct AnimationSetupPlugin;

impl Plugin for AnimationSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationSet {
            indices: HashMap::from([
                (AnimationId::IdleRight, (0, 1)),
                (AnimationId::IdleLeft, (4, 5)),
                (AnimationId::WalkRight, (12, 15)),
                (AnimationId::WalkLeft, (8, 11)),
                (AnimationId::WalkUp, (20, 23)),
                (AnimationId::WalkDown, (16, 19)),
                (AnimationId::HurtRight, (24, 25)),
                (AnimationId::HurtLeft, (26, 27)),
                
            ])
        });
        app.add_systems(Update, (update_animation, reset_animation_index,
            animate_sprite, attack_animation, auto_zorder).chain().after(damage_execution_system));
        app.add_systems(Update, (draw_helditem, update_held_item_dir).chain());
    }
}




#[derive(Resource)]
pub struct AnimationSet {
    pub indices: HashMap<AnimationId, (usize, usize)>
}











