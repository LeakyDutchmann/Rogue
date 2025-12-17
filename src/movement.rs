use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Camera2d, Fixed, KeyCode, Query, Res, Time, Transform, With, Without};
use crate::components::Player;
use crate::Speed;

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



pub fn camera_update(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(player_transform) = player.single() {
        if let Ok(mut camera_transform) = camera.single_mut() {
            let follow_speed = 5.0;
            camera_transform.translation.x += (player_transform.translation.x - camera_transform.translation.x)
                * follow_speed * time.delta_secs();
            camera_transform.translation.y += (player_transform.translation.y - camera_transform.translation.y)
                * follow_speed * time.delta_secs();

        }
    }
}