use bevy::ecs::entity;

use super::*;

pub fn apply_swarn_buff_system(
    mut commands: Commands,
    mut non_buffed: Query<(Entity, &EnemyId, &mut Health, &mut Speed), Without<Buffed>>,
    mut buffed: Query<(Entity, &EnemyId, &mut Health, &mut Speed, &Children), With<Buffed>>,
    buffed_marker: Query<&BuffVisualMarker>,
    enemy_reg: Res<EnemyRegistry>,
    mut swarm_buff: ResMut<SwarmBuffState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if swarm_buff.0 == true {
        for (enemy_e, id, mut hp, mut speed) in non_buffed.iter_mut() {
            if let Some(def) = enemy_reg.definitions.get(&id.id) {
                if let Some(buff) = &def.swarm_buff {
                    if let Some(buff_hp) = buff.hp {
                        let new_hp = (def.hp as f32 * (buff_hp as f32 / 100.0)).round() as i32;
                        hp.0 = new_hp;
                    }
                    if let Some(buff_speed) = buff.speed {
                        let new_speed = def.speed as f32 * (buff_speed as f32 / 100.0);
                        speed.0 = new_speed;
                    }
                    commands.entity(enemy_e).insert(Buffed);
                    let color = Color::srgb(1.0, 0.0, 0.0);
                    let child = commands.spawn((
                        Mesh2d(meshes.add(Triangle2d::new(Vec2::new(0.0, -2.0), Vec2::new(-2.0, 2.0), Vec2::new(2.0, 2.0)))),
                        MeshMaterial2d(materials.add(color)),
                        Transform::from_xyz(0.0, 8.0, 2.0),
                        BuffVisualMarker,
                    )).id();
                    commands.entity(enemy_e).add_child(child);
                }
            }
        }
    } else {
        for (enemy_e, id, mut hp, mut speed, children) in buffed.iter_mut() {
            for child in children.iter() {
                if let Ok(_) = buffed_marker.get(child) {
                    commands.entity(child).despawn();
                }
            }
            if let Some(def) = enemy_reg.definitions.get(&id.id) {
                hp.0 = def.hp;
                speed.0 = def.speed as f32;
                commands.entity(enemy_e).remove::<Buffed>();
            }
        }
    }
}

pub fn track_enemies_near_player(
    enemy: Query<&Enemy>,
    player_tf: Res<PlayerTransform>,
    world: Res<WorldGrid>,
    mut swarm_buff: ResMut<SwarmBuffState>,
    mut console: ResMut<Console>,
) {
    let pos = player_tf.0.translation.truncate();
    let cell_x = (pos.x / CELL_SIZE).round() as i32;
    let cell_y = (pos.y / CELL_SIZE).round() as i32;
    let cells = get_cells_in_radius((cell_x, cell_y), 200.0);
    let entities = get_entities_in_cells(cells, &world);
    let mut count = 0;
    for entity in entities {
        if let Ok(_) = enemy.get(entity) {
            count += 1;
        }
    }
    if count >= 5 {
        swarm_buff.0 = true;
    } else {
        swarm_buff.0 = false;
    }
    console.log(format!("{:?} enemy near", count))
} 

