mod systems;
mod spawn;
mod functions;

pub use systems::*;
use spawn::*;
use functions::*;
use super::*;
use serde::Deserialize;
use crate::messages::{SpawnWindowRequest, CloseWindowRequest, UiSlotUpdate};
use crate::ui::handle_input;
use crate::ui::WorkBenchSlot;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InteractionState {
            interacting: InteractionStage::Idle,
            entity: None,
            interaction_type: InteractionType::None,
            ui_window_id: None,
        });
        app.add_systems(Update, interact_with_workbench);
        app.add_systems(Update, (interact_with_structure));
        app.add_systems(Update, quick_move_from_container.after(ui_slot_click_handler));
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
   Chest,
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
    pub item: Option<String>,
}


#[derive(Component)]
pub struct OvenOutputSlot {
    pub index: usize,
    pub item: Option<String>,
}

