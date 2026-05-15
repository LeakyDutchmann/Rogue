use super::*;

pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(Entity, &Interaction), Without<UiBackground>>,
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

pub fn ui_slot_click_handler(
    mut reader: MessageReader<UiClick>,
    mut inventory: Query<&mut Inventory, With<Player>>,
    slot: Query<&UiSlot>,
    mut cursor_carrier: Query<&mut CursorCarrier>,
    item_reg: Res<ItemRegistry>,
    interaction_state: Res<InteractionState>,
    mut chest: Query<&mut Chest>,
    mut processing: Query<&mut Processing>,
    mut writer: MessageWriter<QuickMoveFromContainer>,
) {
    for msg in reader.read() {
        if let Ok(uislot) = slot.get(msg.entity) {
            let mut cursor_c = cursor_carrier.single_mut().unwrap();
            match uislot.kind {
                UiSlotKind::Inventory => {
                    if let Ok(mut player_inventory) = inventory.single_mut() {
                        if let Some(item_stack) = player_inventory.items.get_mut(uislot.index) {
                            if msg.shift_pressed {
                                writer.write({
                                    QuickMoveFromContainer {
                                        container: ContainerType::Inventory,
                                        index: uislot.index,
                                    }
                                });
                            } else {
                                 handle_slot_interaction(&mut cursor_c, item_stack, &item_reg, msg);
                            }
                        }
                    }
                }
                UiSlotKind::Chest => {
                    if let Ok(mut chest) = chest.get_mut(interaction_state.entity.unwrap()) {
                        if let Some(item_stack) = chest.items.get_mut(uislot.index) {
                            if msg.shift_pressed {
                                writer.write({
                                    QuickMoveFromContainer {
                                        container: ContainerType::Chest { entity: interaction_state.entity.unwrap() },
                                        index: uislot.index,
                                    }
                                });
                            } else {
                                 handle_slot_interaction(&mut cursor_c, item_stack, &item_reg, msg);
                            }
                        }
                    }
                }
                UiSlotKind::Output => {
                    if let Ok(mut processing) = processing.get_mut(interaction_state.entity.unwrap()) {
                        if let Some(item_stack) = processing.output.get_mut(uislot.index) {
                            if msg.shift_pressed {
                                writer.write({
                                    QuickMoveFromContainer {
                                        container: ContainerType::Output { entity: interaction_state.entity.unwrap() },
                                        index: uislot.index,
                                    }
                                });
                            } else {
                                 handle_slot_interaction(&mut cursor_c, item_stack, &item_reg, msg);
                            }
                        }
                    }
                }
                UiSlotKind::Input => {
                    if let Ok(mut processing) = processing.get_mut(interaction_state.entity.unwrap()) {
                        if let Some(item_stack) = processing.input.get_mut(uislot.index) {
                            if msg.shift_pressed {
                                writer.write({
                                    QuickMoveFromContainer {
                                        container: ContainerType::Input { entity: interaction_state.entity.unwrap() },
                                        index: uislot.index,
                                    }
                                });
                            } else {
                                 handle_slot_interaction(&mut cursor_c, item_stack, &item_reg, msg);
                            }
                        }
                    }
                }
            }
        }
    }
}