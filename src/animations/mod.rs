mod animations_setup; 
mod components;


use animations_setup::*;
pub use components::*;
use crate::components::{Facing, ActorState, ActorStateType, FacingDirection};
use std::collections::HashMap;
use crate::player::{Player};
use crate::items::{AnimationStyle};
use crate::combat::{AttackAnimation};
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
            ])
        });
        app.add_systems(Update, animate_sprite);
        app.add_systems(Update, update_animation);
        app.add_systems(Update, reset_animation_index.before(animate_sprite));
        app.add_systems(Update, attack_animation);
        app.add_systems(Update, auto_zorder);
    }
}




#[derive(Resource)]
pub struct AnimationSet {
    pub indices: HashMap<AnimationId, (usize, usize)>
}











