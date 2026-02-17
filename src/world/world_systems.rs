use crate::world::*;
use bevy::{color::palettes::basic::PURPLE, prelude::*};

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

fn radius_to_cells(radius: f32) -> i32 {
    (radius / CELL_SIZE).ceil() as i32
}

pub fn get_cells_in_radius(central: (i32, i32), radius: f32) -> Vec<(i32, i32)> {
    let mut neighbour_cells = Vec::new();
    let delta = radius_to_cells(radius);
    for dx in -delta..=delta {
        for dy in -delta..=delta {
            neighbour_cells.push((central.0 + dx, central.1 + dy));
        }
    }
    neighbour_cells
}

pub fn get_cells_3x3(central: (i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbour_cells = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            neighbour_cells.push((central.0 + dx, central.1 + dy));
        }
    }
    neighbour_cells
}


pub fn get_entities_in_cells(cells: Vec<(i32, i32)>, world: &WorldGrid) -> Vec<Entity> {
    let mut entities = Vec::new();
    for cell in cells {
        if let Some(entities_in_cell) = world.cells.get(&cell) {
            for &e in entities_in_cell {
                entities.push(e);
            }
        }
    }
    entities
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

pub fn modify_empty(
    mut commands: Commands,
    cells: Res<EmptyCellsWorldPos>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for &pos in cells.cells.iter(){
        commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(PURPLE))),
                Transform::from_xyz(pos.x, pos.y, 2.0).with_scale(Vec3::splat(32.0)),
            ));
        println!("spawned at {:?}", pos);
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

