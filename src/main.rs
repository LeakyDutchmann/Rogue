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

pub fn assemble_item(definition: &ItemDefinition, commands: &mut Commands) -> Entity {
    let mut entity = commands.spawn(());
    if let Some(combat_stats) = &definition.combat_stats {
        entity.insert(CombatStats{
            attack_speed: combat_stats.attack_speed as f32,
            swing_angle: combat_stats.swing_angle as f32,
            radius: combat_stats.radius as f32,
        });
    }
    if let Some(weapon_stats) = &definition.weapon_stats {
        entity.insert(WeaponStats{
            enemy_damage: weapon_stats.enemy_damage,
        });
    }
    if let Some(tool_stats) = &definition.tool_stats {
        entity.insert(ToolStats {
            structure_damage: tool_stats.structure_damage,
        });
    }
    if let Some(durability) = definition.durability {
        entity.insert(Durability {
            durability: durability,
        });
    }
    if definition.usable {
        entity.insert(Usable);
    }
    entity.insert(Sprite::from_image(definition.sprite.clone()));
    entity.insert(ItemId::Sword);
    entity.insert(AnimationPattern {
        pattern: definition.animation_style,
    });
    entity.id()
}

pub fn setup(
    mut commands: Commands,
    item_reg: Res<ItemRegistry>,
) {
    if let Some(def) = item_reg.items.get(&ItemId::Sword) {
        let item_entity = assemble_item(def, &mut commands);
        commands.entity(item_entity).insert(Transform::from_xyz(0.0, 0.0, 0.0));
        commands.entity(item_entity).insert(OnGround);
    }
}









