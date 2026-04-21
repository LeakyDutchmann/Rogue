use super::*;

pub fn update_worldgird(
    mut entities: Query<(Entity, &Transform), Added<Transform>>,
    mut world: ResMut<WorldGrid>,
) {
    for (entity, transform) in entities.iter_mut() {
        let pos = transform.translation.truncate();
        let cell_x = (pos.x / CELL_SIZE).round() as i32;
        let cell_y = (pos.y / CELL_SIZE).round() as i32;
        world.cells.entry((cell_x, cell_y)).or_default().push(entity);
    }
}

pub fn find_empty_cells(
    world: Res<WorldGrid>,
    mut empty_cells: ResMut<EmptyCellsWorldPos>,
    wall: Query<&Wall>,
) {
    for (&coords, entities) in world.cells.iter() {
        let blocked = entities.iter()
            .any(|e| wall.get(*e).is_ok());
        if !blocked {
            empty_cells.cells.push(Vec2::from(
                (coords.0 as f32 * TILE_SIZE, coords.1 as f32 * TILE_SIZE)
            ));
        }
        
    }
    
}

pub fn update_empty_cells(
    mut reader: MessageReader<MapChanged>,
    mut empty_cells: ResMut<EmptyCellsWorldPos>,
) {
    for msg in reader.read() {
        empty_cells.cells.push(msg.pos);
    } 
}

pub fn update_bounds(
    shared: ResMut<SharedBounds>,
    grid: Res<EmptyCellsWorldPos>,
) {
    if !grid.is_changed() {
        return; 
    }
    let mut bounds = shared.0.write().unwrap();
    bounds.clear();
    for cell in grid.cells.iter() {
        bounds.insert(Position {
            x: (cell.x / CELL_SIZE) as i32,
            y: (cell.y / CELL_SIZE) as i32,
        });
    }
}
