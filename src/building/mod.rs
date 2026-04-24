mod systems;
mod setup;
mod input;

use super::*;
use systems::*;
use setup::*;
pub use input::*;

use crate::inventory::{UiClickTrack, check_if_inventory_has_item};
use crate::structures::{load_structures,};
use bevy::ui::FocusPolicy;
use crate::crafting::{RecipeRegistry};

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildingMode {
            state: BuildingState::Off,
        });
        app.insert_resource(CanPlaceStruct {
            state: false,
        });
        app.add_systems(Update, (toggle_building_mode, set_building_ui_visibility));
        app.add_systems(Startup, (setup_building_mode_ui, setup_building_ui_nodes).chain().after(load_structures));
        app.add_systems(Update, builder_ui_interactions);
        app.add_systems(Update, (cursor_structure_carrier_update, can_place_structure, build_structure)
            .chain()
            .after(builder_ui_interactions)
        );
    }
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