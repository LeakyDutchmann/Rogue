mod components;
mod map_setup;
mod mouse_input;
mod player;
mod camera_setup;

use bevy::{prelude::*, sprite};
use bevy::time::Fixed;
use std::collections::HashMap;
use components::*;
use mouse_input::*;
use map_setup::*;
use player::*;
use camera_setup::*;

// #[derive(Resource)]
// pub struct MyTimer(pub Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        .add_plugins((MapSetupPlugin, PlayerSetupPlugin, CameraSetupPlugin))
        //.insert_resource(MyTimer(Timer::from_seconds(0.1, TimerMode::Once)))
        .insert_resource(CursorWorldPos(None))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_message::<MouseClickEvent>()
        .add_message::<ApplyDestruction>()
        .add_message::<MapChanged>()
        .add_message::<ScrollMessage>()
        .add_systems(Update, (get_cursor_position, mouse_click_handler, mouse_events, destruction_system))
        .add_systems(Update, (scroll_events, camera_scroll_in))
        .run();
}











