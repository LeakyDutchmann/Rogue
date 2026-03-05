use super::*;



pub fn damage_execution_system(
    mut commands: Commands,
    mut reader: MessageReader<ApplyDamage>,
    mut writer: MessageWriter<MapChanged>,
    mut health: Query<&mut Health>,
    mut actor_qr: Query<(&mut ActiveAnimation, &mut ActorState, &FacingDirection)>
) {
    for destruction in reader.read() {
        if let Ok(mut hp) = health.get_mut(destruction.entity) {
            hp.0 -= destruction.damage;
            if destruction.damage_type == DamageType::ToEnemyDamage {
                if let Ok((mut active, mut actor_state, facing)) = actor_qr.get_mut(destruction.entity) {
                    actor_state.state = ActorStateType::Hurt;
                    active.set_animation(AnimationId::hurt_from(facing.facing));
                    commands.entity(destruction.entity).insert(HurtTimer {
                        timer: Timer::from_seconds(2.0, TimerMode::Once)
                    });
                    println!("Hurt");
                }
            }
            if hp.0 <= 0 {
                commands.entity(destruction.entity).despawn();
                if destruction.damage_type == DamageType::ToTileDamage {
                    writer.write(MapChanged {
                        position: world_pos_to_tile_pos(destruction.position),
                    });
                }
            }
        } else {
        }
    }
}

pub fn tick_hurt_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut hurt_qr: Query<(Entity, &mut HurtTimer)>,
) {
    for (e, mut timer) in hurt_qr.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            commands.entity(e).remove::<HurtTimer>();
        }
    }
}