mod colisions;


use bevy::prelude::*;
pub use colisions::*;
use crate::components::{MovementIntent, MovementResolved, Speed};
use crate::world::{WorldGrid, CELL_SIZE, get_cells_3x3, get_entities_in_cells};
use crate::player::move_player;



pub struct ColisionPlugin; 

impl Plugin for ColisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, resolve_movement.after(move_player));
    }
}

pub enum ColiderShape {
    Circle { radius: f32},
    Rectangle {width: f32, height: f32}
}


#[derive(Component)]
pub struct Colider {
    pub shape: ColiderShape,
    pub _offsety: f32,
    pub _sensor: bool,
    
}


impl Colider {
    pub fn check_colisions(&self, pos1: Vec2, other: &Colider, pos2: Vec2) -> bool {
        match (&self.shape, &other.shape) {
            (ColiderShape::Circle {radius: r1},ColiderShape::Circle {radius: r2}) => {
                false
            },
            (ColiderShape::Circle { radius }, ColiderShape::Rectangle { width, height }) |
            (ColiderShape::Rectangle { width, height }, ColiderShape::Circle { radius }) => {
                let half_w = width / 2.0;
                let half_h = height / 2.0;
                            
                let closest_x = (pos1.x - pos2.x).clamp(-half_w, half_w) + pos2.x;
                let closest_y = (pos1.y - pos2.y).clamp(-half_h, half_h) + pos2.y;
                let closest = Vec2::new(closest_x, closest_y);
                            
                pos1.distance_squared(closest) < radius * radius
            },
            (ColiderShape::Rectangle { width: _width, height: _height}, ColiderShape::Rectangle { width: _width2, height: _height2}) => {
                false
            }
        }
    }
}



