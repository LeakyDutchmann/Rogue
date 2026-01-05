use crate::player::*;


pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //PLAYER SETUP
    let texture = asset_server.load("player_spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32), 
        4,
        6,
        None,
        None
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0
            }
        ),
        Player {
            state: PlayerState::Idle,
            facing: Facing::Right,
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Speed(125.0),
    ));
}
