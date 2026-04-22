mod mouse_input;

pub use mouse_input::*;
use bevy::prelude::*;
use crate::messages::{MouseClickEvent, ScrollMessage, ScrollDir};

use bevy::camera::{Camera};
use bevy::input::ButtonInput;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::Vec2;
use bevy::prelude::{GlobalTransform,MessageWriter, MessageReader, MouseButton, Query, Res, ResMut, Resource, Window, With};
use bevy::window::PrimaryWindow;


pub struct MouseInputPlugin; 

impl Plugin for MouseInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (get_cursor_position, mouse_click_handler, scroll_events));
        app.insert_resource(CursorWorldPos(None));
    }
}


#[derive(Resource, Default)]
pub struct CursorWorldPos(pub Option<Vec2>);
