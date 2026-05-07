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

pub fn sync_work_bench_ui(
    mut commands: Commands,
    interact_state: Res<InteractionState>,
    item_reg: Res<ItemRegistry>,
    recipe_reg: Res<RecipeRegistry>,
    mut work_bench_slots: Query<(Entity, &mut WorkBenchSlot)>,
    work_bench: Query<&WorkBench>,
    mut console: ResMut<Console>,
    mut reader: MessageReader<UiWindowSpawned>,
) {
    for _ in reader.read() {
        if interact_state.interacting != InteractionStage::Interacting {
            return;
        }
        if let Ok(_work_bench_marker) = work_bench.get(interact_state.entity.unwrap()) {
            let mut all_recipes = recipe_reg.recipes.iter().map(|(k, v)| k.clone()).collect::<Vec<_>>();
            console.log(format!("{} recipies", all_recipes.len()));
            let mut slots: Vec<(Entity, Mut<WorkBenchSlot>)> =
                work_bench_slots.iter_mut().collect();
            
            slots.sort_by_key(|(_, slot)| slot.index);
            for ((slot_e, mut slot_data)) in slots {
                if let Some(item_id) = all_recipes.pop() {
                    let id_copy = item_id.clone();
                    if let Some(def) = item_reg.items.get(&item_id) {
                        slot_data.item = Some(item_id);
                        let child = commands.spawn(
                            ImageNode::new(def.icon.clone())
                        ).id();
                        console.log(format!("added item {}", id_copy));
                        commands.entity(slot_e).add_child(child);
                    } else {
                        console.log(format!("no item found for recipe: {}", item_id));
                    }
                }
            }
        } 
    }
}