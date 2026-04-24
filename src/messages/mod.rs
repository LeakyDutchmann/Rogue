use super::*;

mod combat;
mod input;
mod inventory;
mod pathfinding;
mod world;
mod map;

pub use map::*;
pub use combat::*;
pub use input::*;
pub use inventory::*;
pub use pathfinding::*;
pub use world::*;

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
            .add_message::<SlotClicked>()
            .add_message::<InsertToInventory>()
            .add_message::<GetFromInventory>()
            .add_message::<DropFromCursor>()
            .add_message::<DoubleClicked>()
            .add_message::<SpawnStructureRequest>()
            .add_message::<RemoveFromInventory>()
            .add_message::<PrepareChunk>()
            .add_message::<DisableChunk>()
            .add_message::<SaveChunk>()
            .add_message::<LoadChunk>()
            .add_message::<UpdateTile>();
    }
}