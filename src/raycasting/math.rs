use super::*;

pub fn build_mesh_from_triangles(triangles: Vec<(Vec3, Vec3, Vec3)>) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for (a, b, c) in triangles {
        for vertex in [a, b, c] {
            // Check if this vertex already exists
            let existing = positions.iter().position(|p| {
                (p[0] - vertex.x).abs() < f32::EPSILON &&
                (p[1] - vertex.y).abs() < f32::EPSILON &&
                (p[2] - vertex.z).abs() < f32::EPSILON
            });

            if let Some(index) = existing {
                indices.push(index as u32);
            } else {
                indices.push(positions.len() as u32);
                positions.push([vertex.x, vertex.y, vertex.z]);
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

pub fn ray_hits_circle(ray: &Ray2d, center: Vec2, radius: f32) -> Option<f32> {
    let oc = center - ray.origin;
    let t = oc.dot(*ray.direction); // проєкція на напрямок
    if t < 0.0 {
        return None; // коло позаду
    }
    let closest = ray.origin + *ray.direction * t;
    if closest.distance(center) <= radius {
        Some(t) // hit на відстані t
    } else {
        None
    }
}

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