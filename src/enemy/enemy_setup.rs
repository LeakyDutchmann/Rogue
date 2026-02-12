use super::*;

pub fn setup_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    empty_cells: Res<EmptyCells>,
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
    
    let mut pos = Vec2::ZERO;
    let cells = get_cells_in_radius((0, 0), 300.0);
    for cell in cells {
        if let Some(cell) = empty_cells.position.iter().next() {
            pos = Vec2::from((cell.0 as f32, cell.1 as f32));
        }
    }
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
                    font_size: 6.0,
                    ..Default::default()
                },
                Transform::from_xyz(pos.x, pos.y, 1.0),
                Marker
            ));
        });
}

pub fn update_hp_on_marker(
    mut text_query: Query<(&mut Text2d, &ChildOf), With<Marker>>,
    health_query: Query<&Health>,
) {
    for (mut text, parent) in text_query.iter_mut() {
        if let Ok(health) = health_query.get(parent.0) {
            text.0 = health.0.to_string();
        }
    }
}

