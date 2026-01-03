use bevy::prelude::*;
use crate::components::*;

pub fn camera_setup(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.4,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn camera_follow_player(
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