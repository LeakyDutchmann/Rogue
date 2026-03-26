use super::*;

pub fn toggle_building_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut building_mode: ResMut<BuildingMode>,
    mut inventory_open: ResMut<InventoryOpen>,
) {
    if keys.just_pressed(KeyCode::KeyB) {
        if building_mode.state == false {
            building_mode.state = true;
            inventory_open.0 = true;
        } else {
            building_mode.state = false;
            inventory_open.0 = false;
        }
    }
}

pub fn set_building_ui_visibility(
    building_mode: Res<BuildingMode>,
    mut ui_node: Query<&mut Visibility, With<BuildingUiNode>>,
) {
    for mut visibility in ui_node.iter_mut() {
        *visibility = if building_mode.state {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
