mod movement;
mod components;
mod map;
mod mouse_input;
mod cave_generating;

use bevy::prelude::*;
use bevy::time::Fixed;
use components::*;
use mouse_input::*;
use movement::*;
use map::*;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}


#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);


// #[derive(Resource)]
// pub struct MyTimer(pub Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        //.insert_resource(MyTimer(Timer::from_seconds(0.1, TimerMode::Once)))
        .insert_resource(CursorWorldPos(None))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_message::<MouseClickEvent>()
        .add_message::<ApplyDestruction>()
        .add_message::<MapChanged>()
        .add_systems(Startup, (setup_atlas,floor_setup, map_setup, setup).chain())
        .add_systems(FixedUpdate, (move_player, camera_update))
        .add_systems(Update, (get_cursor_position, mouse_click_handler, mouse_events, destruction_system, update_map, animate_sprite))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //CAMERA SETUP

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
    ));

    //PLAYER SETUP
    let texture = asset_server.load("player_idle.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32), 
        4,
        1,
        None,
        None
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            }
        ),
        Player,
        Transform::from_xyz(0.0, 0.0, 1.0),
        Speed(250.0),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.20, TimerMode::Repeating)),
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

