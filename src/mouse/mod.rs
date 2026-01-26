mod mouse_input;

pub use mouse_input::*;
use bevy::prelude::*;
use crate::components::*;
use crate::map_setup::*;
use crate::items::*;
use crate::player::*;

use bevy::camera::{Camera};
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseWheel;
use bevy::math::{IVec2, Vec2};
use bevy::prelude::{GlobalTransform, Entity, Commands, Message, MessageWriter, MessageReader, MouseButton, Query, Res, ResMut, Resource, Window, With};
use bevy::window::PrimaryWindow;



pub struct MouseInputPlugin; 

impl Plugin for MouseInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (get_cursor_position, mouse_click_handler, scroll_events, item_click_system));
        app.add_message::<MouseClickEvent>();
        app.add_message::<ScrollMessage>();
        app.insert_resource(CursorWorldPos(None));
    }
}

//Resourses

#[derive(Resource, Default)]
pub struct CursorWorldPos(pub Option<Vec2>);



//Messages or Events

#[derive(Message)]
pub enum MouseClickEvent {
    LeftClick(Vec2),
}





#[derive(Message)]
pub struct ScrollMessage {
    pub event: ScrollDir,
}


//Components



//Enums

#[derive(PartialEq, Clone)]
pub enum ScrollDir {
    ScrollUp,
    ScrollDown,
}