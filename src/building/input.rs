use super::*; 

pub fn builder_ui_interactions(
    children: Query<&Children>,
    builder_slots: Query<&BuildingUiSlot>,
    mut structure_cursor: Query<&mut CursorStructureCarrier>,
    mut builder_mode: ResMut<BuildingMode>,
    mut reader: MessageReader<UiClick>,
) {
    for msg in reader.read() {
        if let Ok(children) = children.get(msg.entity) {
            for child in children {
                if let Ok(building_slot) = builder_slots.get(*child) {
                    if let Some(structure) = &building_slot.structure {
                        if let Ok(mut cursor) = structure_cursor.single_mut() {
                            cursor.structure = Some(structure.clone());
                            builder_mode.state = BuildingState::Placing;
                        }
                    }
                }
            }
        }
    }
}
