use super::*;

mod combat;
mod input;
mod inventory;
mod pathfinding;
mod world;
mod map;
mod ui_messages;
mod processing;
mod enemy;

pub use map::*;
pub use combat::*;
pub use input::*;
pub use inventory::*;
pub use pathfinding::*;
pub use world::*;
pub use ui_messages::*;
pub use processing::*;
pub use enemy::*;

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MouseClickEvent>()
            .add_message::<ScrollMessage>()
            .add_message::<MapChanged>()
            .add_message::<ApplyDamage>()
            .add_message::<CalculateDamage>()
            .add_message::<FindPath>()
            .add_message::<KeyPressed>()
            .add_message::<SpawnItemRequest>()
            .add_message::<DropFromCursor>()
            .add_message::<SpawnStructureRequest>()
            .add_message::<RemoveFromInventory>()
            .add_message::<PrepareChunk>()
            .add_message::<DisableChunk>()
            .add_message::<SaveChunk>()
            .add_message::<LoadChunk>()
            .add_message::<UpdateTile>()
            .add_message::<CloseWindowRequest>()
            .add_message::<UiClick>()
            .add_message::<UiWindowSpawned>()
            .add_message::<UpdateProcessing>()
            .add_message::<EnemySpawnRequest>()         
            .add_message::<RebuildGrid>()
            .add_message::<QuickMoveFromContainer>();
    }
}