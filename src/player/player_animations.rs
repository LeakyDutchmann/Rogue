use super::*;


pub fn attack_progression(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut AttackAnimation)>,
    mut writer: MessageWriter<HitMessage>,
) {
    for (entity, mut anim) in query.iter_mut() {
        anim.progress += time.delta_secs() / anim.duration;
        anim.progress = anim.progress.clamp(0.0, 1.0);
        
        if !anim.hit_triggered && anim.progress >= 0.5 {
            anim.hit_triggered = true;
            writer.write(HitMessage {
                item: anim.item,
                target: anim.target,
                item_radius: anim.item_radius,
                item_pos: anim.item_pos,
            });
            
            println!("Impact send!");
        }
        if anim.hit_triggered && anim.progress >= 1.0 {
            commands.entity(entity).remove::<AttackAnimation>();
        }
    } 
}

pub fn attack_animation(
    mut query: Query<(&mut Transform, &mut AttackAnimation)>,
) {
    for (mut transform, anim) in query.iter_mut() {
        let angle = swing_angle(anim.progress, anim.max_angle);
        
        transform.rotation = Quat::from_rotation_z(angle);
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