mod systems;
mod setup;

use super::*;

use systems::*;
use setup::*;
use crate::inventory::UiBackground;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiHoveringState {
            entity: None 
        });
        app.add_systems(Startup, spawn_tool_tip);
        app.add_systems(Update, hover_system);
        app.add_systems(Update, (tool_tip_follow_cursor, update_tool_tip).chain().after(hover_system));
    }
}

#[derive(Resource)]
pub struct UiHoveringState {
    pub entity: Option<Entity>,
}


#[derive(Component)]
pub struct ToolTip;