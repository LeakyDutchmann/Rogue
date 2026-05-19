use super::*;

pub fn generate_position_near(cells: &Vec<Vec2>, target: Vec2) -> Vec2 {
    let mut final_pos = Vec2::from((33.0, 33.0));
    let mut rng = rand::rng();
    let filtered: Vec<_> = cells
        .iter()
        .filter(|&&pos| {
                pos.distance(target) <= 300.0
                })
        .cloned()
        .collect();
    if let Some(cell) = filtered.choose(&mut rng) {
        final_pos = Vec2::from((cell.x, cell.y));
    }
    final_pos
}

pub fn update_enemy_state(
    mut text_query: Query<(&mut Text2d, &ChildOf, &mut TextColor), With<Marker>>,
    health_query: Query<&EnemyAwareness>,
) {
    for (mut text, parent, mut color) in text_query.iter_mut() {
        if let Ok(vision) = health_query.get(parent.0) {
            text.0 = match vision.state {
                AwarenessType::Unaware => "Unaware".to_string(),
                AwarenessType::Direct => "Direct".to_string(),
                AwarenessType::Indirect => "Indirect".to_string(),
            };
            *color = match vision.state {
                AwarenessType::Unaware => TextColor(Color::srgb(0.0, 1.0, 0.0)),
                AwarenessType::Direct => TextColor(Color::srgb(1.0, 0.0, 0.0)),
                AwarenessType::Indirect => TextColor(Color::srgb(1.0, 1.0, 0.0)),
            };
        }
    }
}



