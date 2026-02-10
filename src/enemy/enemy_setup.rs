use super::*;

pub fn setup_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("enemy_spitesheet.png");
    let texture_atlas = TextureAtlasLayout::from_grid (
        UVec2::splat(32), 
        4,
        6,
        None,
        None
    );
    let texture_atlas_layout = texture_atlas_layouts.add(texture_atlas);
    
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }
        ),
        Transform::from_xyz(40.0, 0.0, 1.0),
        
    ));
}