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

