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
    mut query: Query<(&mut Transform, &mut AttackAnimation, &ChildOf, &mut Sprite)>,
    parent_tf: Query<&Transform, Without<AttackAnimation>>,
) {
    for (mut transform, anim, child_of, mut sprite) in query.iter_mut() {
        if let Ok(parent_tf) = parent_tf.get(child_of.0) {
            if let Some(cursor_pos) = anim.target {
                let player_pos = parent_tf.translation;
                let to_cursor = (cursor_pos - player_pos.xy()).normalize();
                let to_cursor_angle = to_cursor.to_angle();
                let offset = 20.0;
                match anim.anim_pattern {
                    AnimationStyle::Sword => {
                        if cursor_pos.x <= player_pos.x {
                           
                            let start_angle = to_cursor_angle - anim.max_angle / 2.0;
                            let end_angle = to_cursor_angle + anim.max_angle / 2.0;
                            let angle = start_angle + (end_angle - start_angle) * anim.progress;
                            
                            let point_x = offset * angle.cos();
                            let point_y = offset * angle.sin();
                            
                            sprite.flip_x = true;
                            transform.translation.x = point_x;
                            transform.translation.y = point_y;
                            
                            transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);  
                        } else {
                            
                            let start_angle = to_cursor_angle + anim.max_angle / 2.0;
                            let end_angle = to_cursor_angle - anim.max_angle / 2.0;
                            let angle = start_angle + (end_angle - start_angle) * anim.progress;
                            let point_x = offset * angle.cos();
                            let point_y = offset * angle.sin();
                            
                            sprite.flip_x = false;
                            transform.translation.x = point_x;
                            transform.translation.y = point_y;
                            
                            transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
                            
                        }
                    }
                    AnimationStyle::PickAxe => {
                        if cursor_pos.x <= player_pos.x {
                           
                            let start_angle = to_cursor_angle - anim.max_angle / 2.0;
                            let end_angle = to_cursor_angle + anim.max_angle / 2.0;
                            let angle = start_angle + (end_angle - start_angle) * anim.progress;                     
                            sprite.flip_x = true;
                            transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);  
                        } else {
                            
                            let start_angle = to_cursor_angle + anim.max_angle / 2.0;
                            let end_angle = to_cursor_angle - anim.max_angle / 2.0;
                            let angle = start_angle + (end_angle - start_angle) * anim.progress;                    
                            sprite.flip_x = false;                      
                            transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
                            
                        }
                    }
                }
                
                
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
