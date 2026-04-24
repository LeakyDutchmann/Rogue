mod systems;
mod spawn;

use systems::*;
use spawn::*;
use super::*;
use serde::Deserialize;
use crate::messages::{SpawnWindowRequest, CloseWindowRequest};

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InteractionState {
            interacting: InteractionStage::Idle,
            entity: None,
            interaction_type: InteractionType::None,
        });
        app.add_systems(Update, (spawn_basic_oven_window));
        app.add_systems(Update, (interact_with_structure, show_structure_window));
        app.add_systems(Update, (close_window));
    }
}


#[derive(Resource)]
pub struct InteractionState {
    pub interacting: InteractionStage,
    pub entity: Option<Entity>,
    pub interaction_type: InteractionType,
} 


#[derive(Component)]
pub struct Interactable;


#[derive(Clone, Deserialize, Debug)]
pub enum InteractionType {
   BasicOven,
   WorkBench,
   None,
}


#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum InteractionStage {
   Idle,
   Interacting,
   Intializing,
}

#[derive(Component)]
pub struct UiStructureWindow;
