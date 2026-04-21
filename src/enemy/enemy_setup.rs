use super::*;

pub fn generate_position_near(cells: &Vec<Vec2>, target: Vec2) -> Vec2 {
    let mut final_pos = Vec2::from((33.0, 33.0));
    let mut rng = rand::rng();
    let filtered: Vec<_> = cells
        .iter()
        .filter(|&&pos| {
                pos.distance(target) <= 400.0
                })
        .cloned()
        .collect();
    if let Some(cell) = filtered.choose(&mut rng) {
        final_pos = Vec2::from((cell.x, cell.y));
    }
    final_pos
}

pub fn setup_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    empty_cells: Res<EmptyCellsWorldPos>,
) {
    let texture = asset_server.load("characters/enemy_spitesheet.png");
    let texture_atlas = TextureAtlasLayout::from_grid (
        UVec2::splat(32), 
        4,
        7,
        None,
        None
    );
    let texture_atlas_layout = texture_atlas_layouts.add(texture_atlas);
    
    for _ in 0..1 {
        break;
        let pos = generate_position_near(&empty_cells.cells, Vec2::from((0.0, 0.0)));       
        commands.spawn((
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                }
            ),
            Transform::from_xyz(pos.x, pos.y, 1.0),
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
            EnemyAwareness {
                state: AwarenessType::Unaware,
                player_seen: false,
                radius: 200.0,
                awareness_timer: Timer::from_seconds(5.0, TimerMode::Once),
            },
            HurtBox {
                radius: 5.0,
                fraction: FractionType::Enemy,
            }
            
        )).with_children(|parent| {
            parent.spawn((
                HeldItem {
                    held: Some("PickAxe".to_string()),
                    last_held: None,
                },
                Transform::default(),
            ));
        });
    }
}

pub fn update_enemy_state(
    mut text_query: Query<(&mut Text2d, &ChildOf), With<Marker>>,
    health_query: Query<&EnemyAwareness>,
) {
    for (mut text, parent) in text_query.iter_mut() {
        if let Ok(vision) = health_query.get(parent.0) {
            text.0 = match vision.state {
                AwarenessType::Unaware => "Unaware".to_string(),
                AwarenessType::Direct => "Direct".to_string(),
                AwarenessType::Indirect => "Indirect".to_string(),
            }
        }
    }
}


