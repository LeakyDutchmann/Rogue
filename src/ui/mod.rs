mod systems;

use super::*;

use systems::*;
use crate::inventory::UiBackground;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hover_system);
    }
}