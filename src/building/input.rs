use super::*; 

pub fn builder_ui_interactions(
    mut slots: Query<(Entity, &Interaction, &Children, &mut BorderColor), (With<BuildingUiNode>, Changed<Interaction>)>,
    builder_slots: Query<&BuildingUiSlot>,
    mut structure_cursor: Query<&mut CursorStructureCarrier>,
    mut builder_mode: ResMut<BuildingMode>,
    mut ui_click: ResMut<UiClickTrack>,
    time: Res<Time>,
) {
    for (entity, interaction, children, mut border) in slots.iter_mut() {
        if interaction == &Interaction::Pressed {
            // *border = BorderColor::all(Color::srgb(2.0, 2.0, 2.0));
            for child in children {
                if let Ok(building_slot) = builder_slots.get(*child) {
                    if let Some(structure) = &building_slot.structure {
                        if let Ok(mut cursor) = structure_cursor.single_mut() {
                            cursor.structure = Some(structure.clone());
                            builder_mode.state = BuildingState::Placing;
                            println!("pressed: {}", structure);
                            ui_click.last = time.elapsed_secs_f64();
                        }
                    }
                }
            }
            println!("pressed");
        } 
    }
}
