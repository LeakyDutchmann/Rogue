use super::*;

pub fn animate_kick( mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity,&mut Transform, &mut KickAnimation)>,
) {
    for (entity, mut transform, mut anim) in query.iter_mut() {
        if anim.active {

            anim.progress += time.delta_secs() / anim.duration;
            
            anim.progress = anim.progress.clamp(0.0, 1.0);
            let angle = swing_angle(anim.progress, anim.max_angle);
            
            transform.rotation = Quat::from_rotation_z(angle);
            
            if !anim.impact_triggered && anim.progress >= 0.5 {
                anim.impact_triggered = true;
                println!("Impact triggered!");
            }
            
            if anim.progress >= 1.0 {
                anim.active = false;
                anim.progress = 0.0;
                anim.duration = 0.0;
                anim.impact_triggered = false;
            } 
        }
    } 
} 


fn swing_angle(progress: f32, max_angle_rad: f32) -> f32 {
let p = progress.clamp(0.0, 1.0);
// 0..1..0 
let t = if p <= 0.5 {
    p * 2.0 
} else {
    (1.0 - p) * 2.0 
};
t * max_angle_rad
}

pub fn start_kick(
    mut hand: Query<(&mut KickAnimation, &HeldItem), With<HeldItem>>,
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
                    anim.impact_triggered = false;
                    anim.target = Some(*click_pos);
                    anim.item = item;
                }
            }
        }
        
    }
}

//NOTE you successfully started the kick animation, but now you have to define a impact msg and send it when impact triggered, "
// in there you should place entity of item and pos of click. In destruction system, or A-la "apply dmg system" you have to 
// calculate who gets damaged and how much damage they take based on item properties which you are also going to imlement,