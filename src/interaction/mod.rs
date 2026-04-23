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
            interacting: false,
            object: None,
        });
        app.add_systems(Startup, setup_interfaces);
        app.add_systems(Update, interact_with_structure);
    }
}


#[derive(Resource)]
pub struct InteractionState {
    pub interacting: bool,
    pub object: Option<Entity>,
} 


#[derive(Component)]
pub struct Interactable;


#[derive(Clone, Deserialize)]
pub enum InteractionType {
   Oven,
   WorkBench,
   None,
}