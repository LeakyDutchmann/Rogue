use super::*;

pub fn ai_movement(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &FacingDirection, &mut ActorState, &Transform), (With<Enemy>, Without<MovementIntent>)>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
    for (enemy_entity, facing_direction, mut actor_state, enemy_transform) in enemy_query.iter_mut() {
        let player_position = player.translation.truncate();
        let enemy_position = enemy_transform.translation;

        let to_player = (player_position - enemy_position.xy()).normalize();
        
        commands.entity(enemy_entity).insert(MovementIntent {
            direction: to_player,
        });
        println!("Enemy AI movement");
        
    }
}