mod systems;
mod setup;
mod input;
mod data;
mod functions;
mod ui_windows;
mod components;
mod sync;

use super::*;

use systems::*;
pub use components::*;
use setup::*;
pub use input::*;
use data::*;
pub use functions::*;
pub use ui_windows::*;
use serde::Deserialize;
use std::collections::HashMap;
use bevy::ui::FocusPolicy;
use crate::inventory::UiBackground;
use sync::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiHoveringState {
            entity: None,
            last_time: 0.0,
        });
        app.insert_resource(UiWindowRegistry {
            windows: HashMap::new(),
        });
        app.add_systems(Startup, (load_ui_winows, spawn_tool_tip));
        app.add_systems(Update, hover_system);
        app.add_systems(Update, (sync_oven_ui, sync_work_bench_ui, sync_chest_ui));
        app.add_systems(Update, (tool_tip_follow_cursor, update_tool_tip).chain().after(hover_system));
        app.add_systems(Update, handle_input);
        app.add_systems(Update, (show_structure_window, close_window));
    }
}


#[derive(Resource)] 
pub struct UiWindowRegistry {
    pub windows: HashMap<String,RawNode>, 
}


#[derive(Resource)]
pub struct UiHoveringState {
    pub entity: Option<Entity>,
    pub last_time: f64,
}


#[derive(Component)]
pub struct ToolTip;


#[derive(Component)]
pub struct InputUiSlot;


#[derive(Component)]
pub struct OutputUiSlot;
