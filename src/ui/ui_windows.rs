use super::*;

pub fn show_structure_window(
    mut commands: Commands,
    mut interaction_state: ResMut<InteractionState>,
    mut console: ResMut<Console>,
    ui_reg: Res<UiWindowRegistry>,
    asset_server: Res<AssetServer>,
    mut writer: MessageWriter<UiWindowSpawned>,
) {
    if interaction_state.interacting == InteractionStage::Intializing {
        if let Some(ui_window_id) = &interaction_state.ui_window_id {
            assemble_ui(ui_window_id.clone(), &ui_reg, &mut commands, &asset_server);
            interaction_state.interacting = InteractionStage::Interacting;
            writer.write(UiWindowSpawned);
        } else {
            console.log(format!("No Ui window id"));
        }
    } 
}

pub fn close_window(
    mut commands: Commands,
    mut window: Query<Entity, With<UiStructureWindow>>,
    mut reader: MessageReader<CloseWindowRequest>,
    mut console: ResMut<Console>,
) {
    for _ in reader.read() {
        console.log(format!("despawning"));
        for entity in window.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}