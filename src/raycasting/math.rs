use super::*;

pub fn ray_hits_aabb(ray: &Ray2d, center: Vec2, half_size: Vec2) -> Option<f32> {
    let min = center - half_size;
    let max = center + half_size;

    let inv_dir = Vec2::new(1.0 / ray.direction.x, 1.0 / ray.direction.y);

    let t1 = (min.x - ray.origin.x) * inv_dir.x;
    let t2 = (max.x - ray.origin.x) * inv_dir.x;
    let t3 = (min.y - ray.origin.y) * inv_dir.y;
    let t4 = (max.y - ray.origin.y) * inv_dir.y;

    let tmin = t1.min(t2).max(t3.min(t4));
    let tmax = t1.max(t2).min(t3.max(t4));

    if tmax >= 0.0 && tmax >= tmin {
        Some(tmin.max(0.0))
    } else {
        None
    }
}

