use super::*; 

pub fn builder_ui_interactions(
    mut slots: Query<(Entity, &Interaction, &Children, &mut BorderColor), (With<BuildingUiNode>, Changed<Interaction>)>,
) {
    for (entity, interaction, children, mut border) in slots.iter_mut() {
        if interaction == &Interaction::Pressed {
            *border = BorderColor::all(Color::srgb(1.0, 1.0, 1.0));
            println!("pressed");
        } else if interaction == &Interaction::Hovered {
            *border = BorderColor::all(Color::srgb(1.0, 1.0, 1.0));
        } else {
            *border = BorderColor::all(Color::srgb(0.5, 0.5, 0.5));
        }
    }
}
