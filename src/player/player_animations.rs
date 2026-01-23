use super::*;

pub fn animate_kick( mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity,&mut Transform, &mut KickAnimation)>,
) {
    // for (entity, mut transform, mut anim) in query.iter_mut() {
    //     anim.progress += time.delta_secs() / anim.duration;
        
    //     anim.progress = anim.progress.clamp(0.0, 1.0);
        
    //     let angle = swing_angle(anim.progress, anim.max_angle);
        
    //     transform.rotation = Quat::from_rotation_z(angle);
        
    //     if anim.progress >= 1.0 {
    //         anim.active = false; 
    //     } 
    // } 
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
    
) {}