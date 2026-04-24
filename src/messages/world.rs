use super::*;


#[derive(Message)]
pub struct SpawnStructureRequest {
    pub position: Vec2,
    pub item_id: String,
    pub chunk_position: IVec2,
}


#[derive(Message)]
pub struct SpawnItemRequest {
    pub position: Vec2,
    pub item_id: String,
}