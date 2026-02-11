use super::*;

pub fn ai_movement(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut FacingDirection, &mut ActorState, &Transform), (With<Enemy>, Without<MovementIntent>)>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
    for (enemy_entity, mut facing_direction, mut actor_state, enemy_transform) in enemy_query.iter_mut() {
        let player_position = player.translation.truncate();
        let enemy_position = enemy_transform.translation.truncate();
        let mut to_player = Vec2::ZERO;
        if enemy_position.distance(player_position) < 400.0 {
            to_player = (player_position - enemy_position).normalize();
        }
        
        if to_player != Vec2::ZERO {
            commands.entity(enemy_entity).insert(MovementIntent {
                direction: to_player,
            });
            actor_state.state = ActorStateType::Walking;
            facing_direction.facing = Facing::from_direction(to_player);
        } else {
            actor_state.state = ActorStateType::Idle;
        }
        
        
        println!("Enemy AI movement");
        
    }
}