use crate::player::*;






pub fn keyboard_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: MessageWriter<KeyPressed>, 
) {
    if keys.just_pressed(KeyCode::KeyG) {
        writer.write(KeyPressed {
            key: KeyCode::KeyG 
        });
    }
    if keys.just_pressed(KeyCode::Tab) {
        writer.write(KeyPressed {
            key: KeyCode::Tab 
        });
    }
    if keys.just_pressed(KeyCode::Minus) {
        writer.write(KeyPressed {
            key: KeyCode::Minus 
        });
    }
    if keys.just_pressed(KeyCode::Equal) {
        writer.write(KeyPressed {
            key: KeyCode::Equal 
        });
    }
    
}













