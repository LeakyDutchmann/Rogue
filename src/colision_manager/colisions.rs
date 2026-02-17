use crate::colision_manager::*;


pub fn resolve_movement(
    time: Res<Time>,
    mut commands: Commands,
    intender: Query<(Entity, &Transform, &MovementIntent, &Colider, &Speed), With<MovementIntent>>,
    others: Query<(&Transform, &Colider)>,
    world: ResMut<WorldGrid>
) {
    for (entity, transform, direction_intended, colider, speed) in intender.iter() {
        let current_pos = transform.translation.truncate();
        let delta = direction_intended.direction * speed.0 * time.delta_secs();
        let mut resolved_dir = Vec2::ZERO;
        let mut blocked_x = false;
        let mut blocked_y = false;
        let cell_x = (current_pos.x / CELL_SIZE ).floor() as i32;
        let cell_y = (current_pos.y / CELL_SIZE ).floor() as i32;
        let central_cell = (cell_x, cell_y);
        let neighbour_cells = get_cells_3x3(central_cell);
        
        let other_entities = get_entities_in_cells(neighbour_cells, &world);
        
        for other_entity in other_entities {
            if other_entity == entity { continue }
        
            match others.get(other_entity) {
                Ok((other_tf, other_col)) => {
                    let col_pos = other_tf.translation.truncate();
                    
                    if !blocked_x {
                        if delta.x != 0.0 {
                            let try_pos = current_pos + Vec2::new(delta.x, 0.0);
                            if colider.check_colisions(try_pos, other_col, col_pos) {
                                blocked_x = true;
                            }
                        }
                    }
                    if !blocked_y {
                        if delta.y != 0.0 {
                            let try_pos = current_pos + Vec2::new(0.0, delta.y); 
                            if colider.check_colisions(try_pos, other_col, col_pos) {
                                blocked_y = true;
                            } 
                        }
                    }
                }
                Err(_) => continue 
            }
        }
        if !blocked_x {
            resolved_dir.x = delta.x;
        }
        if !blocked_y {
            resolved_dir.y = delta.y;
        }
        if resolved_dir != Vec2::ZERO {
            commands.entity(entity).insert(MovementResolved{ direction: resolved_dir});
            println!("Movement resolved {:?}", resolved_dir);
        }
        
    
        commands.entity(entity).remove::<MovementIntent>();
    }
}



