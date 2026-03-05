use super::*;


pub fn hit_detection_system_old(
    world: Res<WorldGrid>,
    query: Query<(&mut MapTile, &Transform), With<Wall>>,
    transform: Query<&Transform>,
    enemy_marker: Query<&Enemy>,
    mut reader: MessageReader<HitMessage>,
    mut writer: MessageWriter<CalculateDamage>,
) {
    for impact in reader.read() {
        let mut hit_registered = false;
        if let Some(impact_pos) = impact.target {
            if let Some(item_used) = impact.item {
                let cell_x = (impact_pos.x / CELL_SIZE).round() as i32;
                let cell_y = (impact_pos.y / CELL_SIZE).round() as i32;
                if let Some(cell_vec) = world.cells.get(&(cell_x, cell_y)) {
                    for entity in cell_vec.iter() {
                        if let Ok((tile, tile_pos)) = query.get(*entity) {
                            if tile.tile_type != TileType::Floor && tile.tile_type != TileType::Empty {
                                if impact.item_pos.distance(tile_pos.translation.truncate()) <= impact.item_radius {
                                    // println!("hitted tile");
                                    writer.write(CalculateDamage {
                                        attack_item: item_used,
                                        target: *entity,
                                        position: impact_pos,
                                        damage_type: DamageType::ToTileDamage,
                                    });
                                } else {
                                    // println!("to far");
                                }
                            } 
                        } else {
                            // println!("hitting in radius then");
                            let item_cell_x = (impact.item_pos.x / CELL_SIZE).round() as i32;
                            let item_cell_y = (impact.item_pos.y / CELL_SIZE).round() as i32;
                            let cells = get_cells_in_radius((item_cell_x, item_cell_y), impact.item_radius);
                            let entities_in_cells = get_entities_in_cells(cells, &world);
                            let mut min_distance = impact.item_radius;
                            let mut closest_entity: Option<Entity> = None;
                            let mut closest_entity_pos = None;
                            for entity in entities_in_cells {
                                if let Ok(_) = enemy_marker.get(entity) {
                                    if let Ok(entity_pos) = transform.get(entity) {
                                        let distance = impact.item_pos.distance(entity_pos.translation.truncate());
                                        if distance < min_distance {
                                            min_distance = distance;
                                            closest_entity = Some(entity);
                                            closest_entity_pos = Some(entity_pos.translation.truncate());
                                        }
                                    }
                                }
                            }
                            if !hit_registered {
                                if let (Some(target), Some(position)) = (closest_entity, closest_entity_pos) {
                                    writer.write( CalculateDamage {
                                        attack_item: item_used,
                                        target: target,
                                        position: position,
                                        damage_type: DamageType::ToEnemyDamage,
                                    });
                                    hit_registered = true;
                                    // println!("Hit enemy!")
                                } else {
                                    // println!("No enemy in radius!")
                                }
                            }
                        }
        
                    } 
                }
            }
        }
    }
}

pub fn normalize_angle(a: f32) -> f32 {
    let a = a % (2.0 * std::f32::consts::PI);
    if a > std::f32::consts::PI { a - 2.0 * std::f32::consts::PI }
    else if a < -std::f32::consts::PI { a + 2.0 * std::f32::consts::PI }
    else { a }
}

pub fn hit_detection_system(
    mut commands: Commands,
    mut hitbox_qr: Query<(Entity, &mut HitBox, &Transform), Without<HitBoxUsed>>,
    mut hurtbox_qr: Query<(&HurtBox, &Transform), Without<HitBox>>,
    worldgrid: Res<WorldGrid>,
    mut writer: MessageWriter<CalculateDamage>,
) {
    for (hitbx_e, mut hitbox, hitbox_tf) in hitbox_qr.iter_mut() {
        let hitbox_pos = hitbox_tf.translation.truncate();
        let item_cell_x = (hitbox_pos.x / CELL_SIZE).round() as i32;
        let item_cell_y = (hitbox_pos.y / CELL_SIZE).round() as i32;
        let cells = get_cells_in_radius((item_cell_x, item_cell_y), hitbox.radius);
        let entities = get_entities_in_cells(cells, &worldgrid);
        for entity in entities {
            if let Ok((hurtbox, tf)) = hurtbox_qr.get(entity) {
                if entity == hitbox.owner {
                    continue
                }
                let to_hurt_box = tf.translation.truncate() - hitbox_pos;
                let distance = to_hurt_box.length();
                if distance > hitbox.radius + hurtbox.radius {
                    continue
                }
                let angle = normalize_angle(to_hurt_box.y.atan2(to_hurt_box.x));
                let start = normalize_angle(hitbox.start_angle);
                let end   = normalize_angle(hitbox.end_angle);
                
                let in_sector = if start <= end {
                    angle >= start && angle <= end
                } else {
                    // crosses ±π boundary: angle is outside the gap between end and start
                    !(angle > end && angle < start)
                };
                if in_sector {
                    writer.write( CalculateDamage {
                        attack_item: hitbox.item_used,
                        target: entity,
                        position: tf.translation.truncate(),
                        damage_type: DamageType::ToEnemyDamage,
                    });
                }
                println!(
                    "hit check | enemy: {:?} | to_hurt_box: {:?} | angle: {:.3} | start: {:.3} | end: {:.3}",
                    entity, to_hurt_box, angle, start, end
                );
            }
        }
        commands.entity(hitbx_e).insert(HitBoxUsed);
    }
}

pub fn calculate_damage(
    weapon_stats: Query<&WeaponStats>,
    tool_stats: Query<&ToolStats>,
    mut reader: MessageReader<CalculateDamage>,
    mut writer: MessageWriter<ApplyDamage>,
) {
    for msg in reader.read() {
        let mut damage_calculated = false;
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
        if !damage_calculated {
            writer.write(ApplyDamage {
                    entity: msg.target,
                    position: msg.position,
                    damage: damage,
                    damage_type: msg.damage_type,
                });
            damage_calculated = true;
        }
    } 
}

pub fn despawn_used_hitboxes(
    mut commands: Commands,
    query: Query<Entity, With<HitBoxUsed>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

