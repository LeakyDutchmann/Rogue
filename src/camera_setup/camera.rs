use crate::camera_setup::*;

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

pub fn camera_scroll_in(
    mut reader: MessageReader<ScrollMessage>,
    mut query: Query<&mut Projection, With<Camera2d>>,
) {
    for msg in reader.read() {
        for mut projection in query.iter_mut() {
            if let Projection::Orthographic(ref mut ortho) = *projection {
                match msg.event {
                ScrollDir::ScrollUp => {
                    ortho.scale *= 0.9;
                }
                ScrollDir::ScrollDown => {
                    ortho.scale *= 1.1;
                }
            }
            }
        }
    }
}