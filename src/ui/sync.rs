use super::*;

pub fn sync_oven_ui(
    mut commands: Commands,
    mut processing: Query<&Processing>,
    interact_state: Res<InteractionState>,
    item_reg: Res<ItemRegistry>,
    mut input: Query<(Entity, &mut OvenInputSlot, &ChildOf)>,
    mut output: Query<(Entity, &mut OvenOutputSlot, &ChildOf)>,
    mut text: Query<&mut Text, With<SlotCounter>>,
    entity: Query<Entity>,
    children: Query<&Children>,
    mut writer: MessageWriter<UiSlotUpdate>,
) {
    if interact_state.interacting != InteractionStage::Interacting {
        return;
    }
    if let Some(oven_entity) = interact_state.entity {
        if let Ok(processing) = processing.get(oven_entity) {
            for (slot_e, mut slot, child_of) in input.iter_mut() {
                if let Some(item_stack) = processing.input.get(slot.index) {
                    if let Some(item) = &item_stack.item_stored {
                        if Some(item) != slot.item.as_ref() {
                            if let Some(def) = item_reg.items.get(item) {
                                commands.entity(slot_e).insert(ImageNode::new(def.icon.clone()));
                                slot.item = Some(item.clone())
                            }
                        }
                    } else {
                        commands.entity(slot_e).remove::<ImageNode>();
                        slot.item = None;
                    }
                }
                if let Ok(entity) = entity.get(child_of.0) {
                    if let Ok(children) = children.get(entity) {
                        for child in children.iter() {
                            if let Ok(mut text) = text.get_mut(child) {
                                if processing.input[0].quantity > 0 {
                                    text.0 = processing.input[0].quantity.to_string();
                                } else {
                                    text.0 = "".to_string();
                                }
                            }
                        }
                    }
                }
            }
            for (slot_e, mut slot, child_of) in output.iter_mut() {
                if let Some(item_stack) = processing.output.get(slot.index) {
                    if let Some(item) = &item_stack.item_stored {
                        if Some(item) != slot.item.as_ref() {
                            if let Some(def) = item_reg.items.get(item) {
                                commands.entity(slot_e).insert(ImageNode::new(def.icon.clone()));
                                slot.item = Some(item.clone())
                            }
                        }
                    } else {
                        commands.entity(slot_e).remove::<ImageNode>();
                        slot.item = None;
                    }
                }
                if let Ok(entity) = entity.get(child_of.0) {
                    if let Ok(children) = children.get(entity) {
                        for child in children.iter() {
                            if let Ok(mut text) = text.get_mut(child) {
                                if processing.output[0].quantity > 0 {
                                    text.0 = processing.output[0].quantity.to_string();
                                } else {
                                    text.0 = "".to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
