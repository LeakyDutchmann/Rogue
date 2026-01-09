use bevy::prelude::*;

use crate::components::*;
use crate::colisions::*;


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
) {
    for (entity, mut transform, speed, movement) in intender.iter_mut() {
        let direction = movement.direction;
        let movement = direction * speed.0 * time.delta_secs();
        
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        
        commands.entity(entity).remove::<MovementResolved>();
    }
}