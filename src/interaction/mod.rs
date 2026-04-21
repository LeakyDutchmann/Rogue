mod systems;

use systems::*;
use super::*;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        
    }
}


#[derive(Component)]
pub struct Interactable;