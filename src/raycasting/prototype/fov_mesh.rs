use super::*;

pub fn cast_fov_rays(
    mut player_query: Query<(&Transform, &mut FieldOfView), With<Player>>,
    coliders: Query<(&Transform, &Colider), Without<Player>>,
) {
    if let Ok((playet_tf, mut fov)) = player_query.single_mut() {
        let player_pos = playet_tf.translation.truncate();
        let rays = generate_rays(player_pos, 1.0);
        let mut hits: Vec<Vec2> = Vec::new();
        for ray in rays {
            //intersections! 
            let mut closest_hit: Option<Vec2> = None;
            let mut closest_distance = f32::MAX;
            for (transform, colider) in coliders.iter() {
                let colider_pos = transform.translation.truncate();
                if player_pos.distance(colider_pos) > 200.0 {
                    continue;
                }
                let hit_opt = match colider.shape {
                    ColiderShape::Circle { radius } => ray_hits_circle(&ray, colider_pos, radius)
                        .map(|pt| ray.origin + ray.direction * pt),
                    ColiderShape::Rectangle { width, height } => ray_hits_aabb(&ray, colider_pos, Vec2::new(width / 2.0, height / 2.0))
                        .map(|pt| ray.origin + ray.direction * pt)
                };
                if let Some(hit_point) = hit_opt {
                    let distance = player_pos.distance(hit_point);
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_hit = Some(hit_point);
                    }
                }
            }
            if let Some(hit) = closest_hit {
                hits.push(hit);
            }
        }
        sort_points_by_angle(player_pos, &mut hits);
        let mut triangles: Vec<(Vec3, Vec3, Vec3)> = Vec::new();
        let p1 = player_pos;
        for i in 0..hits.len() {
            let p2 = hits[i];
            let p3 = hits[(i + 1) % hits.len()];
            triangles.push((Vec3::from(p1.extend(0.0)), Vec3::from(p2.extend(0.0)), Vec3::from(p3.extend(0.0))));
            // gizmos.line_2d(p1, p2, Color::WHITE);
            // gizmos.line_2d(p2, p3, Color::WHITE);
            // gizmos.line_2d(p3, p1, Color::WHITE);
        }
        fov.triangles = Some(triangles);
    }
}

pub fn update_fov_mesh(
    parent_qr: Query<(Entity, &FieldOfView, &Children, &GlobalTransform)>,
    fov_shape: Query<&CustomShape>,
    mut mesh_qr: Query<&mut Mesh2d>,
    shape: Query<Entity, With<CustomShape>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (parent, fov, children, parent_transform) in parent_qr.iter() {
        println!("Updating fov");
        if let Some(triangles) = &fov.triangles {
            let world_to_local = parent_transform.affine().inverse();
            let local_triangles: Vec<_> = triangles
                .iter()
                .map(|(a, b, c)| (
                    world_to_local.transform_point3(*a),
                    world_to_local.transform_point3(*b),
                    world_to_local.transform_point3(*c),
                ))
                .collect();
            let mut has_fov_shape = false;
            
            for child in children.iter() {
                if fov_shape.get(child).is_ok() {
                    has_fov_shape = true;
                    if let Ok(mesh2d) = mesh_qr.get(child) {
                        if let Some(mesh_asset) = meshes.get_mut(&mesh2d.0) {
                            *mesh_asset = build_mesh_from_triangles(local_triangles.clone());
                            break;
                        }
                    }
                }
            }
            
            if has_fov_shape {
                continue;
            }
            let shape = build_mesh_from_triangles(local_triangles.clone());
            let child = commands.spawn((
                Mesh2d(meshes.add(shape)),
                MeshMaterial2d(materials.add(
                    ColorMaterial {
                        color: Color::srgba(0.8, 0.9, 1.0, 0.25),
                        alpha_mode: AlphaMode2d::Blend,
                        ..Default::default()
                    }
                )),
                Transform::from_translation(Vec3::ZERO),
                CustomShape,
            )).id();
            
            commands.entity(parent).add_child(child);
        }
        
    }
}