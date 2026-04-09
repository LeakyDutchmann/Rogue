use super::*;

#[derive(Debug, PartialEq, Copy, Clone, Hash)]
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

impl TileMaterial {
    pub fn pick_tile_material(generator: &mut StdRng) -> TileMaterial {
        let number = generator.random_range(0..100);
        match number {
            0..60 => TileMaterial::Structurix,
            60..80 => TileMaterial::Secturix,
            80..100 => TileMaterial::Mechanae,
            _ => TileMaterial::Structurix,
        }
    }
    pub fn get_ore_id(&self) -> Option<String> {
        match &self {
            TileMaterial::Structurix => Some("Structurix".to_string()),
            TileMaterial::Mechanae => Some("Mechanae_ore".to_string()),
            TileMaterial::Secturix => Some("Secturix_ore".to_string()),
            TileMaterial::None => None,
        }
    }
}

#[derive(Component)]
pub struct MapTile {
    pub local_pos: USizeVec2,
    pub tile_type: TileType,
    pub material: TileMaterial,
}


#[derive(Component)]
pub struct Wall;