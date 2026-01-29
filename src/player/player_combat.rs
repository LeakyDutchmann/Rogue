use bevy::ecs::system::command;

use super::*;

pub fn initialize_attack(
    mut commands: Commands,
    mut hand: Query<(Entity, &HeldItem), (With<HeldItem>, Without<AttackAnimation>)>,
    mut reader: MessageReader<MouseClickEvent>,
) { 
    for click in reader.read() {
        if let MouseClickEvent::LeftClick(click_pos) = click {
            if let Ok((hend_e, item)) = hand.single_mut() {
                let item = item.last_held;
                commands.entity(hend_e).insert(
                    AttackAnimation {
                        progress: 0.0,      // 0..1
                        duration: 0.2,      // seconds
                        max_angle: std::f32::consts::PI / 4.0,     // radians
                        hit_triggered: false,
                        target: Some(*click_pos),
                        item: item,
                    }
                );
            }
        }
        
    }
}


