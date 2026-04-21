use super::*;

pub fn hover_system(
    mut query: Query<(Entity, &mut BorderColor, &Interaction), Without<UiBackground>>,
    mut hovering_state: ResMut<UiHoveringState>,
    time: Res<Time>,
) {
    let mut hovered: Option<Entity> = None;
    for (entity, mut border_color, interaction) in query.iter_mut() {
        if *interaction == Interaction::Hovered {
            *border_color = BorderColor::all(Color::srgb(1.0, 1.0, 1.0));
            hovered = Some(entity);
        } else if *interaction == Interaction::Pressed {
            *border_color = BorderColor::all(Color::srgb(1.0, 0.4, 0.0));
        } else {
            *border_color = BorderColor::all(Color::srgb(0.5, 0.5, 0.5));
        }
    }
    if let Some(entity) = hovered {
        if hovering_state.entity != Some(entity) {
            hovering_state.entity = Some(entity);
            hovering_state.last_time = time.elapsed_secs_f64();
        }
    } else {
        hovering_state.entity = None;
    }
}


pub fn tool_tip_follow_cursor(
    windows: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<&mut Node, With<ToolTip>>,
) {
    if let Ok(window) = windows.single() {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(mut node) = query.single_mut() {
                node.left = Val::Px(cursor_pos.x);
                node.top = Val::Px(cursor_pos.y);
            }
        }
    }
}

pub fn update_tool_tip(
    mut query: Query<(&mut Text, &mut Visibility), With<ToolTip>>,
    children: Query<&Children>,
    hovering_state: Res<UiHoveringState>,
    item_identificator: Query<&SlotIcon>,
    structure_identificator: Query<&BuildingUiSlot>,
    player_inventory: Query<&Inventory, With<Player>>,
    recipe_registry: Res<RecipeRegistry>,
    time: Res<Time>,
) {
    if let Ok((mut text, mut visibility)) = query.single_mut() {
        if let Ok(inventory) = player_inventory.single() {
            if let Some(entity) = hovering_state.entity {
                if let Ok(children) = children.get(entity) {
                    let mut structure_id: Option<String> = None;
                    let mut item_id_found: Option<String> = None;
                    for child in children.iter() {
                        if let Ok(structure_slot) = structure_identificator.get(child) {
                            if let Some(structure) = &structure_slot.structure {
                                structure_id = Some(structure.clone());
                                break;                   
                            } 
                        } else if let Ok(item_slot) = item_identificator.get(child) {
                            if let Some(item) = inventory.items.get(item_slot.index) {
                                if let Some(item_id) = &item.item_stored {
                                    if structure_id.is_none() {
                                        item_id_found = Some(item_id.clone());
                                    }
                                }
                            }
                        }
                    }
                    let now = time.elapsed_secs_f64();
                    if now - hovering_state.last_time > 0.15 {
                        if let Some(structure_id) = structure_id {
                            text.0 = structure_id.clone();
                            if let Some(recipe) = recipe_registry.recipes.get(&structure_id) {
                                for (item, amount) in &recipe.ingredients {
                                    text.0.push_str(&format!("\n{}: {}", item, amount));
                                }
                            }
                            *visibility = Visibility::Visible;
                        } else if let Some(item_id) = item_id_found {
                            text.0 = item_id;
                            *visibility = Visibility::Visible;
                        } else {
                            text.0 = "".to_string();
                            *visibility = Visibility::Hidden;
                        }
                    } else {
                        text.0 = "".to_string();
                        *visibility = Visibility::Hidden;
                    }
                    
                }  
            } else {
                text.0 = "".to_string();
                *visibility = Visibility::Hidden;
            }
        }
    } 
}