mod systems;
mod setup;
mod input;
mod functions;

use super::*;
use serde::Deserialize;
use systems::*;
use setup::*;
use input::*;
use functions::*;

use crate::inventory::{UiClickTrack, check_if_inventory_has_item};
use crate::components::*;
use std::collections::HashMap;
use bevy::ui::FocusPolicy;
use crate::crafting::{RecipeRegistry, RecipeDefinition};
use crate::map_setup::ParrentChunk;
use crate::interaction::Interactable;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildingMode {
            state: BuildingState::Off,
        });
        app.insert_resource( StructureRegistry {
            structures: HashMap::new()
        });
        app.insert_resource(CanPlaceStruct {
            state: false,
        });
        app.add_systems(Update, (toggle_building_mode, set_building_ui_visibility));
        app.add_systems(Startup, (setup_building_mode_ui, load_structures, setup_building_ui_nodes).chain());
        app.add_systems(Update, builder_ui_interactions);
        app.add_systems(Update, (cursor_structure_carrier_update, can_place_structure, build_structure, spawn_structure)
            .chain()
            .after(builder_ui_interactions)
        );
    }
}

#[derive(Component, Debug)]
pub struct StructureId {
    pub id: String,
}


#[derive(Resource)]
pub struct CanPlaceStruct {
    pub state: bool,
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BuildingState {
    Off,
    On,
    Placing,
}

#[derive(Resource)]
pub struct BuildingMode {
    pub state: BuildingState,
}


#[derive(Resource)]
pub struct StructureRegistry {
    pub structures: HashMap<String, StructureDefinition>,
}


#[derive(Component)]
pub struct CursorStructureCarrier {
    pub structure: Option<String>,
}


#[derive(Component)]
pub struct BuildingUiNode;


#[derive(Component)]
pub struct BuildingRootUiNode;


#[derive(Component)]
pub struct BuildingUiSlot {
    pub structure: Option<String>,
}


#[derive(Deserialize)]
pub struct StructureDefinitionRaw {
    pub name: String,
    pub sprite_path: String,
    pub icon_path: String,
    pub interactable: bool,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub radius: Option<f32>,
    pub recipe: Option<RecipeDefinition>,
}


#[derive(Clone)]
pub struct StructureDefinition {
    pub sprite: Handle<Image>,
    pub icon: Handle<Image>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub radius: Option<f32>,
    pub interactable: bool,
}



