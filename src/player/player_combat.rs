use super::*;

pub fn initialize_attack(
    mut commands: Commands,
    mut hand: Query<(Entity, &HeldItem, &GlobalTransform), (With<HeldItem>, Without<AttackAnimation>)>,
    attack_stats: Query<&CombatStats>,
    mut reader: MessageReader<MouseClickEvent>,
) { 
    for click in reader.read() {
        if let MouseClickEvent::LeftClick(click_pos) = click {
            if let Ok((hend_e, item, transform)) = hand.single_mut() {
                if let Some(item) = item.last_held  {
                    if let Ok(stats) = attack_stats.get(item) {
                        commands.entity(hend_e).insert(
                            AttackAnimation {
                                progress: 0.0,
                                duration: 60.0 / stats.attack_speed,
                                max_angle: stats.swing_angle,
                                hit_triggered: false,
                                target: Some(*click_pos),
                                item: Some(item),
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


