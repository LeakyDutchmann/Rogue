use super::*;

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ItemDropped>()
            .add_message::<MouseClickEvent>()
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
            .add_message::<DropFromCursor>();
    }
}


#[derive(Message)]
pub struct DropFromCursor;


pub enum ItemQuantity {
    One,
    MaxFromOne,
    Max,
    HalfStack,
    Сustom(i32),
    Empty
}

impl ItemQuantity {
    pub fn match_quantity(&self, stack_size: i32, item_quantity: i32) -> i32 {
        match self {
            ItemQuantity::One => 1,
            ItemQuantity::MaxFromOne => item_quantity,
            ItemQuantity::Max => if item_quantity == stack_size { stack_size } else { item_quantity },
            ItemQuantity::HalfStack => if item_quantity > stack_size / 2 { stack_size / 2 } else { item_quantity },
            _ => 0,
        }
    }
}


#[derive(Message)]
pub struct InsertToInventory {
    pub item: ItemId,
    pub quantity: i32,
    pub slot: Option<usize>
}


#[derive(Message)]
pub struct GetFromInventory {
    pub quantity: ItemQuantity,
    pub slot: usize
}


pub enum ClickType {
    LeftSingle,
    LeftDouble,
    CtrlLeftSingle,
    ShiftLeftSingle,
}


#[derive(Message)]
pub struct SlotClicked {
    pub click_type: ClickType,
    pub entity: Entity,
    pub slot_index: usize,
}


#[derive(Message)]
pub struct KeyPressed {
    pub key: KeyCode,
}


#[derive(Message)]
pub struct SpawnItemRequest {
    pub position: Vec2,
    pub item_id: ItemId,
}


#[derive(PartialEq, Clone)]
pub enum ScrollDir {
    ScrollUp,
    ScrollDown,
}


#[derive(Message)]
pub struct ItemDropped {
    pub item: Option<Entity>,
}

#[derive(Message, Clone)]
pub enum MouseClickEvent {
    LeftClick(Vec2),
    DoubleLeftClick(Vec2),
    _RightClick(Vec2),
}


#[derive(Message)]
pub struct ScrollMessage {
    pub event: ScrollDir,
}


#[derive(Message)]
pub struct FindPath {
    pub seeker: Entity,
    pub seeker_pos: Vec2,
    pub target_pos: Vec2,
}


#[derive(Message)]
pub struct MapChanged {
    pub position: IVec2,
}


#[derive(Message)]
pub struct ApplyDamage {
    pub entity: Entity,
    pub from_pos: Vec2,
    pub position: Vec2,
    pub damage: i32,
    pub damage_type: DamageType,
}

#[derive(PartialEq, Clone, Copy)]
pub enum DamageType {
    ToTileDamage,
    ToEnemyDamage,
}


#[derive(Message)]
pub struct CalculateDamage {
    pub attack_item: ItemId,
    pub target: Entity,
    pub from_pos: Vec2,
    pub position: Vec2,
    pub damage_type: DamageType,
}
