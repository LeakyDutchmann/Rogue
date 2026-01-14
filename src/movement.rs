use bevy::prelude::*;

use crate::components::*;
use crate::colision_manager::*;
use crate::world::*;


pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement.after(resolve_movement));
    }
}


pub fn apply_movement(
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    mut intender: Query<(Entity, &mut Transform, &Speed, &MovementResolved), With<MovementResolved>>,
    mut world: ResMut<WorldGrid>,
) {
    for (entity, mut transform, speed, movement) in intender.iter_mut() {
        let old_pos = transform.translation.truncate();
        let old_cell = (
            (old_pos.x / CELL_SIZE).floor() as i32,
            (old_pos.y / CELL_SIZE).floor() as i32,
        );
        
        let direction = movement.direction;
        // let movement = direction * speed.0 * time.delta_secs();
        let new_pos = old_pos + direction;
        let new_cell = (
            (new_pos.x / CELL_SIZE).floor() as i32,
            (new_pos.y / CELL_SIZE).floor() as i32,
        );
        if new_cell != old_cell {
            if let Some(entities) = world.cells.get_mut(&old_cell) {
                entities.retain(|e| *e != entity);
            }
            world.cells.entry(new_cell).or_default().push(entity);
        }
        transform.translation.x += direction.x;
        transform.translation.y += direction.y;
        
        commands.entity(entity).remove::<MovementResolved>();
    }
}