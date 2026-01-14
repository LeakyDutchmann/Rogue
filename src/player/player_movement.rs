use crate::player::*;


pub fn move_player(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(Entity, &mut Player), With<Player>>,
) {
    for (player_e, mut player_c) in &mut player {
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
            player_c.state = PlayerState::Walking;
            player_c.facing = direction_to_facing(direction);
        } else {
            player_c.state = PlayerState::Idle;
        }
    }
}

fn direction_to_facing(direction: Vec2)-> Facing {
    if direction.x.abs() >= direction.y.abs() {
            if direction.x > 0.0 { Facing::Right } else { Facing::Left }
        } else {
            if direction.y > 0.0 { Facing::Up } else { Facing::Down }
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
    mut player: Query<(&Transform, &mut Player), With<Player>>,
) {
    for (transform, mut player) in player.iter_mut() {
        if player.state == PlayerState::Idle {
            if let Some(cursor_pos) = cursor.0 {
                let player_pos = transform.translation.truncate();
                if player_pos.x >= cursor_pos.x {
                    player.facing = Facing::Left
                } else {
                    player.facing = Facing::Right
                }
            }
            
        }
    }
}