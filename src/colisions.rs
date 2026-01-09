use bevy::prelude::*;

use crate::components::*;
use crate::map_setup::*;
use crate::player::*;
use crate::mouse::*;

pub struct ColisionPlugin; 

impl Plugin for ColisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, resolve_movement);
    }
}

pub fn resolve_movement(
    mut commands: Commands,
    mut intender: Query<(Entity, &Transform, &MovementIntent), With<MovementIntent>>,
) {
    for (entity, transform, direction_intended) in intender.iter_mut() {
        commands.entity(entity).insert(MovementResolved { direction: direction_intended.direction });
        println!("movement resolved");
        commands.entity(entity).remove::<MovementIntent>();
        println!("extra intend removed");
    }
}