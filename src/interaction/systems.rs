use super::*;

pub fn interact_with_structure(
    keys: Res<ButtonInput<KeyCode>>,
    player_transform: Res<PlayerTransform>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        let player_pos = player_transform.0.translation;
        let cell_x = player_pos.x.floor() as usize;
        let cell_y = player_pos.y.floor() as usize;
    }
}