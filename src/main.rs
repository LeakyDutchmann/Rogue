mod components;
mod map_setup;
mod mouse;
mod player;
mod camera_setup;
mod animations;
mod colision_manager;
mod movement;
mod world;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        .add_plugins((MapSetupPlugin, PlayerSetupPlugin, CameraSetupPlugin, MouseInputPlugin, AnimationSetupPlugin))
        .add_plugins((ColisionPlugin, MovementPlugin))
        .add_plugins(WorldPlugin)
        .add_systems(Startup, setup)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(
            Color::srgb(1.0, 0.0, 0.0)  // Red
        ))),
        Colider {
            shape: ColiderShape::Circle { radius: 10.0},
            offsety: 0.0,
            sensor: true,
        },
        Transform::from_xyz(90.0, 0.0, 1.0),
    ));
}









