use super::*;

pub fn ai_brain_system(
    mut enemies: Query<(&Transform, &EnemyAwareness, &mut EnemyState)>,
    player_sg: Single<&Transform, (With<Player>, Without<Enemy>)>,
    swarm_buff: Res<SwarmBuffState>,
) {
    let player_pos = player_sg.translation.truncate();
    for (tf, awareness, mut state) in enemies.iter_mut() {
        let enemy_pos = tf.translation.truncate();
        let distance = (player_pos - enemy_pos).length();
        match awareness.state {
            AwarenessType::Unaware => {
                state.set(EnemyStateType::Patroling);
            },
            AwarenessType::Direct => {
                if distance <= 64.0 {
                    state.set(EnemyStateType::Pursuing);
                } else if swarm_buff.0 {
                    state.set(EnemyStateType::Surrounding);
                } else {
                    state.set(EnemyStateType::Surrounding);
                }
            },
            AwarenessType::Indirect => {
                state.set(EnemyStateType::Pathfinding);
            },
        }
    }
}

pub fn show_enemy_state(
    mut enemies: Query<(&mut EnemyState, &Children), Changed<EnemyState>>,
    mut text: Query<&mut Text2d, With<DebugMarker>>
) {
    for (state, children) in enemies.iter_mut() {
        for child in children.iter() {
            if let Ok(mut text) = text.get_mut(child) {
                match state.current {
                    EnemyStateType::Patroling => {
                        text.0 = "Patroling".to_string();
                    },
                    EnemyStateType::Pursuing => {
                        text.0 = "Pursuing".to_string();
                    },
                    EnemyStateType::Surrounding => {
                        text.0 = "Surrounding".to_string();
                    },
                    EnemyStateType::Pathfinding => {
                        text.0 = "Pathfinding".to_string();
                    },
                    _ => {
                        text.0 = "Idle".to_string();
                    },
                }
            }
        }
    }
}