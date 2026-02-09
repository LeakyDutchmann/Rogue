use super::*;

pub fn attack_progression_system(
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