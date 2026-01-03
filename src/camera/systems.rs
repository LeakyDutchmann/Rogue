




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