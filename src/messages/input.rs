use super::*;


#[derive(Message)]
pub struct UiClick {
    pub entity: Entity,
    pub double: bool,
    pub shift_pressed: bool,
    pub ctrl_pressed: bool,
}


#[derive(Message)]
pub struct DoubleClicked {
    pub slot_index: usize,
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


#[derive(PartialEq, Clone)]
pub enum ScrollDir {
    ScrollUp,
    ScrollDown,
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