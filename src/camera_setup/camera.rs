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
    // You can also do `match` on tuples, e.g.
    // match (player.single(), camera.single_mut()) {
    //   Ok(p), Ok(c) => {},
    //   _ =>
    // }
    //
    // I recommend to panic if these objects are not found, because it means something
    // is critically wrong. You can even unwrap() or expect("Player not found") them.
    //
    // When it's OK to use unwrap/expect: https://burntsushi.net/unwrap/

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
    // You are welcome to borrow the smooth zooming code from Stepsons,
    // proportional to the wheel delta.
    // I didn't do the math myself, I borrowed it somewhere (shame on me)
    // https://github.com/stepsons-of-universe/macroquad-tiled-redux/blob/3fe76cd87a3e2621f19c62e9c9a78d12c6703f3a/examples/human/main.rs#L272
    //
    // BTW scrolling in jerky now, because you are ignoring HOW MUCH the wheel has turned. Do
    // include this information into `ScrollMessage`.
    //
    // Go through all events in the buffer, calculate the total delta, and then apply it.
    // This way, you won't lose too much precision.

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