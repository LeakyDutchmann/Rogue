use super::*;

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &Interaction), Changed<Interaction>>,
    time: Res<Time>,
    mut ui_click_track: ResMut<UiClickTrack>,
    mut writer: MessageWriter<UiClick>,
    mut console: ResMut<Console>,
) {
    let mut ctrl_pressed: bool = false;
    let mut shift_pressed: bool = false;
    if keys.pressed(KeyCode::ControlLeft) {
        ctrl_pressed = true;
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        shift_pressed = true;
    }
    for (entity, interaction) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let now = time.elapsed_secs_f64();
            if now - ui_click_track.last >= 0.2   {
                writer.write( UiClick {
                    entity: entity,
                    double: false,
                    shift_pressed,
                    ctrl_pressed,
                });
                console.log(format!("logged single with shift: {:?}, ctrl: {:?}", shift_pressed, ctrl_pressed));
            } else {
                writer.write( UiClick {
                    entity: entity,
                    double: true,
                    shift_pressed,
                    ctrl_pressed,
                });
                console.log(format!("logged double with shift: {:?}, ctrl: {:?}", shift_pressed, ctrl_pressed));
            }
            ui_click_track.last = now;
        }
    }
}