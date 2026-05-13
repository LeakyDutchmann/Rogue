use super::*;

#[derive(PartialEq, Clone, Debug)]
pub enum ClickKind {
    LMB,
    RMB,
}


#[derive(Message)]
pub struct UiClick {
    pub entity: Entity,
    pub kind: ClickKind,
    pub double: bool,
    pub shift_pressed: bool,
    pub ctrl_pressed: bool,
}


#[derive(Message)]
pub struct KeyPressed {
    pub key: KeyCode,
}


#[derive(Message, Clone)]
pub enum MouseClickEvent {
    LeftClick(Vec2),
    RightClick(Vec2),
}


#[derive(Message)]
pub struct ScrollMessage {
    pub delta: Vec2,
}