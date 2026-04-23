mod systems;
mod setup;

use systems::*;
use setup::*;
use super::*;
use serde::Deserialize;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InteractionState {
            interacting: InteractionStage::Idle,
            entity: None,
            interaction_type: InteractionType::None,
        });
        app.add_systems(Startup, setup_interfaces);
        app.add_systems(Update, (interact_with_structure, show_structure_window));
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
   Syncing,
   Interacting,
   Intializing,
}


#[derive(Component)]
pub struct BasicOvenWindow;


#[derive(Component)]
pub struct UiStructureMenu;
