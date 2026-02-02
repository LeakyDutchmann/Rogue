use super::*;


pub fn hit_detection_system(
    world: Res<WorldGrid>,
    query: Query<(&mut MapTile, &Transform), With<Wall>>,
    mut reader: MessageReader<HitMessage>,
    mut writer: MessageWriter<CalculateDamage>,
) {
    for impact in reader.read() {
        let impact_pos = impact.target.unwrap();
        if let Some(item_used) = impact.item {
            let cell_x = (impact_pos.x / CELL_SIZE).round() as i32;
            let cell_y = (impact_pos.y / CELL_SIZE).round() as i32;
            if let Some(cell_vec) = world.cells.get(&(cell_x, cell_y)) {
                for entity in cell_vec.iter() {
                    if let Ok((tile, tile_pos)) = query.get(*entity) {
                        if tile.tile_type != TileType::Floor && tile.tile_type != TileType::Empty {
                            if impact.item_pos.distance(tile_pos.translation.truncate()) <= impact.item_radius {
                                println!("hitted tile");
                                writer.write(CalculateDamage {
                                    attack_item: item_used,
                                    target: *entity,
                                    position: world_pos_to_tile_pos(impact_pos),
                                    damage_type: DamageType::ToTileDamage,
                                });
                            }
                        } 
                    } else {
                        println!("hitting in radius then");
                    }
    
                } 
            }
        }
    }
}


pub fn calculate_damage(
    weapon_stats: Query<&WeaponStats>,
    tool_stats: Query<&ToolStats>,
    mut reader: MessageReader<CalculateDamage>,
    mut writer: MessageWriter<ApplyDamage>,
) {
    for msg in reader.read() {
        let mut damage = 0;
        if let Ok(enemy_dmg) = weapon_stats.get(msg.attack_item) {
            match msg.damage_type {
                DamageType::ToEnemyDamage => {
                    damage = enemy_dmg.enemy_damage;
                }
                DamageType::ToTileDamage => {
                    damage = enemy_dmg.enemy_damage / 4;
                }
            }
        } else if let Ok(tool_dmg) = tool_stats.get(msg.attack_item) {
            match msg.damage_type {
                DamageType::ToEnemyDamage => {
                    damage = tool_dmg.structure_damage / 4;
                }
                DamageType::ToTileDamage => {
                    damage = tool_dmg.structure_damage ;
                }
            }
        }
        writer.write(ApplyDamage {
                entity: msg.target,
                position: msg.position,
                damage: damage,
            });
    } 
}
