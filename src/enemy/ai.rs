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

pub fn ai_movement(
    mut commands: Commands,
    mut enemy_qr: Query<(Entity, &EnemyAwareness, &Transform, &mut FacingDirection, &mut ActorState),
        (With<Enemy>, Without<MovementIntent>, Without<AiPath>, Without<HurtTimer>)>,
    mut writer: MessageWriter<FindPath>,
    player_sg: Single<&Transform, (With<Player>, Without<Enemy>)>
) {
    for (enemy_entity, awarenes, tf, mut facing_direction, mut actor_state) in enemy_qr.iter_mut() {
        let enemy_pos = tf.translation.truncate();
        let player_pos = player_sg.translation.truncate();
        let to_player = (player_pos - enemy_pos).normalize();
        match awarenes.state {
            AwarenessType::Direct => {
                    commands.entity(enemy_entity).insert(MovementIntent {
                        direction: to_player,
                    });
                    actor_state.state = ActorStateType::Walking;
                    facing_direction.facing = Facing::from_direction(to_player);
            },
            AwarenessType::Indirect => {
                    writer.write(FindPath {
                        seeker: enemy_entity,
                        seeker_pos: enemy_pos,
                        target_pos: player_pos,
                    });
                } 
            AwarenessType::Unaware => {
                actor_state.state = ActorStateType::Idle;
                facing_direction.facing = Facing::from_direction(to_player);
            }
        }         
    }
}

pub fn ai_steering(
    mut enemy_qr: Query<(Entity, &Transform, &mut MovementIntent), With<Enemy>>,
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

pub fn ai_initialize_attack(
    mut commands: Commands,
    player_tf: Query<(Entity, &Transform), With<Player>>,
    enemy_qr: Query<(Entity, &Transform,), With<Enemy>>,
    hend_qr: Query<(Entity, &HeldItem, &ChildOf), (Without<AttackAnimation>, Without<CoolDown>)>,
    attack_stats: Query<(&CombatStats, &AnimationPattern)>,
    registry: Res<ItemRegistry>,
) { 
    for (hend_e, held_item, childof) in hend_qr.iter() {
        if let Ok((enemy_e, enemy_tf)) = enemy_qr.get(childof.0) {
            if let Some(item) = held_item.held {
                if let Ok((player_e, player_tf)) = player_tf.single() {
                    if let Some(def) = registry.items.get(&item) {
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
                                commands.entity(hend_e).insert(
                                    AttackAnimation {
                                        anim_pattern: animation_style,
                                        progress: 0.0,
                                        duration: 60.0 / c_stats.attack_speed as f32,
                                        max_angle: (c_stats.swing_angle as f32).to_radians(),
                                        hit_triggered: false,
                                        cursor_pos: player_pos,
                                        item: item,
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