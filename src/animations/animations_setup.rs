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
    mut query: Query<(&mut ActiveAnimation, &Player)>,
) {
    for (mut animation, player) in query.iter_mut() {
        let new_animation = if player.state == PlayerState::Idle {
            AnimationId::idle_from(player.facing)
        } else if player.state == PlayerState::Walking {
            AnimationId::walk_from(player.facing)
        } else {
            AnimationId::IdleRight
        };
        
        if new_animation != animation.current {
            animation.previous = animation.current;
            animation.current = new_animation;
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
