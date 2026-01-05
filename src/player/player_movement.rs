use crate::player::*;


pub fn move_player(
    time: Res<Time<Fixed>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &Speed, &mut Player)>,
) {
    for (mut transform, speed, mut player) in &mut player {
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
        let movement = direction * speed.0 * time.delta_secs();

        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        
        if direction != Vec2::ZERO {
            player.state = PlayerState::Walking;
            player.facing = direction_to_facing(direction);
            println!("Player is walking in direction {:?}", player.facing);
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