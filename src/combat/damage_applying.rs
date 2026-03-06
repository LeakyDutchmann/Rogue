use bevy::math::NormedVectorSpace;

use crate::components::MovementIntent;

use super::*;


fn calculate_knockback(from: Vec2, to: Vec2) -> Vec2 {
    let dir = to - from;
    dir.normalize()
}

pub fn damage_execution_system(
    mut commands: Commands,
    mut reader: MessageReader<ApplyDamage>,
    mut writer: MessageWriter<MapChanged>,
    mut health: Query<&mut Health>,
    mut actor_qr: Query<(&mut ActorState)>
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
    mut hurt_qr: Query<(Entity, &mut HurtTimer, &KnockBack)>,
) {
    for (e, mut hurt_timer, kback_stats) in hurt_qr.iter_mut() {
        let knockback = calculate_knockback(kback_stats.from, kback_stats.to);
        commands.entity(e).insert(MovementIntent {
            direction: knockback * 200.0,
        });
        hurt_timer.timer.tick(time.delta());
        if hurt_timer.timer.just_finished() {
            commands.entity(e).remove::<HurtTimer>();
        }    
    }
}

