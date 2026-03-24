use super::*;

#[derive(Component)]
pub struct UiBackground;


#[derive(Component)]
pub struct SlotCounter;


#[derive(Component)]
pub struct Slot {
    pub index: usize,
}


#[derive(Component)]
pub struct CursorCarrier {
    pub item: Option<ItemId>,
    pub quantity: i32,
}

impl CursorCarrier {
    pub fn clear(&mut self) {
        self.item = None;
        self.quantity = 0;
    }
}


#[derive(Component)]
pub struct SlotIcon {
    pub index: usize,
}


#[derive(Component)]
pub struct ActiveSlot {
    pub index: i32,
}


#[derive(Clone)]
pub struct ItemStack {
    pub item_stored: Option<ItemId>,
    pub quantity: i32,
}


#[derive(Component)]
pub struct Inventory {
    pub capacity: usize,
    pub items: Vec<ItemStack>,
}


#[derive(Component)]
pub struct InventoryGrid;


#[derive(Component)]
pub struct HotBar;