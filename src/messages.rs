use super::*;

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
            .add_message::<SpawnChunk>()
            .add_message::<DisableChunk>();
    }
}

#[derive(Message)]
pub struct SpawnChunk {
    pub position: IVec2,
}


#[derive(Message)]
pub struct DisableChunk {
    pub position: IVec2,
}


#[derive(Message)]
pub struct RemoveFromInventory {
    pub quantity: i32,
    pub item: String
}


#[derive(Message)]
pub struct SpawnStructureRequest {
    pub position: Vec2,
    pub item_id: String,
}


#[derive(Message)]
pub struct DropFromCursor {
    pub direction: Vec2,
}


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
pub struct DoubleClicked {
    pub slot_index: usize,
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


pub enum ClickType {
    LeftSingle,
    CtrlLeftSingle,
    ShiftLeftSingle,
}


#[derive(Message)]
pub struct SlotClicked {
    pub click_type: ClickType,
    pub slot_index: usize,
}


#[derive(Message)]
pub struct KeyPressed {
    pub key: KeyCode,
}


#[derive(Message)]
pub struct SpawnItemRequest {
    pub position: Vec2,
    pub item_id: String,
}


#[derive(PartialEq, Clone)]
pub enum ScrollDir {
    ScrollUp,
    ScrollDown,
}


#[derive(Message, Clone)]
pub enum MouseClickEvent {
    LeftClick(Vec2),
    RightClick(Vec2),
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
    pub local_pos: IVec2,
    pub chunk_pos: IVec2,
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
    ToStructureDamage,
}


#[derive(Message)]
pub struct CalculateDamage {
    pub attack_item: String,
    pub target: Entity,
    pub from_pos: Vec2,
    pub position: Vec2,
    pub damage_type: DamageType,
}
