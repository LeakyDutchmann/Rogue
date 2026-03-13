use crate::mouse::*;


pub fn get_cursor_position(
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut cursor: ResMut<CursorWorldPos>,
) {
    let window = window_q.single().unwrap();
    let (camera, cam_transform) = camera_q.single().unwrap();
    cursor.0 = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
}

pub fn mouse_click_handler(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor: ResMut<CursorWorldPos>,
    mut writer: MessageWriter<MouseClickEvent>,
) {
    let Some(position) = cursor.0 else {
        return;
    };
    if buttons.pressed(MouseButton::Left) {
        writer.write(MouseClickEvent::LeftClick(position));
    }
}

pub fn scroll_events(
    mut scroll: MessageReader<MouseWheel>,
    mut writer: MessageWriter<ScrollMessage>,
) {
    for ev in scroll.read() {
        if ev.y == 1.0 {
            writer.write(ScrollMessage {
                event: ScrollDir::ScrollUp
            });
            println!("Scroll up");
        }
        if ev.y == -1.0 {
            writer.write(ScrollMessage {
                event: ScrollDir::ScrollDown
            });
        }
    }
}




