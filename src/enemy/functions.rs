use super::*;

pub fn assemble_enemy(
    commands: &mut Commands,
    assets_server: &AssetServer,
    enemy_name: String,
    enemy_reg: &EnemyRegistry,
    atlas_layouts: &mut Assets<TextureAtlasLayout>,
    pos: Vec2,
) {
    if let Some(def) = enemy_reg.definitions.get(&enemy_name) {
        let texture = assets_server.load(&def.sprite_sheet);
        let atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 7, None, None);
        let texture_atlas_layout = atlas_layouts.add(atlas_layout);
        let entity = commands.spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas { 
                    layout: texture_atlas_layout,
                    index: 0 
                } 
            ),
            Health(def.hp),
            Transform::from_xyz(pos.x, pos.y, -pos.y * 0.001 - 10.0), ////
            Speed(def.speed as f32),
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            HurtBox {
                radius: def.hurt_radius as f32,
                fraction: def.fraction.clone(),
            },
            Enemy,
            EnemyAwareness {
                state: AwarenessType::Unaware,
                player_seen: false,
                radius: def.awareness_range as f32,
                awareness_timer: Timer::from_seconds(def.pursuit_time as f32, TimerMode::Once),
            },
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
        )).id();
        if let Some(held_item) = &def.held_item {
            let child = commands.spawn(
                HeldItem {
                    held: Some(held_item.clone()),
                    last_held: None,
                }
            ).id();
            commands.entity(entity).add_child(child);
        }
        match &def.colider.shape {
            ColiderShapeRaw::Circle { radius } => {
                commands.entity(entity).insert(Colider { 
                    shape: ColiderShape::Circle { radius: *radius as f32 },
                    _offsety: 0.0,
                    _sensor: false,
                });
            }
            ColiderShapeRaw::Rectangle { width, height } => {
                commands.entity(entity).insert(Colider { 
                    shape: ColiderShape::Rectangle { width: *width as f32, height: *height as f32 },
                    _offsety: 0.0,
                    _sensor: false,
                });
            }   
        }
        let debug_child = commands.spawn((
            Text2d::new("Setup".to_string()),
            TextFont {
                font_size: 5.0,
                ..Default::default()
            },
            Transform::from_xyz(0.0, -5.0, 1.0),
            Marker,
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
        )).id();
        commands.entity(entity).add_child(debug_child);
    }
}