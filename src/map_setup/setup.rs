use super::*;

pub fn setup_atlas(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut map_atlases: ResMut<MapAtlases>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 8, 8, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);
    map_atlases.atlases.insert(TileMaterial::Structurix, MapAtlas {
        texture: asset_server.load("tiles/Structurix.png"),
        layout: layout_handle.clone(),
    });
    map_atlases.atlases.insert(TileMaterial::Mechanae, MapAtlas {
        texture: asset_server.load("tiles/Mechanae.png"),
        layout: layout_handle.clone(),
    });
    map_atlases.atlases.insert(TileMaterial::Secturix, MapAtlas {
        texture: asset_server.load("tiles/Secturix.png"),
        layout: layout_handle.clone(),
    });
}

pub fn setup_world_dir(
    mut saved: ResMut<SavedChunks>,
) {
    std::fs::create_dir_all("world").unwrap();
    if let Some(chunk_positions) = get_positions_of_saved_chunks() {
        for pos in chunk_positions {
            saved.chunks.insert(pos);
        }
    }
}