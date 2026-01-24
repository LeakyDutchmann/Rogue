use crate::mouse::*;


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

pub fn tile_click_system(
    mut query: Query<(Entity, &mut MapTile,), With<Wall>>,
    mut reader: MessageReader<MouseClickEvent>,
    mut writer: MessageWriter<ApplyDestruction>,
) {
    for click in reader.read() {
        if let MouseClickEvent::LeftClick(pos) = click {
            let click_pos = world_to_tile(*pos);
            for (entity, tile) in query.iter_mut() {
                let tile_pos = tile.position;
                if click_pos == tile_pos {
                    writer.write(ApplyDestruction{
                        entity,
                        position: tile_pos,
                    });
                }
                
            }
        }
    }
}

pub fn item_click_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), (With<Item>, With<OnGround>)>,
    mut player: Query<Entity, With<Player>>,
    mut reader: MessageReader<MouseClickEvent>,
) {
    let mut player = player.single_mut().unwrap();
    for click in reader.read() {
        if let MouseClickEvent::LeftClick(click_pos) = click {
            for (entity, transform) in query.iter_mut() {
                let item_pos = transform.translation.truncate();
                let dist = click_pos.distance(item_pos);
                if dist <= 32.0  {
                    commands.entity(player).insert(IntentPickingUp { target: entity});
                    println!("trying to pick up item!")
                }
                
            }
        }
    }
}


// This looks like a World or maybe Camera method. Or Camera method that takes World argument,
// because it needs to know size of the world.
// The words you may be looking for is "world to screen / screen to world coordinate conversion".
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

pub fn scroll_events(
    mut scroll: MessageReader<MouseWheel>,
    mut writer: MessageWriter<ScrollMessage>,
) {
    for ev in scroll.read() {
        if ev.y == 1.0 {
            writer.write(ScrollMessage {
                event: ScrollDir::ScrollUp
            });
            println!("Scroll up");
        }
        if ev.y == -1.0 {
            writer.write(ScrollMessage {
                event: ScrollDir::ScrollDown
            });
        }
    }
}




