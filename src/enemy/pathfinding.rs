use super::*;

use bevy::color::palettes::basic::PURPLE;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn heuristic(&self, b: &Position) -> i32 {
        (self.x - b.x).abs() + (self.y - b.y).abs()
    }
}

#[derive(Clone, Debug)]
struct Node {
    position: Position,
    g: i32,
    h: i32,
    f: i32,
    parent: Option<Position>,
}


#[derive(Debug)]
pub enum PathfindingError {
    NoPath,
    StartNotInBounds,
    GoalNotInBounds,
}

pub fn find_path(start: Position, goal: Position, bounds: HashSet<Position> ) -> Result<Vec<Position>, PathfindingError> {
    if !bounds.contains(&start) {
        return Err(PathfindingError::StartNotInBounds)
    }
    if !bounds.contains(&goal) {
        return Err(PathfindingError::GoalNotInBounds)
    }
    let mut path: Vec<Position> = Vec::new();
    let mut to_search: HashMap<Position, Node> = HashMap::new();
    let mut processed: HashMap<Position, Node> = HashMap::new();
    to_search.insert(start.clone(), Node {
        position: start.clone(),
        g: 0,
        h: 0,
        f: 0,
        parent: None,
    });
    while !to_search.is_empty() {
        let mut current_node = None;
        let mut lowest_f = -1;
        for (position, node) in to_search.iter() {
            if lowest_f == -1 || node.f < lowest_f {
                current_node = Some(node);
                lowest_f = node.f;
            }
        }
        if let None = current_node {
            break;
        }
        let current_node = current_node.unwrap().clone();
        let mut current_position = current_node.position.clone();
        to_search.remove(&current_position);
        // Add n to the CLOSED list
        let g = current_node.g + 1;
        let h = current_position.heuristic(&goal);
        let f = g + h;
        processed.insert(current_position.clone(), Node {
            position: current_position.clone(),
            g: g,
            h: h,
            f: f,
            parent: current_node.parent,
        });
        if current_position == goal {
            let mut nodelist: Vec<Position> = vec![];
            loop {
                nodelist.push(current_position.clone());
                if let Some(node) = processed.get(&current_position) {
                    match &node.parent {
                        Some(parent_pos) => current_position = parent_pos.clone(),
                        None => break, // досягли старту
                    }
                }
            }
            path = nodelist; 
            break;
        }
        let mut neighbors: Vec<Position> = vec![];
            neighbors.push(Position { x: current_position.x + 1, y: current_position.y, });
            neighbors.push(Position { x: current_position.x - 1, y: current_position.y, });
            neighbors.push(Position { x: current_position.x, y: current_position.y + 1, });
            neighbors.push(Position { x: current_position.x, y: current_position.y - 1, });
        
            for neighbor in neighbors {
                if !bounds.contains(&neighbor) {
                    continue;
                }
                let h = neighbor.heuristic(&goal);
                let g = current_node.g + 1;
                let f = g + h;
                if to_search.contains_key(&neighbor) {
                    if g > to_search.get(&neighbor).unwrap().g {
                        continue;
                    }
                }
                if processed.contains_key(&neighbor) {
                    if g > processed.get(&neighbor).unwrap().g {
                        continue;
                    }
                }
                to_search.remove(&neighbor);
                processed.remove(&neighbor);
                to_search.insert(neighbor.clone(), Node {
                    position: neighbor.clone(),
                    g: g,
                    h: h,
                    f: f,
                    parent: Some(current_position.clone()) });
        
            }
    }
    Ok(path)
}

fn spawn_optimized_pathfinding_task(
    commands: &mut Commands,
    target: Entity,
    bounds: HashSet<Position>,
    start: Position,
    goal: Position,
) {

     let thread_pool = AsyncComputeTaskPool::get();
 
     let task = thread_pool.spawn(async move {
         let path = find_path(start, goal, bounds );
         path
     });
     println!("Task spawned");
     commands.entity(target).insert(PathfindingTask(task));
}

