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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        .add_plugins(DevPlugin) // FPS COUNTER e.t.c
        .add_plugins(EnemyPlugin)
        .add_plugins(MessagesPlugin)
        .add_plugins((MapSetupPlugin, PlayerSetupPlugin, CameraSetupPlugin, MouseInputPlugin, AnimationSetupPlugin))
        .add_plugins((ColisionPlugin, MovementPlugin))
        .add_plugins((WorldPlugin, ItemsPlugin, CombatPlugin))
        .add_plugins(VisionPlugin)
        .add_systems(Startup, setup)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}



pub fn setup(
    mut commands: Commands,
    item_reg: Res<ItemRegistry>,
) {
    if let Some(def) = item_reg.items.get(&ItemId::Sword) {
        let item_entity = assemble_item(def, &mut commands, &ItemId::Sword);
        commands.entity(item_entity).insert(Transform::from_xyz(0.0, 0.0, 0.0));
        commands.entity(item_entity).insert(OnGround);
    }
    if let Some(def_2) = item_reg.items.get(&ItemId::PickAxe) {
        let item_entity = assemble_item(def_2, &mut commands, &ItemId::PickAxe);
        commands.entity(item_entity).insert(Transform::from_xyz(0.0, 0.0, 0.0));
        commands.entity(item_entity).insert(OnGround);
    }
}









