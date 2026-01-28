use super::*;

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ItemDropped>()
            .add_message::<HitMessage>()
            .add_message::<MouseClickEvent>()
            .add_message::<ScrollMessage>()
            .add_message::<MapChanged>()
            .add_message::<ApplyDamage>();
    }
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
pub struct HitMessage {
    pub item: Option<Entity>,
    pub target: Option<Vec2>,
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
pub struct MapChanged {
    pub position: IVec2,
}


#[derive(Message)]
pub struct ApplyDamage {
    pub entity: Entity,
    pub position: IVec2,
    pub damage: i32,
}