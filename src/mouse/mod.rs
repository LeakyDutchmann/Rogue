mod mouse_input;

pub use mouse_input::*;
use bevy::prelude::*;
use crate::items::{IntentPickingUp, OnGround};
use crate::player::Player;
use crate::messages::{MouseClickEvent, ScrollMessage, ScrollDir};

use bevy::camera::{Camera};
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseWheel;
use bevy::math::Vec2;
use bevy::prelude::{GlobalTransform, Entity, Commands, MessageWriter, MessageReader, MouseButton, Query, Res, ResMut, Resource, Window, With};
use bevy::window::PrimaryWindow;



pub struct MouseInputPlugin; 

impl Plugin for MouseInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (get_cursor_position, mouse_click_handler, scroll_events));
        app.insert_resource(CursorWorldPos(None));
        app.insert_resource(DoubleClickState {
            last_click: 0.0,
            pending: false,
            pending_pos: Vec2::ZERO,
        });
    }
}


#[derive(Resource, Default)]
pub struct CursorWorldPos(pub Option<Vec2>);


#[derive(Resource)]
struct DoubleClickState {
    last_click: f64,
    pending: bool,
    pending_pos: Vec2,
}
