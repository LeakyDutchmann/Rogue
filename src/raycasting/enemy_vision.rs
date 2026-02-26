use super::*;

pub fn enemy_vision_system(
    mut commands: Commands,
    mut enemies: Query<(Entity,&mut EnemyVision, &Transform), With<Enemy>>,
    player: Query<&Transform, With<Player>>,
    wall_qr: Query<(&Transform, &Colider), With<Wall>>,
    world: Res<WorldGrid>,
    mut gizmos: Gizmos,
) {
    for (enemy_e, mut vision, enemy_tf) in enemies.iter_mut() {
        for player_tf in player.iter() {
            let enemy_pos = enemy_tf.translation.truncate();
            let player_pos = player_tf.translation.truncate();
            let d = (enemy_pos - player_pos).length_squared();
            if d > vision.fov_radius * vision.fov_radius {
                continue
            }
            let cell_x = (enemy_pos.x / CELL_SIZE ).floor() as i32;
            let cell_y = (enemy_pos.y / CELL_SIZE ).floor() as i32;
            let central_cell = (cell_x, cell_y);
            let mut cells = get_cells_in_radius(central_cell, vision.fov_radius);
            let entities = get_entities_in_cells(cells, &world);
            let mut coliders_to_check: Vec<(&Transform, &Colider)> = Vec::new();
            for entity in entities {
                if let Ok((tf, colider)) = wall_qr.get(entity) {
                    coliders_to_check.push((tf, colider))
                }
            }
            let vec_to_player = (player_pos - enemy_pos).normalize();
            if let Ok(dir) = Dir2::new(vec_to_player) {
                let ray = Ray2d::new(enemy_pos, dir);
                let mut closest_hit: Option<Vec2> = None;
                let mut closest_distance = f32::MAX;
                for (colider_tf, colider) in coliders_to_check {
                    
                    let colider_pos = colider_tf.translation.truncate();
                    let hit_opt = match colider.shape {
                        ColiderShape::Circle { radius } => None,
                        ColiderShape::Rectangle { width, height } => ray_hits_aabb(&ray, colider_pos, Vec2::new(width / 2.0, height / 2.0))
                            .map(|pt| ray.origin + ray.direction * pt)
                    };
                    if let Some(hit_point) = hit_opt {
                        let distance = (hit_point - enemy_pos).length_squared();
                        if distance < closest_distance {
                            closest_distance = distance;
                            closest_hit = Some(hit_point);
                        }
                    } else {
                        continue
                    }
                }
                if let Some(hit) = closest_hit {
                    if closest_distance >= d - 0.01 {
                        vision.state = EnemyVisionState::Direct;
                        vision.last_seen_pos = Some(player_pos);
                        gizmos.line_2d(enemy_pos, player_pos, Color::WHITE);
                        commands.entity(enemy_e).remove::<AiPath>();
                    } else {
                        vision.state = EnemyVisionState::PathFinding;
                    }
                } else if closest_hit.is_none() {
                    vision.state = EnemyVisionState::Direct;
                    vision.last_seen_pos = Some(player_pos);
                    gizmos.line_2d(enemy_pos, player_pos, Color::WHITE);
                    commands.entity(enemy_e).remove::<AiPath>();
                }
            }
        }
            
    }
}
