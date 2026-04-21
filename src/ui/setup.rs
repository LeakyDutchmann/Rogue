use super::*;

pub fn spawn_tool_tip(
    mut commands: Commands,
) {
    commands.spawn((
        ToolTip,
        Node {
            border: UiRect::all(Val::Px(4.0)),
            width: Val::Px(200.0),
            height: Val::Px(100.0),
            ..Default::default()
        },
        BackgroundColor(Color::BLACK),
        BorderColor::all(Color::WHITE),
        ZIndex(5),
        Text::new("_")
    ));
}