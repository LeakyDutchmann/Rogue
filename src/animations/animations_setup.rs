use super::*;

pub fn animate_sprite(
    time: Res<Time>,
    sprites: Res<AnimationSet>,
    mut query: Query<(&ActiveAnimation, &mut AnimationTimer, &mut Sprite)>,
) {
    for (animation_type, mut timer, mut sprite) in &mut query {
        if let Some(&(first, last)) = sprites.indices.get(&animation_type.current) {
            
            timer.tick(time.delta());
            if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == last {
                first
            } else {
                atlas.index + 1
            };
        }
            
        }
        
    }
}

pub fn update_animation(
    mut query: Query<(&mut ActiveAnimation, &ActorState, &FacingDirection)>,
) {
    for (mut animation, actor_state, facing_dir) in query.iter_mut() {
        let new_animation = if actor_state.state == ActorStateType::Idle {
            AnimationId::idle_from(facing_dir.facing)
        } else if actor_state.state == ActorStateType::Walking {
            AnimationId::walk_from(facing_dir.facing)
        } else if actor_state.state == ActorStateType::Hurt {
            AnimationId::hurt_from(facing_dir.facing)
        } else {
            AnimationId::IdleRight
        };
        if new_animation != animation.current {
             animation.set_animation(new_animation);
        }
       
    }
}

pub fn reset_animation_index(
    sprites: Res<AnimationSet>,
    mut query: Query<(&mut Sprite, &mut AnimationTimer, &ActiveAnimation), Changed<ActiveAnimation>>,
) {
    for (mut sprite, mut timer, animation) in &mut query {
        if let Some(&(first, _last)) = sprites.indices.get(&animation.current) {
            if let Some(sprite) = &mut sprite.texture_atlas {
                sprite.index = first;
                timer.reset();
            }         
        }
    }
}

pub fn attack_animation(
    mut query: Query<(&mut Transform, &mut AttackAnimation, &ChildOf, &mut Sprite)>,
    parent_tf: Query<&Transform, Without<AttackAnimation>>,
) {
    for (mut transform, anim, child_of, mut sprite) in query.iter_mut() {
        if let Ok(parent_tf) = parent_tf.get(child_of.0) {
            let cursor_pos = anim.cursor_pos;
            let player_pos = parent_tf.translation;
            let to_cursor = (cursor_pos - player_pos.xy()).normalize();
            let to_cursor_angle = to_cursor.to_angle();
            let offset = 20.0;
            match anim.anim_pattern {
                AnimationStyle::Sword => {
                    if cursor_pos.x <= player_pos.x {
                       
                        let start_angle = to_cursor_angle - anim.max_angle / 2.0;
                        let end_angle = to_cursor_angle + anim.max_angle / 2.0;
                        let angle = start_angle + (end_angle - start_angle) * anim.progress;
                        
                        let point_x = offset * angle.cos();
                        let point_y = offset * angle.sin();
                        
                        sprite.flip_x = true;
                        transform.translation.x = point_x;
                        transform.translation.y = point_y;
                        
                        transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);  
                    } else {
                        
                        let start_angle = to_cursor_angle + anim.max_angle / 2.0;
                        let end_angle = to_cursor_angle - anim.max_angle / 2.0;
                        let angle = start_angle + (end_angle - start_angle) * anim.progress;
                        let point_x = offset * angle.cos();
                        let point_y = offset * angle.sin();
                        
                        sprite.flip_x = false;
                        transform.translation.x = point_x;
                        transform.translation.y = point_y;
                        
                        transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
                        
                    }
                }
                AnimationStyle::PickAxe => {
                    if cursor_pos.x <= player_pos.x {
                       
                        let start_angle = to_cursor_angle - anim.max_angle / 2.0;
                        let end_angle = to_cursor_angle + anim.max_angle / 2.0;
                        let angle = start_angle + (end_angle - start_angle) * anim.progress;                     
                        sprite.flip_x = true;
                        transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);  
                    } else {
                        
                        let start_angle = to_cursor_angle + anim.max_angle / 2.0;
                        let end_angle = to_cursor_angle - anim.max_angle / 2.0;
                        let angle = start_angle + (end_angle - start_angle) * anim.progress;                    
                        sprite.flip_x = false;                      
                        transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
                        
                    }
                }
            }
        }    
    }
} 

//i should repalace ActorState with other compomnent here
pub fn auto_zorder(
    mut movers: Query<&mut Transform, With<ActorState>>,
) {
    for mut tf in movers.iter_mut() {
         tf.translation.z = -tf.translation.y * 0.001;
    }
}

pub fn update_held_item_dir(
    mut held_item: Query<(&mut Transform, &ChildOf, &mut Sprite), (With<HeldItem>, Without<AttackAnimation>)>,
    facing_qr: Query<&FacingDirection>,
) {
    for (mut hand_pos, childof, mut sprite) in held_item.iter_mut() {
        if let Ok(facing) = facing_qr.get(childof.0) {
            match facing.facing {
                Facing::Up => {
                    hand_pos.translation = Vec3::new(0.0, 0.0, -1.0);
                    hand_pos.rotation = Quat::from_rotation_z((30.0_f32).to_radians());
                },
                Facing::Down => {
                    hand_pos.translation = Vec3::new(0.0, 0.0, 1.0);
                    hand_pos.rotation = Quat::from_rotation_z((30.0_f32).to_radians());
                },
                Facing::Left => {
                    hand_pos.translation = Vec3::new(0.0, 0.0, 1.0);
                    hand_pos.rotation = Quat::from_rotation_z(-(30.0_f32).to_radians());
                    sprite.flip_x = true;
                },
                Facing::Right => {
                    hand_pos.translation = Vec3::new(0.0, 0.0, 1.0);
                    hand_pos.rotation = Quat::from_rotation_z((30.0_f32).to_radians());
                },
            }
        }
    }
}

pub fn draw_helditem(
    mut commands: Commands,
    mut held_item: Query<(Entity, &mut HeldItem, &ChildOf), (With<HeldItem>, Without<AttackAnimation>)>,
    parent: Query<&ActorState>,
    registry: Res<ItemRegistry>,
) {
    for (actor_hand, held_item, childof) in held_item.iter_mut() {
        if let Ok(actor_state) = parent.get(childof.0) {
            if actor_state.state != ActorStateType::Dead {
                if let Some(held) = held_item.held.as_ref() {
                    if let Some(last_held) = held_item.last_held.as_ref() {
                        if held != last_held {
                            if let Some(def) = registry.items.get(held) {
                                commands.entity(actor_hand).insert(Sprite::from_image(def.sprite.clone()));
                            }
                        }
                    } else {
                        if let Some(def) = registry.items.get(held) {
                            commands.entity(actor_hand).insert(Sprite::from_image(def.sprite.clone()));
                        }
                        
                    }
                     
                } else {
                    commands.entity(actor_hand).remove::<Sprite>();
                }
            } 
        } else {
            println!("didn't get actorstate");
        }
    }
}