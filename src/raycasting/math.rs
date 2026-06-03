use super::*;

pub fn ray_hits_aabb_circlecast(
    ray: &Ray2d,
    center: Vec2,
    half_size: Vec2,
    radius: f32,
) -> Option<f32> {
    let expanded_half_size = half_size + Vec2::splat(radius);
    ray_hits_aabb(ray, center, expanded_half_size)
}



pub fn ray_hits_aabb(ray: &Ray2d, center: Vec2, half_size: Vec2) -> Option<f32> {
    let min = center - half_size;
    let max = center + half_size;
    let inv_dir = Vec2::new(1.0 / ray.direction.x, 1.0 / ray.direction.y);
    let inf = f32::INFINITY;
    
    if ray.direction.x != 0.0 && ray.direction.y != 0.0 {
        
    
        let t1 = (min.x - ray.origin.x) * inv_dir.x;
        let t2 = (max.x - ray.origin.x) * inv_dir.x;
        let t3 = (min.y - ray.origin.y) * inv_dir.y;
        let t4 = (max.y - ray.origin.y) * inv_dir.y;
    
        let enter_x = t1.min(t2);
        let exit_x = t1.max(t2);
        let enter_y = t3.min(t4);
        let exit_y = t3.max(t4);
    
        let tmin = enter_x.max(enter_y);
        let tmax = exit_x.min(exit_y);
        
        // if tmin < 0.01 {
        //     return None;
        // }
    
        if tmax >= 0.0 && tmax >= tmin {
            Some(tmin.max(0.0))
        } else {
            None
        }
    } else if ray.direction.x == 0.0 {
        if ray.origin.x >= min.x && ray.origin.x <= max.x {
            let t3 = (min.y - ray.origin.y) * inv_dir.y;
            let t4 = (max.y - ray.origin.y) * inv_dir.y;

            let enter_x = -inf;
            let exit_x = inf;
            let enter_y = t3.min(t4);
            let exit_y = t3.max(t4);

            let tmin = enter_x.max(enter_y);
            let tmax = exit_x.min(exit_y);

            if tmax >= 0.0 && tmax >= tmin {
                Some(tmin.max(0.0))
            } else {
                None
            }
        
        } else {
            None
        }
    } else {
        if ray.origin.y >= min.y && ray.origin.y <= max.y {
            let t1 = (min.x - ray.origin.x) * inv_dir.x;
            let t2 = (max.x - ray.origin.x) * inv_dir.x;

            let enter_x = t1.min(t2);
            let exit_x = t1.max(t2);
            let enter_y = -inf;
            let exit_y = inf;

            let tmin = enter_x.max(enter_y);
            let tmax = exit_x.min(exit_y);

            if tmax >= 0.0 && tmax >= tmin {
                Some(tmin.max(0.0))
            } else {
                None
            }
        } else {
            None
        }
    }
    
}

pub fn ray_hits_circle_circlecast(ray: &Ray2d, center: Vec2, radius: f32, agent_radius: f32) -> Option<f32> {
    let expanded_radius = radius + agent_radius;
    ray_hits_circle(ray, center, expanded_radius)
}

pub fn ray_hits_circle(ray: &Ray2d, center: Vec2, radius: f32) -> Option<f32> {
    let oc = center - ray.origin;
    let t = oc.dot(*ray.direction); 
    if t < 0.0 {
        return None;
    }
    let closest = ray.origin + *ray.direction * t;
    if closest.distance(center) <= radius {
        Some(t) 
    } else {
        None
    }
}

pub fn generate_rays(center: Vec2, step_deg: f32) -> Vec<Ray2d> {
    let mut rays: Vec<Ray2d> = Vec::new();
    let mut angle_deg: f32 = 0.0; // явно f32

    while angle_deg < 360.0_f32 {
        let angle_rad = angle_deg.to_radians(); // тепер компілятор знає, що це f32
        let direction = Vec2::new(angle_rad.cos(), angle_rad.sin());
        if let Ok(dir) = Dir2::new(direction) {
            let ray = Ray2d {
                origin: center,
                direction: dir,
            };
            rays.push(ray);
            angle_deg += step_deg;
        }
    }

    rays
}

pub fn sort_points_by_angle(origin: Vec2, points: &mut [Vec2]) {
    points.sort_by(|a, b| {
        let angle_a = (a.y - origin.y).atan2(a.x - origin.x);
        let angle_b = (b.y - origin.y).atan2(b.x - origin.x);

        angle_a.total_cmp(&angle_b)
    });
}