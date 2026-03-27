use super::*;

fn radius_to_cells(radius: f32) -> i32 {
    (radius / CELL_SIZE).floor() as i32
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