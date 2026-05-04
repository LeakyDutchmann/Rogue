use super::*;

pub fn show_structure_window(
    mut commands: Commands,
    mut interaction_state: ResMut<InteractionState>,
    mut writer: MessageWriter<UiForceSync>,
    mut console: ResMut<Console>,
    ui_reg: Res<UiWindowRegistry>,
    asset_server: Res<AssetServer>,
) {
    if interaction_state.interacting == InteractionStage::Intializing {
        if let Some(ui_window_id) = &interaction_state.ui_window_id {
            assemble_ui(ui_window_id.clone(), &ui_reg, &mut commands, &asset_server);
            writer.write(UiForceSync {
                oven_entity: interaction_state.entity.expect("No entity in interaction state"),
            });
            console.log(format!("Assembled UI for window: {}", ui_window_id));
            interaction_state.interacting = InteractionStage::Interacting;
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
    for msg in reader.read() {
        console.log(format!("despawning"));
        for entity in window.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}