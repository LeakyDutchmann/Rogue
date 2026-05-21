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

pub fn setup_enemy_registry(
    mut enemy_registry: ResMut<EnemyRegistry>,
) {
    let path = "./data/enemy";
    if let Ok(defs) = load_definitions_for::<EnemyDefinitionRaw>(path) {
        for def in defs {
            let enemy_def = EnemyDefinition {
                hp: def.hp,
                sprite_sheet: def.sprite_sheet,
                dead_sprite: def.dead_sprite,
                kind: def.kind,
                speed: def.speed,
                awareness_range: def.awareness_range,
                pursuit_time: def.pursuit_time,
                colider: def.colider,
                hurt_radius: def.hurt_radius,
                fraction: def.fraction,
                held_item: def.held_item,
            };
            println!("loaded enemy: {}", def.name);
            enemy_registry.definitions.insert(def.name.clone(), enemy_def);
        }
    } else {
        println!("failed to load enemy definitions");
    }
    
}



