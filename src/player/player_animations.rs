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
    player_query: Query<&Transform, (With<Player>, Without<AttackAnimation>)>,
) {
    for (mut transform, anim) in query.iter_mut() {
        for player_tf in player_query.iter() {
            if let Some(cursor_pos) = anim.target {
                let player_pos = player_tf.translation;
                let to_cursor = (cursor_pos - player_pos.xy()).normalize();
                let to_cursor_angle = to_cursor.to_angle();
                let start_angle = to_cursor_angle - anim.max_angle / 2.0;
                let end_angle = to_cursor_angle + anim.max_angle / 2.0;
                let angle = start_angle + (end_angle - start_angle) * anim.progress;
                let offset = 20.0;
                let point_x = offset * angle.cos();
                let point_y = offset * angle.sin();
                transform.translation.x = point_x;
                transform.translation.y = point_y;
            
                transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);

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