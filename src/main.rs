mod movement;
mod components;
mod map_setup;
mod mouse_input;

use bevy::{prelude::*, sprite};
use bevy::time::Fixed;
use std::collections::HashMap;
use components::*;
use mouse_input::*;
use movement::*;
use map_setup::*;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, PartialEq)]
pub struct ActiveAnimation {
    pub current: AnimationType,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy,)]
pub enum AnimationType {
    IdleLeft,
    IdleRight,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}


#[derive(Resource)]
pub struct SpriteSheetIndices {
    pub indices: HashMap<AnimationType, (usize, usize)>,
}

// #[derive(Resource)]
// pub struct MyTimer(pub Timer);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        .add_plugins(MapSetupPlugin)
        //.insert_resource(MyTimer(Timer::from_seconds(0.1, TimerMode::Once)))
        .insert_resource(CursorWorldPos(None))
        .insert_resource(SpriteSheetIndices { indices: HashMap::new() })
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_message::<MouseClickEvent>()
        .add_message::<ApplyDestruction>()
        .add_message::<MapChanged>()
        .add_message::<ScrollMessage>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_player, camera_update))
        .add_systems(Update, (get_cursor_position, mouse_click_handler, mouse_events, destruction_system))
        .add_systems(Update, animate_sprite)
        .add_systems(Update, (match_animation_type, scroll_events, camera_scroll_in))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut sprites: ResMut<SpriteSheetIndices>,
) {
    //CAMERA SETUP

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.4,
            ..OrthographicProjection::default_2d()
        }),
    ));
    //animation_setup
    sprites.indices.insert(AnimationType::IdleRight, (0, 1));
    sprites.indices.insert(AnimationType::IdleLeft, (8, 10));
    sprites.indices.insert(AnimationType::MoveDown, (16, 19));
    sprites.indices.insert(AnimationType::MoveUp, (20, 23));
    sprites.indices.insert(AnimationType::MoveRight, (12, 15));
    sprites.indices.insert(AnimationType::MoveLeft, (8, 11));


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
                index: sprites.indices[&AnimationType::IdleLeft].0
            }
        ),
        Player,
        Transform::from_xyz(0.0, 0.0, 1.0),
        Speed(125.0),
        AnimationTimer(Timer::from_seconds(0.20, TimerMode::Repeating)),
        ActiveAnimation {
            current: AnimationType::IdleLeft,
        }
    ));
}

fn match_animation_type(
    mut query: Query<(&ActiveAnimation, &mut Sprite), (With<Player>, Changed<ActiveAnimation>)>,
    spritesheet: Res<SpriteSheetIndices>,
) {
    for (active, mut sprite) in query.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = spritesheet.indices[&active.current].0;
        }
    }
}


fn animate_sprite(
    time: Res<Time>,
    sprites: Res<SpriteSheetIndices>,
    mut query: Query<(&ActiveAnimation, &mut AnimationTimer, &mut Sprite)>,
) {
    for (animation_type, mut timer, mut sprite) in &mut query {
        if let Some(&(first, last)) = sprites.indices.get(&animation_type.current) {
            
            timer.tick(time.delta());
            if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == last {
                first
            } else {
                atlas.index + 1
            };
        }
            
        }
        
    }
}




