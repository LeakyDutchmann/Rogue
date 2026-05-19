use super::*;

pub fn tick_spawner_system(
    time: Res<Time>,
    mut spawner: ResMut<EnemySpawnerTimer>,
    mut console: ResMut<Console>,
    mut writer: MessageWriter<EnemySpawnRequest>,
) {
    spawner.timer.tick(time.delta());
    if spawner.timer.just_finished() {
        console.log(format!("Enemy spawn requested"));
        writer.write(EnemySpawnRequest);
    }
}

pub fn spawn_enemy_system(
    mut commands: Commands,
    mut reader: MessageReader<EnemySpawnRequest>,
    asset_server: Res<AssetServer>,
    player_tf: Res<PlayerTransform>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    empty_cells: Res<EmptyCellsWorldPos>,
) {
    for _ in reader.read() {
        let texture = asset_server.load("characters/enemy_spitesheet.png");
        let texture_atlas = TextureAtlasLayout::from_grid (
            UVec2::splat(32), 
            4,
            7,
            None,
            None
        );
        let texture_atlas_layout = texture_atlas_layouts.add(texture_atlas);
        let pos = generate_position_near(&empty_cells.cells, player_tf.0.translation.xy());       
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
            parent.spawn((
                Text2d::new("Setup".to_string()),
                TextFont {
                    font_size: 5.0,
                    ..Default::default()
                },
                Transform::from_xyz(0.0, -5.0, 1.0),
                Marker,
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
        });
    }
}