mod components;
mod map_setup;
mod mouse;
mod player;
mod camera_setup;
mod animations;

use bevy::prelude::*;
use bevy::time::Fixed;
use mouse::*;
use map_setup::*;
use player::*;
use camera_setup::*;
use animations::*;
use components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        .add_plugins((MapSetupPlugin, PlayerSetupPlugin, CameraSetupPlugin, MouseInputPlugin, AnimationSetupPlugin))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}











