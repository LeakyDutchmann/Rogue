use super::*;


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
    match (player.single(), camera.single_mut()) {
        (Ok(player_pos), Ok(mut camera_pos)) => {
            let follow_speed = 5.0;
            camera_pos.translation.x += (player_pos.translation.x - camera_pos.translation.x)
                * follow_speed * time.delta_secs();
            camera_pos.translation.y += (player_pos.translation.y - camera_pos.translation.y)
                * follow_speed * time.delta_secs();
                }
        _ => { panic!("failed to find player or camera. You're fucked up")}
    }
}

pub fn camera_scroll_in(
    mut reader: MessageReader<KeyPressed>,
    mut query: Query<&mut Projection, With<Camera2d>>,
) {
    for msg in reader.read() {
        for mut projection in query.iter_mut() {
            if let Projection::Orthographic(ref mut ortho) = *projection {
                match msg.key {
                    KeyCode::Equal => {
                    ortho.scale *= 0.9;
                },
                    KeyCode::Minus => {
                        ortho.scale *= 1.1;
                },
                _ => {}
            }
            }
        }
    }
}