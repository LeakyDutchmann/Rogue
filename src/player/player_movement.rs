use super::*;


pub fn move_player(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(Entity, &mut ActorState, &mut FacingDirection), (With<Player>, Without<HurtTimer>)>,
) {
    for (player_e, mut player_state, mut facing_dir) in &mut player {
        let mut direction = Vec2::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        let direction = direction.normalize_or_zero();
        
        if direction != Vec2::ZERO {
            commands.entity(player_e).insert(MovementIntent { direction });
            player_state.state = ActorStateType::Walking;
            facing_dir.facing = Facing::from_direction(direction);
        } else {
            player_state.state = ActorStateType::Idle;
        }
    }
}


// pub fn print_state(
//     player: Query<&Player, With<Player>>,
// ) {
//     for player in player.iter() {
//         println!("state: {:?}, facing: {:?}", player.state, player.facing);
//     }
// }

pub fn player_idle_direction(
    cursor: ResMut<CursorWorldPos>,
    mut player: Query<(&Transform, &mut Player, &mut FacingDirection, &ActorState), With<Player>>,
) {
    for (transform, _player, mut facing_dir, actor_state) in player.iter_mut() {
        if actor_state.state == ActorStateType::Idle {
            if let Some(cursor_pos) = cursor.0 {
                let player_pos = transform.translation.truncate();
                if player_pos.x >= cursor_pos.x {
                    facing_dir.facing = Facing::Left
                } else {
                    facing_dir.facing = Facing::Right
                }
            }
            
        }
    }
}

pub fn update_player_transform(
    mut player_transform: ResMut<PlayerTransform>,
    query: Query<&Transform, With<Player>>
) {
    if let Ok(transform) = query.single() {
        player_transform.0 = *transform;
    }
}