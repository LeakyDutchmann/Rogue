use super::*;

pub fn find_path_ai_system(
    enemies: Query<(Entity, &EnemyState, &Transform), Without<AiPath>>,
    mut writer: MessageWriter<FindPath>,
    player_tf: Single<&Transform, With<Player>>,
) {
    let player_pos = player_tf.translation.truncate();
    for (entity, state, tf) in enemies.iter() {
        if state.current != EnemyStateType::Pathfinding {
            continue;
        }
        let enemy_pos = tf.translation.truncate();
        writer.write(FindPath {
            seeker: entity,
            seeker_pos: enemy_pos,
            target_pos: player_pos,
        });
    }
}

pub fn generate_trial(
    mut commands: Commands,
    shared: Res<SharedBounds>,
    mut reader: MessageReader<FindPath>,
) {
    for msg in reader.read() {
        let player_pos = msg.target_pos;
        let enemy_pos = msg.seeker_pos;
        let start = Position {
            x: (enemy_pos.x / CELL_SIZE).round() as i32,
            y: (enemy_pos.y / CELL_SIZE).round() as i32,
        };
        let goal = Position {
            x: (player_pos.x / CELL_SIZE).round() as i32,
            y: (player_pos.y / CELL_SIZE).round() as i32,
        };
        spawn_optimized_pathfinding_task(
                    &mut commands,
                    msg.seeker,
                    Arc::clone(&shared.0),  // cheap! no HashSet copy
                    start,
                    goal,
                    200,
                );
    }
}

pub fn apply_pathfinding_to_ai(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut PathfindingTask)>,
    grid: Res<SharedBounds>,
) {
    let empty_cells_grid_pos = grid.0.read().unwrap();
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(task_entity).remove::<PathfindingTask>();
            if let Ok(path) = result {
                let mut steps: VecDeque<Vec2> = VecDeque::new();
                for position in path {
                    let x = position.x as f32 * CELL_SIZE;
                    let y = position.y as f32 * CELL_SIZE;
                    steps.push_front(Vec2::new(x, y));               
                }
                if let Ok(optimized) = optimize_path(&mut steps, &empty_cells_grid_pos) {
                    commands.entity(task_entity).insert( AiPath {
                        steps: optimized,
                    });
                } 
            } else if let Err(e) = result {
                println!("Pathfinding error: {:?}", e);
            }
        }
    }
}