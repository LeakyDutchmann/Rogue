mod camera;

use camera::*;
use bevy::prelude::*;

pub struct CameraSetupPlugin;

impl Plugin for CameraSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
        app.add_systems(FixedUpdate, camera_follow_player);
    }
}
