use super::*;

pub fn setup_console(
    mut commands: Commands, 
) {
    
    let root = commands.spawn((
        Node {
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            width: Val::Px(400.0),
            height: Val::Px(400.0),
            top: Val::Px(40.0),
            left: Val::Px(15.0),
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Stretch,
            ..default()
        },
        BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.3)),
        BorderColor::all(Color::srgb(0.2, 0.2, 0.2)),
        UiConsoleMarker,
        
    )).id();
    
    let scroll_container = commands.spawn((
        Node {
            border: UiRect::all(Val::Px(1.0)),
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Stretch,
            top: Val::Px(20.0),
            justify_content: JustifyContent::FlexStart,
            height: percent(80),
            overflow: Overflow::scroll_y(), // n.b.
            ..default()
        },
        BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
        ConsoleScrollZoneMarker,
        Interaction::default(),
    )).id();
    commands.entity(root).add_child(scroll_container);
}

pub fn start_chat(
    mut console: ResMut<Console>    
) {
    console.log(String::from("Welcome to the console"));
    console.log(String::from("You only can read the messages yet"));
    console.log(String::from("Later I, maybe, will implement commands"));
}
