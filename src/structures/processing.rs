use super::*;

pub fn tick_oven_timers(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Processing)>,
    mut writer: MessageWriter<UpdateProcessing>,
    
) {
    for (entity, mut processing) in &mut query {
        if processing.input[0].item_stored.is_none() {
            processing.timer.reset();
            continue;
        }
        processing.timer.tick(time.delta());
        if processing.timer.is_finished() {
            processing.timer.reset();
            writer.write(UpdateProcessing {
                oven_entity: entity
            });
        }
    }
}

pub fn update_processing(
    mut query: Query<&mut Processing>,
    mut reader: MessageReader<UpdateProcessing>,
    item_reg: Res<ItemRegistry>,
    mut console: ResMut<Console>,
    recipe_reg: Res<OvenRecipeRegistry>,
) {
    for msg in reader.read() {
        if let Ok(mut processing) = query.get_mut(msg.oven_entity) {
            if let Some(item) = &processing.input[0].item_stored {
                if let Some(item_def) = item_reg.items.get(item) {
                    if let Some(recipe) = recipe_reg.recipes.get(item) {
                        if processing.output[0].quantity < item_def.max_stack as i32 {
                            if processing.input[0].quantity > 0 {
                                if processing.output[0].item_stored.is_none() {
                                    processing.output[0].item_stored = Some(recipe.output.clone());
                                }
                                if processing.output[0].item_stored != Some(recipe.output.clone()) {
                                    continue;
                                }
                                processing.output[0].quantity += 1;
                                processing.input[0].quantity -= 1;
                                if processing.input[0].quantity == 0 {
                                    processing.input[0].item_stored = None;
                                }
                                console.log(format!("processed one item"))
                            } else {
                                processing.input[0].item_stored = None;
                                console.log(format!("input empty"))
                            }
                        } else {
                            console.log(format!("output full"))
                        }
                    }
                }
            } else {
                console.log(format!("input empty"))
            }
            
        }
    }
}
