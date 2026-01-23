use super::*;

pub fn animate_kick( mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity,&mut Transform, &mut KickAnimation)>,
) {
    for (entity, mut transform, mut anim) in query.iter_mut() {
        if anim.active {
            println!("kicking!");
            anim.progress += time.delta_secs() / anim.duration;
            
            anim.progress = anim.progress.clamp(0.0, 1.0);
            println!("stinking!");
            let angle = swing_angle(anim.progress, anim.max_angle);
            
            transform.rotation = Quat::from_rotation_z(angle);
            println!("fooling!");
            
            if anim.progress >= 1.0 {
                anim.active = false;
                anim.progress = 0.0;
                anim.duration = 0.0;
                println!("resetting!");
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
    mut hand: Query<&mut KickAnimation, With<HeldItem>>,
    mut reader: MessageReader<MouseClickEvent>,
) { 
    for msg in reader.read() {
        if let Ok(mut anim) = hand.single_mut() {
            println!("found animation!");
            if anim.active {
                continue;
            } else {
                anim.active = true;
                anim.duration = 0.2;
                println!("Kick animation started");
            }
        }
    }
}