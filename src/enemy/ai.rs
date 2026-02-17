use super::*;

pub fn ai_movement(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut FacingDirection, &mut ActorState, &Transform, &mut AiPath), (With<Enemy>, Without<MovementIntent>)>,
    // player: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
    for (enemy_entity, mut facing_direction, mut actor_state, enemy_transform, mut ai_path) in enemy_query.iter_mut() {
        // let player_position = player.translation.truncate();
        let enemy_position = enemy_transform.translation.truncate();
        
        if let Some(step) = ai_path.steps.front() {
            println!("going from: {:?} to {:?}", enemy_position, step);
            let threshold = 1.66; // or speed * delta_time
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
                // reached step
                actor_state.state = ActorStateType::Idle;
                ai_path.steps.pop_front(); // remove the step once reached
            }
        }
    }
}
