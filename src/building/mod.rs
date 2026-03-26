mod systems;
mod setup;

use super::*;
use systems::*;
use setup::*;

use crate::inventory::InventoryOpen;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildingMode {
            state: false,
        });
        app.add_systems(Update, (toggle_building_mode, set_building_ui_visibility));
        app.add_systems(Startup, setup_building_mode_ui);
    }
}


#[derive(Resource)]
pub struct BuildingMode {
    state: bool,
}


#[derive(Component)]
pub struct BuildingUiNode;
