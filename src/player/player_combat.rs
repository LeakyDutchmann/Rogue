use super::*;

pub fn initialize_attack(
    mut commands: Commands,
    player_qr: Query<Entity, With<Player>>,
    hand: Query<(Entity, &HeldItem, &ChildOf), (With<HeldItem>, Without<AttackAnimation>)>,
    mut reader: MessageReader<MouseClickEvent>,
    registry: Res<ItemRegistry>,
) { 
    for click in reader.read() {
        if let Ok(player_e) = player_qr.single() {
            if let MouseClickEvent::LeftClick(click_pos) = click {
                for (hend_e, helditem, childof) in hand.iter() {
                    if player_e != childof.0 {
                        continue
                    }
                    if let Some(item) = helditem.held.as_ref()  {
                        if let Some(def) = registry.items.get(item) {
                            if let Some(stats) = &def.combat_stats {
                                if let Some(animation_style) = def.animation_style {
                                    commands.entity(hend_e).insert(
                                        AttackAnimation {
                                            anim_pattern: animation_style,
                                            progress: 0.0,
                                            duration: 60.0 / stats.attack_speed as f32,
                                            max_angle: (stats.swing_angle as f32).to_radians(),
                                            hit_triggered: false,
                                            cursor_pos: *click_pos,
                                            item_radius: stats.radius as f32,
                                            item: item.clone(),
                                        }
                                    );
                                }
                                
                            }
                        }
                    }   
                }
            }
        }
    }
}


