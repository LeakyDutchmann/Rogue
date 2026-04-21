mod systems;

use systems::*;
use super::*;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, interact_with_structure);
    }
}


#[derive(Component)]
pub struct Interactable;