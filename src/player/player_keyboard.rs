use crate::player::*;

pub fn keyboard_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: MessageWriter<KeyPressed>, 
) {
    for key in keys.get_just_pressed() {
        writer.write(KeyPressed { key: *key });
    }
}













