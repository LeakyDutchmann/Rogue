use super::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

pub fn read_items(path: &str) -> Result<Vec<ItemDefinitionRaw>, Box<dyn std::error::Error>> {
    let mut items: Vec<ItemDefinitionRaw> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let data = fs::read_to_string(path)?;
            let structure: ItemDefinitionRaw = serde_json::from_str(&data).unwrap();
            items.push(structure);
        } else {
            continue;
        }
    }
    Ok(items)
}

pub fn setup_items(
    asset_server: Res<AssetServer>,
    mut item_registry: ResMut<ItemRegistry>,
) {
    let path = "./items";
    if let Ok(items) = read_items(path) {
        let mut item_count = 0;
        for item in items {
            let icon_texture = asset_server.load(item.icon.clone());
            let sprite_texture = asset_server.load(item.sprite.clone());
            item_registry.items.insert(item.name.clone(), ItemDefinition {
                icon: icon_texture,
                sprite: sprite_texture,
                durability: item.durability,
                usable: item.usable,
                animation_style: item.animation_style,
                combat_stats: item.combat_stats,
                weapon_stats: item.weapon_stats,
                tool_stats: item.tool_stats,
                max_stack: item.max_stack,
            });
            item_count += 1;
        }
        println!("Loaded {} items", item_count);
    }
}

fn generate_random_coords(pos: Vec2) -> Vec3 {
    let mut rng = rand::rng();
    let dx = rng.random_range(-30.0..30.0);
    let dy = rng.random_range(-30.0..30.0);
    Vec3::new(pos.x + dx, pos.y + dy, 0.0)
}

pub fn pick_up_near_item(
    mut items: Query<(Entity, &mut ItemId, &mut Transform), Without<Player>>,
    mut player: Query<(&Transform, &mut Inventory), With<Player>>,
    registry: Res<ItemRegistry>,
    mut commands: Commands,
) {
    for (item_e, item_id, item_tf) in items.iter_mut() {
        let item_pos = item_tf.translation.truncate();
        if let Ok((player_tf, mut inventory)) = player.single_mut() {
            let player_pos = player_tf.translation.truncate();
            if player_pos.distance(item_pos) > 100.0 {
                continue;
            }
            if player_pos.distance(item_pos) < 10.0 {
                let mut pushed = false;
                for slot in inventory.items.iter_mut() {
                    if let Some(stored_id) = slot.item_stored.clone() {
                        if stored_id == *item_id {
                            let def = registry.items.get(&stored_id).unwrap();
                            if slot.quantity < def.max_stack as i32 {
                                slot.quantity += 1;
                                pushed = true;
                                commands.entity(item_e).despawn();
                                break;
                            }
                        }
                    }
                }
                if !pushed {
                    for slot in inventory.items.iter_mut() {
                        if slot.item_stored.is_none() {
                            slot.item_stored = Some(*item_id);
                            slot.quantity = 1;
                            commands.entity(item_e).despawn();
                            break;                        }
                    }
                }
            }
        }
    }
}

pub fn item_spawn_system(
    mut commands: Commands,
    item_registry: Res<ItemRegistry>,
    mut reader: MessageReader<SpawnItemRequest>,
) {
    for msg in reader.read() {
        if let Some(def) = item_registry.items.get(&msg.item_id) {
            let item_e = assemble_item(def, &mut commands, &msg.item_id);
            commands.entity(item_e).insert(Sprite::from_image(def.sprite.clone()));
            commands.entity(item_e).insert(Transform::from_translation(generate_random_coords(msg.position)));
        }
    }
}



// pub fn update_dropped_items(
//     mut commands: Commands,
//     player_pos: Query<&Transform, With<Player>>,
//     mut reader: MessageReader<ItemDropped>,
//     img: Query<&Item>, 
// ) {
//     for msg in reader.read() {
//         println!("event received");
//         if let Ok(pos) = player_pos.single() {
//             println!("player position found");
//             let player_pos = pos.translation.truncate();
//             let new_pos = generate_random_coords(player_pos);
//             if let Some(item) = msg.item {
//                 println!("item found in msg");
//                 if let Ok(img) = img.get(item) {
//                     commands.entity(item).remove::<InInventory>();
//                     commands.entity(item).insert(OnGround);
//                     commands.entity(item).insert(Sprite::from_image(img.image.clone()),);
//                     commands.entity(item).insert(Transform::from_xyz(new_pos.x, new_pos.y, 1.0));
//                 } 
//             }
//         } 
//     }
// }


