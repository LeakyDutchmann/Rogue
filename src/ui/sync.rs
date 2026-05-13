use super::*;

pub fn sync_ui_slot(
    slot: &mut UiSlot,
    slot_e: Entity,
    item_stack: &ItemStack,
    children: &Children,
    image_node: &Query<&ImageNode>,
    text: &mut Query<&mut Text>,
    item_reg: &ItemRegistry,
    commands: &mut Commands,
) {
    if let Some(item) = &item_stack.item_stored {
        if item_stack.item_stored != slot.item {
            for child in children.iter() {
                if let Ok(_) = image_node.get(child) {
                    commands.entity(child).despawn();
                }
                if let Ok(mut text) = text.get_mut(child) {
                    if item_stack.quantity == 0 {
                        text.0 = "".to_string();
                    } else {
                         text.0 = item_stack.quantity.to_string();
                    }
                   
                }
            }
            if let Some(item_def) = item_reg.items.get(item) {
                let child = commands.spawn(
                    ImageNode::new(item_def.icon.clone())
                ).id();
                commands.entity(slot_e).add_child(child);
                slot.item = item_stack.item_stored.clone();
                slot.quantity = item_stack.quantity as usize;
            } 
        } else {
            for child in children.iter() {
                if let Ok(mut text) = text.get_mut(child) {
                    if item_stack.quantity == 0 {
                        text.0 = "".to_string();
                    } else {
                         text.0 = item_stack.quantity.to_string();
                    }
                    slot.quantity = item_stack.quantity as usize;
                }
            }
        }
    } else {
        for child in children.iter() {
            if let Ok(_) = image_node.get(child) {
                commands.entity(child).despawn();
            }
            if let Ok(mut text) = text.get_mut(child) {
                text.0 = "".to_string();
                slot.item = item_stack.item_stored.clone();
                slot.quantity = item_stack.quantity as usize;
            }
        }
    }
}

pub fn sync_oven_ui(
    mut commands: Commands,
    processing: Query<&Processing>,
    interact_state: Res<InteractionState>,
    item_reg: Res<ItemRegistry>,
    mut slot: Query<(Entity, &mut UiSlot, &Children)>,
    mut text: Query<&mut Text>,
    entity: Query<Entity>,
    children: Query<&Children>,
    image_node: Query<&ImageNode>,
) {
    if interact_state.interacting != InteractionStage::Interacting {
        return;
    }
    if let Some(oven_entity) = interact_state.entity {
        if let Ok(processing) = processing.get(oven_entity) {
            for (slot_e, mut slot, children) in slot.iter_mut() {
                match slot.kind {
                    UiSlotKind::Output => {
                        if let Some(item_stack) = processing.output.get(slot.index) {
                            sync_ui_slot(
                                &mut slot,
                                slot_e,
                                item_stack,
                                children,
                                &image_node,
                                &mut text,
                                &item_reg,
                                &mut commands,
                            );
                        }
                    }
                    UiSlotKind::Input => {
                        if let Some(item_stack) = processing.input.get(slot.index) {
                            sync_ui_slot(
                                &mut slot,
                                slot_e,
                                item_stack,
                                children,
                                &image_node,
                                &mut text,
                                &item_reg,
                                &mut commands,
                            );
                        }
                    }
                    _ => {}
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
                } else {
                }
            }
        } 
    }
}

pub fn sync_chest_ui(
    mut commands: Commands,
    interact_state: Res<InteractionState>,
    item_reg: Res<ItemRegistry>,
    mut chest_slots: Query<(Entity, &mut UiSlot, &Children)>,
    chest: Query<&Chest>,
    mut text: Query<&mut Text>,
    image_node: Query<&ImageNode>,
) {
    if interact_state.interacting != InteractionStage::Interacting {
        return;
    }
    if let Ok(chest) = chest.get(interact_state.entity.unwrap()) {
        for (slot_e, mut slot_data, children) in chest_slots.iter_mut() {
            if slot_data.kind != UiSlotKind::Chest {
                continue;
            }
            if let Some(item_stack) = chest.items.get(slot_data.index) {
                sync_ui_slot(
                    &mut slot_data,
                    slot_e,
                    item_stack,
                    children,
                    &image_node,
                    &mut text,
                    &item_reg,
                    &mut commands,
                );
                
            } 
        }
    }
}