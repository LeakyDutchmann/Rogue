use super::*;


#[derive(Message)]
pub struct UpdateTile {
    pub tile_position: Vec2,
    pub tile_type: TileType,
}


#[derive(Message)]
pub struct LoadChunk {
    pub position: IVec2,
}


#[derive(Message)]
pub struct PrepareChunk {
    pub position: IVec2,
}


#[derive(Message)]
pub struct DisableChunk {
    pub position: IVec2,
}


#[derive(Message)]
pub struct SaveChunk {
    pub position: IVec2,
}


#[derive(Message)]
pub struct MapChanged {
    pub pos: Vec2,
}
