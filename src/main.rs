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

use bevy::prelude::*;
use bevy::time::Fixed;
use mouse::*;
use map_setup::*;
use player::*;
use camera_setup::*;
use animations::*;
use components::*;
use movement::*;
use colision_manager::*;
use world::*;
use items::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        .add_plugins((MapSetupPlugin, PlayerSetupPlugin, CameraSetupPlugin, MouseInputPlugin, AnimationSetupPlugin))
        .add_plugins((ColisionPlugin, MovementPlugin))
        .add_plugins((WorldPlugin, ItemsPlugin))
        .add_systems(Startup, setup)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("potion.png");
    commands.spawn((
        Sprite::from_image(texture.clone()),
        Transform::from_xyz(90.0, 0.0, 1.0),
        Item {
            image: texture.clone(),
        },
        OnGround,
    ));
    
    //making ui inventory
    
    
}




fn check_inventory(
     mut inventory: Query<&mut Inventory, With<Player>>,
) {
    let inventory = inventory.single_mut().unwrap();
    println!("Checking inventory");
    for item in inventory.items.iter() {
        if let Some(item) = item {
            println!("Item found");
        } else {
            println!("No item found");
        }
    }
}








