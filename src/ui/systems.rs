use super::*;

pub fn hover_system(
    mut query: Query<(&mut BorderColor, &Interaction), Without<UiBackground>>
) {
    for (mut border_color, interaction) in query.iter_mut() {
        if *interaction == Interaction::Hovered {
            *border_color = BorderColor::all(Color::srgb(1.0, 1.0, 1.0));
            println!("Hovering slot");
        } else if *interaction == Interaction::Pressed {
            *border_color = BorderColor::all(Color::srgb(1.0, 0.4, 0.0));
        } else {
            *border_color = BorderColor::all(Color::srgb(0.5, 0.5, 0.5));
        }
    }
}