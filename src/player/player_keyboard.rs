use crate::player::*;


pub fn pick_active_slot(
    keys: Res<ButtonInput<KeyCode>>,
    mut active_slot: Query<&mut ActiveSlot, With<Player>>,
) {
    if let Ok(mut slot) = active_slot.single_mut() {
        let mut active = None;
        let hotbar_keys = [
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
        ];

        for (index, key) in hotbar_keys.iter().enumerate() {
            if keys.just_pressed(*key) {
                active = Some(index);
                break;
            }
        }
        if let Some(active) = active {
            slot.index = active;
            println!("Active slot changed to {}", active);
        }
    }
}

pub fn keyboard_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: MessageWriter<KeyPressed>, )
{
    if keys.just_pressed(KeyCode::KeyG) {
        writer.write(KeyPressed {
            key: KeyCode::KeyG 
        });
    }      
    
}













