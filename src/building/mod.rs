mod systems;
mod setup;

use super::*;
use systems::*;
use setup::*;

use crate::inventory::InventoryOpen;
use std::collections::HashMap;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildingMode {
            state: false,
        });
        app.insert_resource( StructureRegistry {
            structures: HashMap::new()
        });
        app.add_systems(Update, (toggle_building_mode, set_building_ui_visibility));
        app.add_systems(Startup, setup_building_mode_ui);
    }
}


#[derive(Resource)]
pub struct BuildingMode {
    state: bool,
}


#[derive(Resource)]
pub struct StructureRegistry {
    structures: HashMap<String, StructureDefinition>,
}


#[derive(Component)]
pub struct BuildingUiNode;


pub struct StructureDefinition {
    pub name: String,
    pub texture: Handle<Image>,
}


