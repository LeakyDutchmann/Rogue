use super::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;

pub fn attack_progression_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut AttackAnimation, &ChildOf)>,
    tf_qr: Query<(&Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, mut anim, childof) in query.iter_mut() {
        anim.progress += time.delta_secs() / anim.duration;
        anim.progress = anim.progress.clamp(0.0, 1.0);
        
        if !anim.hit_triggered && anim.progress >= 0.5 {
            anim.hit_triggered = true;
            if let Ok(parent_tf) = tf_qr.get(childof.0) {
                let parent_pos = parent_tf.translation.truncate();
                let cursor_pos = anim.cursor_pos;
                let to_cursor = (cursor_pos - parent_pos).normalize_or_zero();
                let dir_angle = to_cursor.to_angle();
                let start_angle = dir_angle - anim.max_angle / 2.0;
                let end_angle = dir_angle + anim.max_angle / 2.0;
                commands.spawn((
                    HitBox {
                        owner: childof.0,
                        radius: anim.item_radius,
                        start_angle: start_angle,
                        end_angle: end_angle,
                        item_used: anim.item,
                    },
                    Transform::from_translation(parent_pos.extend(0.0))
                ));
                // let v1 = Vec2::ZERO; // origin, will be placed by Transform
                // let v2 = Vec2::new(start_angle.cos(), start_angle.sin()) * anim.item_radius;
                // let v3 = Vec2::new(end_angle.cos(), end_angle.sin()) * anim.item_radius;
                
                // let mut mesh = Mesh::new(
                //     PrimitiveTopology::TriangleList,
                //     RenderAssetUsages::default(),
                // );
                // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
                //     [v1.x, v1.y, 0.0],
                //     [v2.x, v2.y, 0.0],
                //     [v3.x, v3.y, 0.0],
                // ]);
                // mesh.insert_indices(Indices::U32(vec![0, 1, 2]));
                
                // commands.spawn((
                //     Mesh2d(meshes.add(mesh)),
                //     MeshMaterial2d(materials.add(Color::srgba(1.0, 0.0, 0.0, 0.5))),
                //     Transform::from_translation(parent_pos.extend(100.0)),
                // ));
            }
            
        }
        if anim.hit_triggered && anim.progress >= 1.0 {
            commands.entity(entity).remove::<AttackAnimation>();
        }
    } 
}

