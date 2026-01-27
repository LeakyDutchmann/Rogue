mod camera;

use camera::*;
use bevy::prelude::*;
use crate::player::*;
use crate::mouse::*;
use crate::messages::{ScrollMessage, ScrollDir};

pub struct CameraSetupPlugin;

impl Plugin for CameraSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
        app.add_systems(Update, camera_scroll_in.after(crate::mouse::scroll_events));
        app.add_systems(FixedUpdate, camera_follow_player);
    }
}
