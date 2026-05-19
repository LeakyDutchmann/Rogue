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
    mut reader: MessageReader<RebuildGrid>,
) {
    for msg in reader.read() {
        empty_cells.cells.clear();
        for (&coords, entities) in world.cells.iter() {
            let blocked = entities.iter()
                .any(|e| wall.get(*e).is_ok());
            if !blocked {
                let pos =  Vec2::from((coords.0 as f32 * TILE_SIZE, coords.1 as f32 * TILE_SIZE));
                empty_cells.cells.push(pos);        
            } 
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

// #[derive(Component)]
// pub struct GridMarker;

// pub fn modify_grid(
//     keys: Res<ButtonInput<KeyCode>>,
//     mut commands: Commands,
//     markers: Query<Entity, With<GridMarker>>,
//     mut cells: ResMut<SharedBounds>,
//     mut e_cells: ResMut<EmptyCellsWorldPos>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     wall: Query<&Wall>,
// ) {
//     // if !cells.is_changed() {
//     //     return;
//     // }
//     if keys.just_pressed(KeyCode::Space) {
//         for marker in markers.iter() {
//             commands.entity(marker).despawn();
//         }
//         let bounds = cells.0.read().unwrap();
//         for grid_pos in bounds.iter() {
//             let x = grid_pos.x as f32 * CELL_SIZE;
//             let y = grid_pos.y as f32 * CELL_SIZE;
//             let translation = Vec3::new(x, y, -y * 0.001 + 21.0);
//             commands.spawn((
//                 Mesh2d(meshes.add(Rectangle::new(10.0, 10.0))),
//                 MeshMaterial2d(materials.add(Color::srgb(0.4, 0.1, 0.1))),
//                 Transform::from_translation(translation),
//                 GridMarker,
//             ));
//         }
//     }
//     for grid_pos in e_cells.cells.iter() {
//         let x = grid_pos.x;
//         let y = grid_pos.y;
//         let translation = Vec3::new(x, y, -y * 0.001 + 20.0);
//         commands.spawn((
//             Mesh2d(meshes.add(Rectangle::new(16.0, 16.0))),
//             MeshMaterial2d(materials.add(Color::srgb(0.1, 0.4, 0.1))),
//             Transform::from_translation(translation)
//         ));
//     }
// }

// pub fn check_grid(
//     mut commands: Commands,
//     mut bounds: ResMut<SharedBounds>,
//     mut grid: Res<EmptyCellsWorldPos>,
//     wall: Query<&Wall>,
//     mut console: ResMut<Console>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {   
//     let bounds_unpacked = bounds.0.read().unwrap();
//     for cell in &grid.cells {
//         let wall_pos = Position {
//             x: cell.x as i32,
//             y: cell.y as i32,
//         };
//         if !bounds_unpacked.contains(&wall_pos) {
//             let x = cell.x as f32 * CELL_SIZE;
//             let y = cell.y as f32 * CELL_SIZE;
//             let translation = Vec3::new(x, y, 200.0);
//             commands.spawn((
//                 Mesh2d(meshes.add(Rectangle::new(16.0, 16.0))),
//                 MeshMaterial2d(materials.add(Color::srgb(0.4, 0.1, 0.1))),
//                 Transform::from_translation(translation)
//             ));
//             console.log(format!("found collision at ({}, {})", x, y));
//         }
//     }
// }