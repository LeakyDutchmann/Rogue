use super::*;

#[derive(Component)]
pub struct MapTile {
    pub local_pos: USizeVec2,
    pub tile_type: TileType,
    pub material: TileMaterial,
}


#[derive(Component)]
pub struct Wall;


#[derive(Component)]
pub struct Floor;


#[derive(Component)]
pub struct PendingTaskChunk {
    pub task: Task<ChunkSpawnData>
}


#[derive(Component)]
pub struct PendingChunk {
    pub chunk: ChunkSpawnData,
}


#[derive(Component)]
pub struct SavingPendingChunk {
    pub pos: IVec2,
    pub task: Task<()>
}


#[derive(Component)]
pub struct LoadingPendingChunk {
    pub chunk: Task<Option<ChunkSpawnData>>,
}