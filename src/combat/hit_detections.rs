use super::*;



pub fn normalize_angle(a: f32) -> f32 {
    let a = a % (2.0 * std::f32::consts::PI);
    if a > std::f32::consts::PI { a - 2.0 * std::f32::consts::PI }
    else if a < -std::f32::consts::PI { a + 2.0 * std::f32::consts::PI }
    else { a }
}

pub fn hit_detection_system(
    mut commands: Commands,
    hitbox_qr: Query<(Entity, &HitBox, &Transform), Without<HitBoxUsed>>,
    hurtbox_qr: Query<(&HurtBox, &Transform), Without<HitBox>>,
    worldgrid: Res<WorldGrid>,
    mut writer: MessageWriter<CalculateDamage>,
    query: Query<(&MapTile, &Transform), With<Wall>>,
) {
    for (hitbx_e, hitbox, hitbox_tf) in hitbox_qr.iter() {
        let hitbox_pos = hitbox_tf.translation.truncate();
        let mut hit_something = false;
        let item_cell_x = (hitbox_pos.x / CELL_SIZE).round() as i32;
        let item_cell_y = (hitbox_pos.y / CELL_SIZE).round() as i32;
        let cells = get_cells_in_radius((item_cell_x, item_cell_y), hitbox.radius);
        let entities = get_entities_in_cells(cells, &worldgrid);
        for entity in entities {
            if let Ok((hurtbox, tf)) = hurtbox_qr.get(entity) {
                if entity == hitbox.owner {
                    continue
                }
                if hitbox.fraction == hurtbox.fraction {
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
                        from_pos: hitbox_pos,
                        position: tf.translation.truncate(),
                        damage_type: DamageType::ToEnemyDamage,
                    });
                    hit_something = true;
                }
            }
        }
        if hit_something == false {
            let cell_x = (hitbox.aim.x / CELL_SIZE).round() as i32;
            let cell_y = (hitbox.aim.y / CELL_SIZE).round() as i32;
            if let Some(entities) = worldgrid.cells.get(&(cell_x, cell_y)) {
                for entity in entities {
                    if let Ok((_tile_type, transform)) = query.get(*entity) {
                        let pos = transform.translation.truncate();
                        let to_hurt_box = pos - hitbox_pos;
                        let distance = to_hurt_box.length();
                        if distance > hitbox.radius {
                            continue
                        } else {
                            writer.write( CalculateDamage {
                                attack_item: hitbox.item_used,
                                target: *entity,
                                from_pos: hitbox_pos,
                                position: pos,
                                damage_type: DamageType::ToTileDamage,
                            });
                            commands.entity(hitbx_e).insert(HitBoxUsed);
                            break;
                        }
                    }
                }
            }
        }
        commands.entity(hitbx_e).insert(HitBoxUsed);
    }
}

pub fn calculate_damage(
    mut reader: MessageReader<CalculateDamage>,
    mut writer: MessageWriter<ApplyDamage>,
    registry: Res<ItemRegistry>,
) {
    for msg in reader.read() {
        let mut damage = 0;
        if let Some(def) = registry.items.get(&msg.attack_item) {
            if let Some(attack_stats) = def.weapon_stats.as_ref().copied() {
                match msg.damage_type {
                    DamageType::ToEnemyDamage => {
                        damage = attack_stats.enemy_damage;
                    }
                    DamageType::ToTileDamage => {
                        damage = attack_stats.enemy_damage / 4;
                    }
                }
            } else if let Some(tool_dmg) = def.tool_stats {
                match msg.damage_type {
                    DamageType::ToEnemyDamage => {
                        damage = tool_dmg.structure_damage / 4;
                    }
                    DamageType::ToTileDamage => {
                        damage = tool_dmg.structure_damage ;
                    }
                }
            }
        } 
        writer.write(ApplyDamage {
            entity: msg.target,
            position: msg.position,
            damage: damage,
            from_pos: msg.from_pos,
            damage_type: msg.damage_type,
        });
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

