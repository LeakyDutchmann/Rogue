use super::*;

pub fn start_kick(
    mut hand: Query<(&mut AttackAnimation, &HeldItem), With<HeldItem>>,
    mut reader: MessageReader<MouseClickEvent>,
) { 
    for click in reader.read() {
        if let MouseClickEvent::LeftClick(click_pos) = click {
            if let Ok((mut anim, item)) = hand.single_mut() {
                let item = item.last_held;
                if anim.active {
                    continue;
                } else {
                    anim.active = true;
                    anim.duration = 0.2;
                    anim.hit_triggered = false;
                    anim.target = Some(*click_pos);
                    anim.item = item;
                }
            }
        }
        
    }
}


