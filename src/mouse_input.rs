use bevy::camera::Camera;
use bevy::input::ButtonInput;
use bevy::math::{IVec2, Vec2};
use bevy::prelude::{GlobalTransform, Entity, Commands, Message, MessageWriter, MessageReader, MouseButton, Query, Res, ResMut, Resource, Window, With};
use bevy::transform::commands;
use bevy::window::PrimaryWindow;
use crate::components::*;
use crate::map::*;


#[derive(Resource, Default)]
pub struct CursorWorldPos(pub Option<Vec2>);



#[derive(Message)]
pub enum MouseClickEvent {
    LeftClick(Vec2),
}

#[derive(Message)]
pub struct ApplyDestruction {
    pub entity: Entity,
    pub position: IVec2,
}



#[derive(Message)]
pub struct MapChanged {
    pub position: IVec2,
}


pub fn get_cursor_position(
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut cursor: ResMut<CursorWorldPos>,
) {
    let window = window_q.single().unwrap();
    let (camera, cam_transform) = camera_q.single().unwrap();
    cursor.0 = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
}

pub fn mouse_click_handler(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor: ResMut<CursorWorldPos>,
    mut writer: MessageWriter<MouseClickEvent>,
) {
    let Some(position) = cursor.0 else {
        return;
    };
    if buttons.pressed(MouseButton::Left) {
        writer.write(MouseClickEvent::LeftClick(position));
    }
}

pub fn mouse_events(
    mut query: Query<(Entity, &mut MapTile), With<Wall>>,
    mut reader: MessageReader<MouseClickEvent>,
    mut writer: MessageWriter<ApplyDestruction>,
) {
    for click in reader.read() {
        if let MouseClickEvent::LeftClick(pos) = click {
            let click_pos = world_to_tile(*pos);
            for (entity, tile) in query.iter_mut() {
                let tile_pos = tile.position;
                if click_pos == tile_pos {
                    println!("Clicked on tile {:?}", tile_pos);
                    writer.write(ApplyDestruction{
                        entity,
                        position: tile_pos
                    });
                }
                
            }
        }
    }
}

fn world_to_tile(world: Vec2) -> IVec2 {
    let x = ((world.x + (MAP_WIDTH as f32 / 2.0) * TILE_SIZE) / TILE_SIZE).round() as i32;
    let y = ((world.y + (MAP_HEIGHT as f32 / 2.0) * TILE_SIZE) / TILE_SIZE).round() as i32;
    IVec2::new(x, y)
}

pub fn destruction_system(
    mut commands: Commands,
    mut reader: MessageReader<ApplyDestruction>,
    mut writer: MessageWriter<MapChanged>,
) {
    for destruction in reader.read() {
        commands.entity(destruction.entity).despawn();
        writer.write(MapChanged {
            position: destruction.position
        });
    }
}

