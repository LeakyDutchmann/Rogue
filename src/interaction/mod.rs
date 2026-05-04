mod systems;
mod spawn;

pub use systems::*;
use spawn::*;
use super::*;
use serde::Deserialize;
use crate::messages::{SpawnWindowRequest, CloseWindowRequest, UiSlotUpdate};
use crate::ui::handle_input;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InteractionState {
            interacting: InteractionStage::Idle,
            entity: None,
            interaction_type: InteractionType::None,
            ui_window_id: None,
        });
        app.add_systems(Update, (interact_with_structure));
        app.add_systems(Update, sync_oven_ui);
        app.add_systems(Update, (interact_with_oven_window, ui_slot_update_system).chain());
    }
}


#[derive(Resource)]
pub struct InteractionState {
    pub interacting: InteractionStage,
    pub entity: Option<Entity>,
    pub interaction_type: InteractionType,
    pub ui_window_id: Option<String>,
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


#[derive(Component)]
pub struct OvenInputSlot {
    pub index: usize,
}


#[derive(Component)]
pub struct OvenOutputSlot {
    pub index: usize,
}

