use super::*;

pub fn follow_path(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut FacingDirection, &mut ActorState, &Transform, &mut AiPath), (With<Enemy>, Without<MovementIntent>,  Without<HurtTimer>)>,
) {
    for (enemy_entity, mut facing_direction, mut actor_state, enemy_transform, mut ai_path) in enemy_query.iter_mut() {
        let enemy_position = enemy_transform.translation.truncate();
        if let Some(step) = ai_path.steps.front() {
            let threshold = 1.66;
            let direction = step - enemy_position;
            let distance = direction.length();
            if distance > threshold {
                let to_step = direction.normalize_or_zero();
                commands.entity(enemy_entity).insert(MovementIntent {
                    direction: to_step,
                });
                actor_state.state = ActorStateType::Walking;
                facing_direction.facing = Facing::from_direction(to_step);
            } else {
                actor_state.state = ActorStateType::Idle;
                ai_path.steps.pop_front(); 
            }
        } else {
            actor_state.state = ActorStateType::Idle;
            commands.entity(enemy_entity).remove::<AiPath>();
        }
    }
}

pub fn ai_pursuing_system(
    mut commands: Commands,
    mut enemy_qr: Query<(Entity, &EnemyState, &Transform, &mut FacingDirection, &mut ActorState),
        (With<Enemy>, Without<MovementIntent>, Without<HurtTimer>)>,
    player_sg: Single<&Transform, (With<Player>, Without<Enemy>)>,
    ai_path: Query<&AiPath>,
) {
    for (enemy_entity, state, tf, mut facing_direction, mut actor_state) in enemy_qr.iter_mut() {
        if state.current != EnemyStateType::Pursuing {
            continue;
        }
        let enemy_pos = tf.translation.truncate();
        let player_pos = player_sg.translation.truncate();
        let to_player = (player_pos - enemy_pos).normalize();
        if let Ok(_aipath) = ai_path.get(enemy_entity) {    
            commands.entity(enemy_entity).remove::<AiPath>();
        }
        actor_state.state = ActorStateType::Walking;
        facing_direction.facing = Facing::from_direction(to_player);    
        commands.entity(enemy_entity).insert(MovementIntent {
            direction: to_player,
        });
    }
}

const RADIUS: f32 = 64.0;

pub fn ai_cosmetics_steering(
    mut enemy_qr: Query<(Entity, &Transform, &mut MovementIntent), With<Enemy>>,
    colider_qr: Query<(&Transform, &Colider)>,
    world: Res<WorldGrid>,
) {
    for (intender_e, intender_tf, mut intent) in enemy_qr.iter_mut() {
        let intender_pos = intender_tf.translation.truncate();
        let intender_dir = intent.direction.normalize();
        let cell_x = (intender_pos.x / CELL_SIZE).round() as i32;
        let cell_y = (intender_pos.y / CELL_SIZE).round() as i32;
        let cells = get_cells_3x3((cell_x, cell_y));
        let entities = get_entities_in_cells(cells, &world);
        let mut avoidance = Vec2::ZERO;
        for entity in entities {
            if entity == intender_e {
                continue;
            }
            if let Ok((tf, colider)) = colider_qr.get(entity) {
                match colider.shape {
                    ColiderShape::Circle { radius: _} => {
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
                        avoidance += -to_colider * weight * 1.5;
                    }
                    ColiderShape::Rectangle { width: _, height: _ } => {
                        continue;
                    }
                }
            }
        }
        if avoidance != Vec2::ZERO {
            intent.direction = (intender_dir + avoidance).normalize();
        } else {
            intent.direction = intender_dir;
        }
    }
}

