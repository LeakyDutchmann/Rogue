use super::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    Floor, // atlas index = 1..3 **
    WallSideEast,
    WallSideWest,
    WallSideSouth,
    WallSideNorth,
    WallCentre,
    WallCornerSE,
    WallCornerSW,
    WallCornerNE,
    WallCornerNW,
    WallEndEast,
    WallEndWest,
    WallEndNorth,
    WallEndSouth,
    WallEastWest,
    WallNorthSouth,
    WallSingle,
    Empty, //no tile here. Going to use for corridors
}

impl TileType {
    pub fn tile_type_to_index(self) -> usize {
        let sprite_index = match self {
            TileType::Floor => 0, // First tile in spritesheet, takes first 3 spirtes.
            TileType::Empty => 19,
            TileType::WallSideEast => 4,
            TileType::WallSideWest => 3,
            TileType::WallSideSouth => 5,
            TileType::WallSideNorth => 6,
            TileType::WallCentre => 7,
            TileType::WallCornerSE => 8,
            TileType::WallCornerSW => 9,
            TileType::WallCornerNE => 10,
            TileType::WallCornerNW => 11,
            TileType::WallEndEast => 13,
            TileType::WallEndWest => 12,
            TileType::WallEndNorth => 14,
            TileType::WallEndSouth => 15,
            TileType::WallSingle => 16,
            TileType::WallNorthSouth => 18,
            TileType::WallEastWest => 17,
        };
        sprite_index
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum TileMaterial {
    Structurix,
    Mechanae,
    Secturix,
    None,
}


#[derive(Component)]
pub struct MapTile {
    pub position: IVec2,
    pub tile_type: TileType,
    pub material: TileMaterial,
}


#[derive(Component)]
pub struct Wall;