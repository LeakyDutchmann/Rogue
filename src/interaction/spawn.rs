use super::*;


pub fn spawn_basic_oven_window(
    mut commands: Commands,
    mut reader: MessageReader<SpawnWindowRequest>,
) {
    for msg in reader.read() {
        if msg.window_type != WindowType::BasicOven {
            continue;
        }
        commands.spawn((
            Node {
                border: UiRect::all(Val::Px(2.0)),
                width: Val::Px(400.0),
                height: Val::Px(200.0),
                left: Val::Percent(50.0),
                bottom: Val::Px(-250.0),
                margin: UiRect::left(Val::Px(-200.0)),
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            BorderColor::all(Color::srgb(1.0, 1.0, 1.0)),
            ZIndex(2),
            UiStructureWindow,
        )).with_children(|parrent| {
            parrent.spawn((
                Node {
                    width: Val::Px(100.0),
                    left: Val::Percent(50.0),
                    margin: UiRect::left(Val::Px(-50.0)),
                    ..Default::default()
                },
                Text(String::from("Basic_Oven")),
                ZIndex(3)
            ));
            parrent.spawn((
                Node {
                    border: UiRect::all(Val::Px(2.0)),
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    top: Val::Percent(50.0),
                    margin: UiRect::top(Val::Px(-24.0)),
                    left: Val::Px(0.0),
                    ..default()
                },
                BorderColor::all(Color::srgb(1.0, 0.4, 0.0)),
                Interaction::None,
                ZIndex(4),
            ));
            parrent.spawn((
                Node {
                    border: UiRect::all(Val::Px(2.0)),
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    top: Val::Percent(50.0),
                    margin: UiRect::top(Val::Px(-24.0)),
                    left: Val::Px(200.0),
                    ..default()
                },
                BorderColor::all(Color::srgb(1.0, 0.4, 0.0)),
                Interaction::None,
                ZIndex(4),
            ));
        });
    }
}