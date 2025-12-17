mod movement;
mod components;
mod map;
mod mouse_input;

use bevy::prelude::*;
use bevy::time::Fixed;
use components::*;
use mouse_input::*;
use movement::*;
use map::*;

#[derive(Resource)]
pub struct MyTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        //.insert_resource(MyTimer(Timer::from_seconds(0.1, TimerMode::Once)))
        .insert_resource(CursorWorldPos(None))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_message::<MouseClickEvent>()
        .add_systems(Startup, (setup_atlas,floor_setup, map_setup, setup).chain())
        .add_systems(FixedUpdate, (move_player, camera_update))
        .add_systems(Update, (get_cursor_position, mouse_click_handler))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //CAMERA SETUP

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
    ));

    //PLAYER SETUP

    commands.spawn((
        Sprite::from_image(asset_server.load("player.png")),
        Player,
        Transform::from_xyz(0.0, 0.0, 1.0),
        Speed(250.0),
    ));
}
