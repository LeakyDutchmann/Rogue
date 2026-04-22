mod systems;
mod setup;

use systems::*;
use setup::*;
use super::*;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::*,
    text::FontSmoothing,
};

struct OverlayColor;

impl OverlayColor {
    const _RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                    FpsOverlayPlugin {
                        config: FpsOverlayConfig {
                            text_config: TextFont {
                                // Here we define size of our overlay
                                font_size: 32.0,
                                // If we want, we can use a custom font
                                font: default(),
                                // We could also disable font smoothing,
                                font_smoothing: FontSmoothing::default(),
                                ..default()
                            },
                            // We can also change color of the overlay
                            text_color: OverlayColor::GREEN,
                            // We can also set the refresh interval for the FPS counter
                            refresh_interval: core::time::Duration::from_millis(100),
                            enabled: true,
                            frame_time_graph_config: FrameTimeGraphConfig {
                                enabled: false,
                                // The minimum acceptable fps
                                min_fps: 30.0,
                                // The target fps
                                target_fps: 144.0,
                            },
                        },
                    },
                ));
        app.insert_resource(ConsoleOpen(false));
        app.insert_resource(Console {
            lines: Vec::new(),
        });
        app.add_systems(Startup, (setup_console, start_chat));
        app.add_systems(Update, (toggle_console, set_console_visibility, console_scroll, console_add_output));
    }
}


#[derive(Resource)]
pub struct ConsoleOpen(pub bool);

#[derive(Resource)]
pub struct Console {
    pub lines: Vec<String>,
}

impl Console {
    pub fn log(&mut self, line: String) {
        self.lines.push(line);
    }
}


#[derive(Component)]
pub struct UiConsoleMarker;

#[derive(Component)]
pub struct ConsoleScrollZoneMarker;



