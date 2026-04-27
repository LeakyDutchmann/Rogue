mod systems;
mod setup;
mod input;

use super::*;

use systems::*;
use setup::*;
use input::*;
use crate::inventory::UiBackground;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiHoveringState {
            entity: None,
            last_time: 0.0,
        });
        app.add_systems(Startup, spawn_tool_tip);
        app.add_systems(Update, hover_system);
        app.add_systems(Update, (tool_tip_follow_cursor, update_tool_tip).chain().after(hover_system));
        app.add_systems(Update, handle_input);
    }
}

#[derive(Resource)]
pub struct UiHoveringState {
    pub entity: Option<Entity>,
    pub last_time: f64,
}


#[derive(Component)]
pub struct ToolTip;