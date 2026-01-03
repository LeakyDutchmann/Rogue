use bevy::prelude::*;
use crate::components::*;


pub fn move_player(
    time: Res<Time<Fixed>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &Speed), With<Player>>,
) {
    for (mut transform, speed) in &mut player {
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
    }
}