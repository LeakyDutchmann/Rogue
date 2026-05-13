use super::*;

#[derive(Component)]
pub struct WorkBenchSlot {
    pub item: Option<String>,
    pub index: usize,
}


#[derive(PartialEq)]
pub enum UiSlotKind {
    Chest,
    Output,
    Input,
    Inventory,
}


#[derive(Component)]
pub struct UiSlot {
    pub index: usize,
    pub kind: UiSlotKind,
    pub item: Option<String>,
    pub quantity: usize,
}
