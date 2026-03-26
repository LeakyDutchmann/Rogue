use super::*;

pub fn toggle_building_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut building_mode: ResMut<BuildingMode>,
    mut inventory_open: ResMut<InventoryOpen>,
) {
    if keys.just_pressed(KeyCode::KeyB) {
        if building_mode.state == BuildingState::Off {
            building_mode.state = BuildingState::On;
            inventory_open.0 = true;
        } else if building_mode.state == BuildingState::On {
            building_mode.state = BuildingState::Off;
            inventory_open.0 = false;
        } 
    } 
    if keys.just_pressed(KeyCode::Escape) {
        if building_mode.state == BuildingState::Placing {
            building_mode.state = BuildingState::On;
        }
    }
}

pub fn set_building_ui_visibility(
    building_mode: Res<BuildingMode>,
    mut ui_node: Query<&mut Visibility, With<BuildingUiNode>>,
) {
    for mut visibility in ui_node.iter_mut() {
        *visibility = if building_mode.state == BuildingState::On {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
