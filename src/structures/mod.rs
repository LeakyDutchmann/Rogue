use super::*;

mod systems;
mod functions;
mod setup;
mod components;
mod processing;

use functions::*;
use systems::*;
pub use setup::*;
pub use components::*;
pub use processing::*;

use crate::components::Health;
use std::collections::HashMap;
use crate::building::builder_ui_interactions;
use crate::messages::UpdateProcessing;
use serde::Deserialize;

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource( StructureRegistry {
            structures: HashMap::new()
        });
        app.insert_resource(OvenRecipeRegistry {
            recipes: HashMap::new(),
        });
        app.add_systems(Startup, load_structures);
        app.add_systems(Update, (tick_oven_timers, update_processing).chain());
        app.add_systems(Update, spawn_structure.after(builder_ui_interactions));
    }
}


#[derive(Debug, Deserialize)]
pub struct OvenRecipe {
    pub input: String,
    pub output: String,
    pub process_time: f32,
}


#[derive(Resource)]
pub struct OvenRecipeRegistry {
    pub recipes: HashMap<String, OvenRecipe>
}


#[derive(Component, Debug)]
pub struct StructureId {
    pub id: String,
}


#[derive(Resource)]
pub struct StructureRegistry {
    pub structures: HashMap<String, StructureDefinition>,
}


#[derive(Deserialize)]
pub struct StructureDefinitionRaw {
    pub name: String,
    pub sprite_path: String,
    pub icon_path: String,
    pub interaction: InteractionType,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub radius: Option<f32>,
    pub recipe: Option<RecipeDefinition>,
    pub ui_window_id: Option<String>,
}


#[derive(Clone)]
pub struct StructureDefinition {
    pub sprite: Handle<Image>,
    pub icon: Handle<Image>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub radius: Option<f32>,
    pub interaction: InteractionType,
    pub ui_window_id: Option<String>,
}