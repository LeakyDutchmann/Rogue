use crate::world::*;


pub fn insert_entities(
    mut commands: Commands,
    mut entities: Query<(Entity, &Transform)>,
    mut world: ResMut<WorldGrid>,
) {
    for (entity, transform) in entities.iter_mut() {
        let pos = transform.translation.truncate();
        let cell_x = (pos.x / CELL_SIZE).floor() as i32;
        let cell_y = (pos.y / CELL_SIZE).floor() as i32;
        world.cells.entry((cell_x, cell_y)).or_default().push(entity);
        println!("inserted entities succesfuly");
    }
}


pub fn check_grid(
    player: Query<&Transform, With<Player>>,
    world: Res<WorldGrid>,
) {
    let player_pos = player.single().unwrap().translation.truncate();
    let cell_x = (player_pos.x / CELL_SIZE).floor() as i32;
    let cell_y = (player_pos.y / CELL_SIZE).floor() as i32;
    let cell = world.cells.get(&(cell_x, cell_y));
    for entities in cell {
        println!("Entity in cell: {:?}", entities);
    }
}

