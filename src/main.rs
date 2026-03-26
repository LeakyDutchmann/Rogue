mod components;
mod map_setup;
mod mouse;
mod player;
mod camera_setup;
mod animations;
mod colision_manager;
mod movement;
mod world;
mod items;
mod combat;
mod messages;
mod development;
mod enemy;
mod raycasting;
mod inventory;
mod building;

use bevy::prelude::*;
use bevy::time::Fixed;
use mouse::*;
use map_setup::*;
use player::*;
use camera_setup::*;
use animations::*;
use movement::*;
use colision_manager::*;
use world::*;
use items::*;
use combat::*;
use messages::*;
use development::*;
use enemy::*;
use raycasting::*;
use inventory::*;
use building::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        .add_plugins(DevPlugin) // FPS COUNTER e.t.c
        .add_plugins(EnemyPlugin)
        .add_plugins(MessagesPlugin)
        .add_plugins((MapSetupPlugin, PlayerSetupPlugin, CameraSetupPlugin, MouseInputPlugin, AnimationSetupPlugin, InventoryPlugin))
        .add_plugins((ColisionPlugin, MovementPlugin))
        .add_plugins((WorldPlugin, ItemsPlugin, CombatPlugin, BuildingPlugin))
        .add_plugins(VisionPlugin)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}









