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
pub enum ItemQuantity {
    One,
    MaxFromOne,
    HalfStack,
}


pub struct ItemErr;

impl ItemQuantity {
    pub fn match_quantity(&self, stack_size: i32, item_quantity: i32) -> Result<i32, ItemErr> {
        match self {
            ItemQuantity::One => Ok(1),
            ItemQuantity::MaxFromOne => Ok(item_quantity),
            ItemQuantity::HalfStack => if item_quantity > stack_size / 2 { Ok(stack_size / 2) } else { Ok(item_quantity) },
        }
    }
}


#[derive(Message)]
pub struct InsertToInventory {
    pub quantity: i32,
    pub slot: Option<usize>
}


#[derive(Message)]
pub struct GetFromInventory {
    pub quantity: ItemQuantity,
    pub slot: usize
}
