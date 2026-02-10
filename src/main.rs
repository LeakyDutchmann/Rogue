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

struct OverlayColor;

impl OverlayColor {
    const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        .add_plugins(DevPlugin) // FPS COUNTER e.t.c
        .add_plugins(MessagesPlugin)
        .add_plugins((MapSetupPlugin, PlayerSetupPlugin, CameraSetupPlugin, MouseInputPlugin, AnimationSetupPlugin))
        .add_plugins((ColisionPlugin, MovementPlugin))
        .add_plugins((WorldPlugin, ItemsPlugin, CombatPlugin))
        .add_systems(Startup, setup)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let pickaxe_texture = asset_server.load("pickaxe.png");
    let sword_texture = asset_server.load("sword.png");
    commands.spawn((
        Sprite::from_image(pickaxe_texture.clone()),
        Transform::from_xyz(44.0, 0.0, 1.0),
        Item {
            image: pickaxe_texture.clone(),
            name: "Pickaxe".to_string(),
        },
        OnGround,
        CombatStats {
            attack_speed: 300.0,
            swing_angle: std::f32::consts::PI / 3.0,
            radius: 80.0,
        },
        AnimationPattern{
            pattern: AnimationStyle::PickAxe,
        },
        ToolStats {
            structure_damage: 25,
        },
        Durability {
            durability: 100.0,
        }
    ));
    commands.spawn((
        Sprite::from_image(sword_texture.clone()),
        Transform::from_xyz(44.0, 0.0, 1.0),
        Item {
            image: sword_texture.clone(),
            name: "Sword".to_string(),
        },
        OnGround,
        CombatStats {
            attack_speed: 250.0,
            swing_angle: std::f32::consts::PI / 1.0,
            radius: 120.0,
        },
        AnimationPattern {
            pattern: AnimationStyle::Sword,
        },
        WeaponStats {
            enemy_damage: 50,
        },
        Durability {
            durability: 100.0,
        }
    ));
}




// fn check_inventory(
//      mut inventory: Query<&mut Inventory, With<Player>>,
// ) {
//     let inventory = inventory.single_mut().unwrap();
//     println!("Checking inventory");
//     for item in inventory.items.iter() {
//         if let Some(item) = item {
//             println!("Item found");
//         } else {
//             println!("No item found");
//         }
//     }
// }








