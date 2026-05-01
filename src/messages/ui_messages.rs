use super::*;


#[derive(Debug, Clone, PartialEq)]
pub enum WindowType {
    BasicOven,
    BasicWorkBench
}


#[derive(Message)]
pub struct SpawnWindowRequest {
    pub window_type: WindowType,
}


#[derive(Message)]
pub struct CloseWindowRequest;

#[derive(Message)]
pub struct UiSlotUpdate {
    pub entity: Entity,
    pub to_quantity: usize,
    pub to_item: String,
}