mod animations_setup; 


use animations_setup::*;
use std::collections::HashMap;
use crate::player::*;

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
    }
}


#[derive(Resource)]
pub struct AnimationSet {
    pub indices: HashMap<AnimationId, (usize, usize)>
}


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


#[derive(Component, PartialEq)]
pub struct ActiveAnimation{
    pub current: AnimationId,
    pub previous: AnimationId,
}


#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum AnimationId {
    IdleRight,
    IdleLeft,
    WalkRight,
    WalkLeft,
    WalkUp,
    WalkDown,
}





