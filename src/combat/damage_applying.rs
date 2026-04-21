use super::*;

fn calculate_knockback(from: Vec2, to: Vec2) -> Vec2 {
    let dir = to - from;
    dir.normalize()
}

pub fn damage_execution_system(
    mut commands: Commands,
    mut reader: MessageReader<ApplyDamage>,
    mut writer: MessageWriter<MapChanged>,
    mut item_writer: MessageWriter<SpawnItemRequest>,
    mut health: Query<&mut Health>,
    mut actor_qr: Query<&mut ActorState>,
    deathtimer: Query<&DeathTimer>,
    tile_type: Query<&MapTile>,
    mut chunkgrid: ResMut<ChunkGrid>,
) {
    for destruction in reader.read() {
        if let Ok(mut hp) = health.get_mut(destruction.entity) {
            hp.0 -= destruction.damage;
            if destruction.damage_type == DamageType::ToEnemyDamage {
                if let Ok(mut actor_state) = actor_qr.get_mut(destruction.entity) {
                    actor_state.state = ActorStateType::Hurt;
                    commands.entity(destruction.entity).insert(HurtTimer {
                        timer: Timer::from_seconds(0.5, TimerMode::Once)
                    });
                    commands.entity(destruction.entity).insert(KnockBack {
                        from: destruction.from_pos,
                        to: destruction.position,
                    });
                }   
            }
            if destruction.damage_type == DamageType::ToStructureDamage {
                let chunk_pos = get_chunk_pos(destruction.position);
                if let Some(chunk) = chunkgrid.chunks.get_mut(&chunk_pos) {
                    chunk.changed = true;
                }
            }
            if hp.0 <= 0 {
                if destruction.damage_type == DamageType::ToTileDamage {
                    let chunk_pos = get_chunk_pos(destruction.position);
                    writer.write(MapChanged {
                        pos: destruction.position,
                    });
                    if let Ok(map_tile) = tile_type.get(destruction.entity) {
                        if let Some(ore_id) = map_tile.material.get_ore_id() {
                            item_writer.write(SpawnItemRequest {
                                position: destruction.position,
                                item_id: ore_id.clone(),
                            });
                            commands.entity(destruction.entity).despawn();
                        }
                    }
                } else {
                    if let Ok(mut actor_state) = actor_qr.get_mut(destruction.entity) {
                        if actor_state.state != ActorStateType::Dead {
                            actor_state.state = ActorStateType::Dead;
                            if deathtimer.get(destruction.entity).is_err() {
                                commands.entity(destruction.entity).insert( DeathTimer {
                                    timer: Timer::from_seconds(50.0, TimerMode::Once),
                                });
                            }
                        }  
                    } else {
                        commands.entity(destruction.entity).despawn();
                    }
                }
            }
        }
    }
}

pub fn tick_hurt_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut hurt_qr: Query<(Entity, &mut HurtTimer, &KnockBack)>,
) {
    for (e, mut hurt_timer, kback_stats) in hurt_qr.iter_mut() {
        let knockback = calculate_knockback(kback_stats.from, kback_stats.to);
        commands.entity(e).insert(MovementIntent {
            direction: knockback,
        });
        hurt_timer.timer.tick(time.delta());
        if hurt_timer.timer.just_finished() {
            commands.entity(e).remove::<HurtTimer>();
        }    
    }
}

pub fn dead_actor_processing(
    mut commands: Commands,
    time: Res<Time>,
    mut actors: Query<(Entity, &ActorState, &mut DeathTimer, &mut Sprite)>,
) {
    for (actor_e, actor_s, mut timer, mut sprite) in actors.iter_mut() {
        match actor_s.state {
            ActorStateType::Dead => {
                if timer.timer.elapsed().is_zero() {
                    commands.entity(actor_e).remove::<(
                        Health,
                        Colider,
                        HurtBox,
                        AnimationTimer,
                        ActiveAnimation,
                        FacingDirection,
                        Speed,
                        EnemyAwareness,
                    )>();
                    commands.entity(actor_e).despawn_children();
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = 2;
                    } else {
                    }
                }
                timer.timer.tick(time.delta());
                if timer.timer.is_finished() {
                    commands.entity(actor_e).despawn();
                }
            }
            _ => {continue}
        }
    }
}

pub fn tick_cooldown(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CoolDown)>,
) {
    for (e, mut cool_down) in query.iter_mut() {
        cool_down.timer.tick(time.delta());
        if cool_down.timer.just_finished() {
            commands.entity(e).remove::<CoolDown>();
        }    
    }
}