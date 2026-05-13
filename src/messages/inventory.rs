use super::*;


#[derive(Message)]
pub struct RemoveFromInventory {
    pub quantity: i32,
    pub item: String
}


#[derive(Message)]
pub struct DropFromCursor {
    pub direction: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub enum ContainerType {
    Inventory,
    Chest{entity: Entity},
    Input{entity: Entity},
    Output{entity: Entity}
}

#[derive(Message)]
pub struct QuickMoveFromContainer {
    pub container: ContainerType,
    pub index: usize,
}
