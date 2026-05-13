use super::*;

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(Entity, &Interaction), (Without<UiBackground>)>,
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
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed  {
            let kind = if mouse.just_pressed(MouseButton::Left) {
                ClickKind::LMB
            } else if mouse.just_pressed(MouseButton::Right) {
                ClickKind::RMB
            } else {
                continue;
            };
            let now = time.elapsed_secs_f64();
            if now - ui_click_track.last >= 0.2   {
                writer.write( UiClick {
                    kind: kind.clone(),
                    entity: entity,
                    double: false,
                    shift_pressed,
                    ctrl_pressed,
                });
                ui_click_track.last = now;
                console.log(format!("logged single, kind: {:?}", kind.clone()));
            } else {
                writer.write( UiClick {
                    kind,
                    entity: entity,
                    double: true,
                    shift_pressed,
                    ctrl_pressed,
                });
                ui_click_track.last = now;
                console.log(format!("logged double"));
            }
            
        }
    }
}