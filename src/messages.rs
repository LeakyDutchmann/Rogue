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
            .add_message::<SlotClicked>();
            
    }
}


#[derive(Message)]
pub struct SlotClicked {
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

#[derive(Message)]
pub enum MouseClickEvent {
    LeftClick(Vec2),
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
