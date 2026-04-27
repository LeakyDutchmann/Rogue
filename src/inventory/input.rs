use super::*;
    
pub fn pick_active_slot(
    mut reader: MessageReader<KeyPressed>,
    mut active_slot: Query<&mut ActiveSlot, With<Player>>,
) {
    if let Ok(mut slot) = active_slot.single_mut() {
        for msg in reader.read() {
            let mut active = None;
            match msg.key {
                KeyCode::Digit1 => active = Some(0),
                KeyCode::Digit2 => active = Some(1),
                KeyCode::Digit3 => active = Some(2),
                KeyCode::Digit4 => active = Some(3),
                KeyCode::Digit5 => active = Some(4),
                KeyCode::Digit6 => active = Some(5),
                KeyCode::Digit7 => active = Some(6),
                KeyCode::Digit8 => active = Some(7),
                KeyCode::Digit9 => active = Some(8),
                _ => {}
            }
            if let Some(active) = active {
                slot.index = active;
            }
        }   
    }
}

pub fn inventory_interactions(
    keys: Res<ButtonInput<KeyCode>>,
    mut slots: Query<(&Children, &Interaction), Changed<Interaction>>,
    mut slot: Query<&SlotIcon>,
    mut writer: MessageWriter<SlotClicked>,
    mut writer_double: MessageWriter<DoubleClicked>,
    mut ui_click_track: ResMut<UiClickTrack>,
    time: Res<Time>,
) {
    for (children, interaction) in slots.iter_mut() {
        if *interaction == Interaction::Pressed {
            let now = time.elapsed_secs_f64();
            for child in children.iter() {
                if let Ok(slot) = slot.get_mut(child) {
                    if now - ui_click_track.last >= 0.2 {
                        if keys.pressed(KeyCode::ShiftLeft) {
                            println!("clicked single");
                            break;
                        } else if keys.pressed(KeyCode::ControlLeft) {
                            println!("clicked single");
                            break;
                        } else {
                            println!("clicked single");
                            break;
                        }
                        
                    } else  {
                        println!("clicked double");
                        break;
                    }    
                }
            }
        }
    }
} 

pub fn background_interactions(
    mut query: Query<&Interaction, (Changed<Interaction>, With<UiBackground>)>,
    mut writer: MessageWriter<DropFromCursor>,
    inventory: Res<InventoryOpen>,
    cursor: Res<CursorWorldPos>,
    player: Res<PlayerTransform>,
) {
    for interaction in query.iter_mut() {
        if inventory.0 {
            if *interaction == Interaction::Pressed {
                if let Some(cursor_pos) = cursor.0 {
                    let player_pos = player.0.translation.truncate();
                    let to_cursor = (cursor_pos - player_pos).normalize();
                    writer.write(DropFromCursor {
                        direction: to_cursor,
                    });
                }
            }
        }
    }
}

