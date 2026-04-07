use crate::{map_setup::CHUNK_WIDTH, world::*};

pub fn insert_entities(
    mut entities: Query<(Entity, &Transform)>,
    mut world: ResMut<WorldGrid>,
) {
    for (entity, transform) in entities.iter_mut() {
        let pos = transform.translation.truncate();
        let cell_x = (pos.x / CELL_SIZE).round() as i32;
        let cell_y = (pos.y / CELL_SIZE).round() as i32;
        world.cells.entry((cell_x, cell_y)).or_default().push(entity);
    }
}

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
        let cell_pos = tile_pos_to_world_pos(msg.position);
        empty_cells.cells.push(cell_pos);
    } 
}

pub fn setup_bounds(
    shared: ResMut<SharedBounds>,
    grid: Res<EmptyCellsWorldPos>,
) {
    let mut bounds = shared.0.write().unwrap();
    for cell in grid.cells.iter() {
        bounds.insert(Position {
            x: (cell.x / CELL_SIZE) as i32,
            y: (cell.y / CELL_SIZE) as i32,
        });
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

// pub fn check_grid(
//     player: Query<&Transform, With<Player>>,
//     world: Res<WorldGrid>,
// ) {
//     let player_pos = player.single().unwrap().translation.truncate();
//     let cell_x = (player_pos.x / CELL_SIZE).floor() as i32;
//     let cell_y = (player_pos.y / CELL_SIZE).floor() as i32;
//     let cell = world.cells.get(&(cell_x, cell_y));
//     for entities in cell {
//         println!("Entity in cell: {:?}", entities);
//     }
// }

pub fn track_of_cells(
    cells: Res<WorldGrid>,
) {
    if !cells.is_changed() {
        return;
    }
    let mut count = 0;
    for (_, _) in cells.cells.iter() {
        count += 1;
    }
    println!("Number of cells: {}", count);
}

pub fn modify_grid(
    mut commands: Commands,
    mut cells: ResMut<WorldGrid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (grid_pos, entities) in cells.cells.iter_mut() {
        let x = grid_pos.0 as f32 * CELL_SIZE;
        let y = grid_pos.1 as f32 * CELL_SIZE;
        let translation = Vec3::new(x, y, 20.0);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(16.0, 16.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.4, 0.1))),
            Transform::from_translation(translation)
        ));
        println!("spawning at ({}, {})", grid_pos.0, grid_pos.1);
    }
}