use super::*;

pub fn toggle_console(
    keys: Res<ButtonInput<KeyCode>>,
    mut console_open: ResMut<ConsoleOpen>,
) {
    if keys.just_pressed(KeyCode::Backquote) {
        if !console_open.0 {
            console_open.0 = true;
        } else {
            console_open.0 = false;
        }
    }
}

pub fn set_console_visibility(
    console_open: Res<ConsoleOpen>,
    mut console: Query<&mut Visibility, With<UiConsoleMarker>>
) {
    for mut visibility in console.iter_mut() {
        if console_open.0 {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn console_scroll(
    mut reader: MessageReader<ScrollMessage>,
    mut query: Query<(&Interaction, &mut ScrollPosition, &ComputedNode), With<ConsoleScrollZoneMarker>>,
) {
    for (interaction, mut scroll_position, computed) in query.iter_mut() {
        if *interaction == Interaction::Hovered {
            for msg in reader.read() {
                let mut max_offset = computed.content_size() - computed.size();
                let pos = scroll_position.0.y.clone();
                let scroll = pos + msg.delta.y;
                if max_offset.y < 0.0 {
                    max_offset.y = 0.0;
                }
                scroll_position.0.y = scroll.clamp(0.0, max_offset.y);
            }
        }
    }
}

pub fn console_add_output(
    mut console: ResMut<Console>,
    mut commands: Commands,
    query: Query<Entity, With<ConsoleScrollZoneMarker>>,
) {
    for entity in query.iter() {
        if console.lines.is_empty() {
            continue;
        }
        for line in console.lines.iter() {
            let new_line = commands.spawn((
                Node {
                    min_height: px(20.0),
                    max_height: px(20.0),
                    ..default()
                },
                children![(
                    Text(line.clone()),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                )],      
            )).id();
            commands.entity(entity).add_child(new_line);
        }
        console.lines.clear();
    }
}

pub fn console_snap_to_bottom(
    mut query: Query<(&mut ScrollPosition, &ComputedNode), (With<ConsoleScrollZoneMarker>, Changed<Children>)>,
) {
    for (mut scroll_position, computed) in query.iter_mut() {
        let mut max_offset = computed.content_size() - computed.size();
        if max_offset.y < 0.0 {
            max_offset.y = 0.0;
        }
        scroll_position.0.y = max_offset.y;
    }
}

