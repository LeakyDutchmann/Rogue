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
        Enemy,
        FacingDirection {
            facing: Facing::Right,
        },
        ActorState {
            state: ActorStateType::Idle,
        },
        ActiveAnimation {
            current: AnimationId::IdleRight,
            previous: AnimationId::IdleRight,
        },
        Health(100),
        Speed(100.0),
        Colider {
            shape: ColiderShape::Circle { radius: 3.0},
            _offsety: -5.0,
            _sensor: true,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).with_children(|parent| {
        parent.spawn((
            Text2d::new(100.to_string()),
            TextFont {
                font_size: 10.0,
                ..Default::default()
            },
            Marker
        ));
    });
}

pub fn update_hp_on_marker(
    mut text_query: Query<&mut Text2d, With<Marker>>,
    health_query: Query<&Health>,
) {
    for (mut text, health) in text_query.iter_mut().zip(health_query.iter()) {
        text.0 = health.0.to_string();
    }
}