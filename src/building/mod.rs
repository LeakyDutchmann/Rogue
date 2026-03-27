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

use crate::inventory::{InventoryOpen, UiClickTrack};
use std::collections::HashMap;
use bevy::ui::FocusPolicy;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildingMode {
            state: BuildingState::Off,
        });
        app.insert_resource( StructureRegistry {
            structures: HashMap::new()
        });
        app.insert_resource(PlacingState {
            structure: None,
        });
        app.add_systems(Update, (toggle_building_mode, set_building_ui_visibility));
        app.add_systems(Startup, (setup_building_mode_ui, load_structures, setup_building_ui_nodes).chain());
        app.add_systems(Update, builder_ui_interactions);
        app.add_systems(Update, (cursor_structure_carrier_update, build_structure, spawn_structure)
            .chain()
            .after(builder_ui_interactions)
        );
    }
}

#[derive(Component)]
pub struct StructureId {
    id: String,
}


#[derive(Resource)]
pub struct PlacingState {
    structure: Option<String>,
}


#[derive(PartialEq, Eq, Clone, Debug)]
enum BuildingState {
    Off,
    On,
    Placing,
}

#[derive(Resource)]
pub struct BuildingMode {
    state: BuildingState,
}


#[derive(Resource)]
pub struct StructureRegistry {
    structures: HashMap<String, StructureDefinition>,
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
}


#[derive(Clone)]
pub struct StructureDefinition {
    pub sprite: Handle<Image>,
    pub icon: Handle<Image>,
}