pub fn generate_trial(
    mut commands: Commands,
    grid: Res<EmptyCellsWorldPos>,
    mut reader: MessageReader<FindPath>,
) {
    let mut empty_cells_grid_pos: HashSet<Position> = HashSet::new();
    for cell in grid.cells.iter() {
        let grid_pos = Position {
            x: (cell.x / CELL_SIZE) as i32,
            y: (cell.y / CELL_SIZE) as i32,
        };
        empty_cells_grid_pos.insert(grid_pos);
    }
    for msg in reader.read() {
        let player_pos = msg.target_pos;
        let enemy_pos = msg.seeker_pos;
        let enemy_e = msg.seeker;
        let start = Position {
            x: (enemy_pos.x / CELL_SIZE).round() as i32,
            y: (enemy_pos.y / CELL_SIZE).round() as i32,
        };
        let goal = Position {
            x: (player_pos.x / CELL_SIZE).round() as i32,
            y: (player_pos.y / CELL_SIZE).round() as i32,
        };
        
        let mut bounds = HashSet::new();
        let central_cell = (start.x, start.y);
        let cells_in_bounds = get_cells_in_radius(central_cell, 400.0);
        for cell in cells_in_bounds {
            let pos = Position {
                x: cell.0,
                y: cell.1,
            };
            if empty_cells_grid_pos.get(&pos).is_some() {
                bounds.insert(pos);
            }
        }
        
        spawn_optimized_pathfinding_task(&mut commands, enemy_e, bounds, start, goal);
    }
}

pub fn apply_pathfinding_to_ai(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut PathfindingTask)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: Res<EmptyCellsWorldPos>,
) {
    let mut empty_cells_grid_pos: HashSet<(i32, i32)> = HashSet::new();
    for cell in grid.cells.iter() {
        let grid_pos = ((cell.x / CELL_SIZE) as i32, (cell.y / CELL_SIZE) as i32);
        empty_cells_grid_pos.insert(grid_pos);
    }
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
                    for step in &optimized {
                        println!("Step: {:?}", step);
                        commands.spawn((
                                Mesh2d(meshes.add(Rectangle::default())),
                                MeshMaterial2d(materials.add(Color::srgba(0.5, 0.5, 0.5, 0.5))),
                                Transform::from_xyz(step.x, step.y, 2.0).with_scale(Vec3::splat(16.0)),
                            ));
                    }
                    commands.entity(task_entity).insert( AiPath {
                        steps: optimized,
                    });
                    println!("Path applied");
                } 
            }
        }
    }
}


fn grid_cells_in_rect(start: Vec2, end: Vec2) -> Vec<(i32, i32)> {
    let min_x = (start.x.min(end.x) / CELL_SIZE).floor() as i32;
    let max_x = (start.x.max(end.x) / CELL_SIZE).floor() as i32;
    let min_y = (start.y.min(end.y) / CELL_SIZE).floor() as i32;
    let max_y = (start.y.max(end.y) / CELL_SIZE).floor() as i32;

    let mut cells = Vec::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            cells.push((x, y));
        }
    }
    cells
}

fn is_rect_walkable(start: Vec2, end: Vec2, walkable_cells: &HashSet<(i32, i32)>) -> bool {
    grid_cells_in_rect(start, end)
        .into_iter()
        .all(|cell| walkable_cells.contains(&cell))
}

#[derive(Debug)]
pub struct PathOptimizationError;

fn optimize_path(path: &mut VecDeque<Vec2>, empty_cells: &HashSet<(i32, i32)>  ) -> Result<VecDeque<Vec2>, PathOptimizationError> {
    if path.is_empty() {
        return Err(PathOptimizationError);

    }
    
        let mut simplified = VecDeque::new();
        simplified.push_front(*path.front().unwrap());
        let mut last = 0;
    
        for current in 1..path.len() {
            if !is_rect_walkable(path[last], path[current], empty_cells) {
                // cannot skip, so add previous point
                simplified.push_back(path[current - 1]);
                last = current - 1;
            }
        }
    
        // always add last point
        simplified.push_back(*path.back().unwrap());
        Ok(simplified)
}