pub fn ai_steering(
    mut enemy_qr: Query<(Entity, &Transform, &mut MovementIntent, &Colider), With<Enemy>>,
    colider_qr: Query<(&Transform, &Colider)>,
    world: Res<WorldGrid>,
) {
    for (intender_e, intender_tf, mut intent, colider) in enemy_qr.iter_mut() {
        let agent_radius = match colider.shape {
            ColiderShape::Circle{radius} => radius,
            _ => 0.0,
        };
        if agent_radius == 0.0 {
            continue;
        }
        let intender_pos = intender_tf.translation.truncate();
        let intender_dir = intent.direction.normalize();
        let cell_x = (intender_pos.x / CELL_SIZE).round() as i32;
        let cell_y = (intender_pos.y / CELL_SIZE).round() as i32;
        let cells = get_cells_3x3((cell_x, cell_y));
        let entities = get_entities_in_cells(cells, &world);
        let dirs = vec![
            Vec2::new(0.0, 1.0).normalize(),
            Vec2::new(1.0, 0.0).normalize(),
            Vec2::new(0.0, -1.0).normalize(),
            Vec2::new(-1.0, 0.0).normalize(),
            Vec2::new(1.0, 1.0).normalize(),
            Vec2::new(1.0, -1.0).normalize(),
            Vec2::new(-1.0, 1.0).normalize(),
            Vec2::new(-1.0, -1.0).normalize(),
        ];
        let mut danger = vec![0.0; dirs.len()];
        let mut interest = vec![0.0; dirs.len()];
        let mut final_rate = vec![0.0; dirs.len()];
        for entity in &entities {
            if entity == &intender_e {
                continue;
            }
            if let Ok((tf, colider)) = colider_qr.get(*entity) {
                match colider.shape {
                    ColiderShape::Circle { radius: _ } => {
                        continue;
                    }
                    ColiderShape::Rectangle { width: _width, height: _height } => {
                        let tile_min = tf.translation.truncate() - Vec2::splat(16.0);
                        let tile_max = tf.translation.truncate() + Vec2::splat(16.0);
                        
                        let closest_x = intender_pos.x.clamp(tile_min.x, tile_max.x);
                        let closest_y = intender_pos.y.clamp(tile_min.y, tile_max.y);
                        
                        let closest_point = Vec2::new(closest_x, closest_y);
                        let direction_to_collider = closest_point - intender_pos;
                        
                        let distance = direction_to_collider.length();
                        let weight = if distance <= agent_radius {
                            1.0
                        } else if distance <= RADIUS {
                            (RADIUS - distance) / RADIUS
                        } else {
                            0.0
                        };
                        let dir_to_normalized = direction_to_collider.normalize();
                        for (i, dir) in dirs.iter().enumerate() {
                            let dot = dir_to_normalized.dot(dir.clone()).clamp(0.0, 1.0);
                            let new_danger = dot * weight;
                            if new_danger > danger[i] {
                                danger[i] = new_danger;
                            }
                        }
                    }
                }
            }
        }
        for (i, dir) in dirs.iter().enumerate() {
            let dot = intender_dir.dot(*dir);
            interest[i] = dot.clamp(0.0, 1.0);
        }
        for (i, _dir) in dirs.iter().enumerate() {
            let rating = interest[i] - danger[i];
            final_rate[i] = rating;
        }
        let mut final_vec = Vec2::ZERO;
        for (i, dir) in dirs.iter().enumerate() {
            let w = final_rate[i];
            if w > 0.0 {
                final_vec += dir.clone() * w;
            }
        }
        if final_vec.length_squared() > 0.0 {
            intent.direction = final_vec.normalize();
        }
    }
}

pub fn ai_initialize_attack(
    mut commands: Commands,
    player_tf: Query<(Entity, &Transform), With<Player>>,
    enemy_qr: Query<(Entity, &Transform, &ActorState), With<Enemy>>,
    hend_qr: Query<(Entity, &HeldItem, &ChildOf), (Without<AttackAnimation>, Without<CoolDown>)>,
    registry: Res<ItemRegistry>,
) { 
    for (hend_e, held_item, childof) in hend_qr.iter() {
        if let Ok((enemy_e, enemy_tf, actor)) = enemy_qr.get(childof.0) {
            if let Some(item) = held_item.held.as_ref() {
                if let Ok((player_e, player_tf)) = player_tf.single() {
                    if let Some(def) = registry.items.get(item) {
                        if let Some(c_stats) = def.combat_stats {
                            if player_e == enemy_e {
                                continue;
                            }
                            let enemy_pos = enemy_tf.translation.truncate();
                            let player_pos = player_tf.translation.truncate();
                            if enemy_pos.distance(player_pos) > c_stats.radius as f32 {
                                continue
                            }
                            if let Some(animation_style) = def.animation_style {
                                if actor.state != ActorStateType::Dead {
                                    commands.entity(hend_e).insert(
                                        AttackAnimation {
                                            anim_pattern: animation_style,
                                            progress: 0.0,
                                            duration: 60.0 / c_stats.attack_speed as f32,
                                            max_angle: (c_stats.swing_angle as f32).to_radians(),
                                            hit_triggered: false,
                                            cursor_pos: player_pos,
                                            item: item.clone(),
                                            item_radius: c_stats.radius as f32,
                                        }
                                    );
                                }
                                
                            }
                        }
                        
                    } 
                }
            }
            
        }
    }
}