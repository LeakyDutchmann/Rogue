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
            });
            item_count += 1;
        }
        println!("Loaded {} items", item_count);
    }
}

fn generate_random_coords(pos: Vec2) -> Vec2 {
    let mut rng = rand::rng();
    let dx = rng.random_range(-30.0..30.0);
    let dy = rng.random_range(-30.0..30.0);
    Vec2::new(pos.x + dx, pos.y + dy)
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


