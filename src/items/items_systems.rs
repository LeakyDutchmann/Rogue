use super::*;

pub fn setup_items(
    asset_server: Res<AssetServer>,
    mut item_registry: ResMut<ItemRegistry>,
) {
    let path = "./data/items";
    if let Ok(items) = load_definitions_for::<ItemDefinitionRaw>(path) {
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
    let dx = rng.random_range(-10.0..10.0);
    let dy = rng.random_range(-10.0..10.0);
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
            let distance = player_pos.distance(item_pos);
            if distance > 20.0 {
                continue;
            }
            if distance > 10.0 {
                let to_player = (player_pos - item_pos).normalize();
                commands.entity(item_e).insert(MovementIntent {
                    direction: to_player,
                });
            }
            if distance <= 10.0 {
                let mut pushed = false;
                for slot in inventory.items.iter_mut() {
                    if let Some(stored_id) = slot.item_stored.clone() {
                        if stored_id == item_id.id {
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
                            slot.item_stored = Some(item_id.id.clone());
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

pub fn item_steering(
    mut enemy_qr: Query<(Entity, &Transform, &mut MovementIntent), With<ItemId>>,
    colider_qr: Query<(&Transform, &Colider)>,
    world: Res<WorldGrid>,
) {
    for (intender_e, intender_tf, mut intent) in enemy_qr.iter_mut() {
        let intender_pos = intender_tf.translation.truncate();
        let intender_dir = intent.direction.normalize();
        let cell_x = (intender_pos.x / CELL_SIZE).floor() as i32;
        let cell_y = (intender_pos.y / CELL_SIZE).floor() as i32;
        let cells = get_cells_3x3((cell_x, cell_y));
        let entities = get_entities_in_cells(cells, &world);
        let mut avoidance = Vec2::ZERO;
        for entity in entities {
            if entity == intender_e {
                continue;
            }
            if let Ok((tf, _colider)) = colider_qr.get(entity) {
                let to_colider_raw = tf.translation.truncate() - intender_pos;
                let distance = to_colider_raw.length();
                if distance < 0.001 {
                    continue;
                }
                let to_colider = to_colider_raw / distance; 
                let dot = intender_dir.dot(to_colider);
                if dot <= 0.0 {
                    continue;
                }
                let distance_weight = 1.0 / distance;   
                let angle_weight = dot.clamp(0.0, 1.0);
                let weight = distance_weight * angle_weight;
                avoidance += -to_colider * weight;
            }
        }
        if avoidance != Vec2::ZERO {
            intent.direction = (intender_dir + avoidance).normalize();
        } else {
            intent.direction = intender_dir;
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


