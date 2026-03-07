use super::*;

pub fn initialize_attack(
    mut commands: Commands,
    player_qr: Query<Entity, With<Player>>,
    mut hand: Query<(Entity, &HeldItem, &GlobalTransform, &ChildOf), (With<HeldItem>, Without<AttackAnimation>)>,
    attack_stats: Query<(&CombatStats, &AnimationPattern)>,
    mut reader: MessageReader<MouseClickEvent>,
) { 
    for click in reader.read() {
        if let Ok(player_e) = player_qr.single() {
            if let MouseClickEvent::LeftClick(click_pos) = click {
                for ((hend_e, helditem, transform, childof)) in hand.iter() {
                    if player_e != childof.0 {
                        continue
                    }
                    if let Some(item) = helditem.last_held  {
                        if let Ok((stats, pattern)) = attack_stats.get(item) {
                            commands.entity(hend_e).insert(
                                AttackAnimation {
                                    anim_pattern: pattern.pattern,
                                    progress: 0.0,
                                    duration: 60.0 / stats.attack_speed,
                                    max_angle: stats.swing_angle,
                                    hit_triggered: false,
                                    cursor_pos: *click_pos,
                                    item: item,
                                    item_radius: stats.radius,
                                    item_pos: transform.compute_transform().translation.truncate(),
                                }
                            );
                        }
                    }   
                }
            }
        }
    }
}


