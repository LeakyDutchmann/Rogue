use crate::animations::*;

pub fn animate_sprite(
    time: Res<Time>,
    sprites: Res<AnimationSet>,
    mut query: Query<(&ActiveAnimation, &mut AnimationTimer, &mut Sprite)>,
) {
    for (animation_type, mut timer, mut sprite) in &mut query {
        if let Some(&(first, last)) = sprites.indices.get(&animation_type.current) {
            
            timer.tick(time.delta());
            // I read it that there are repeating timers in Bevy?
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
        // You can use `match` instead of `if`.
        let new_animation = if player.state == PlayerState::Idle {
            // Looks like AnimationId::idle_from(Facing f) method
            match player.facing {
                Facing::Right =>  AnimationId::IdleRight,
                Facing::Left =>  AnimationId::IdleLeft,
                Facing::Up =>  AnimationId::IdleRight,
                Facing::Down =>  AnimationId::IdleRight,
            }
        } else if player.state == PlayerState::Walking {
            // Looks like AnimationId::walk_from(Facing f) method
            match player.facing {
                Facing::Right =>  AnimationId::WalkRight,
                Facing::Left =>  AnimationId::WalkLeft,
                Facing::Up =>  AnimationId::WalkUp,
                Facing::Down =>  AnimationId::WalkDown,
            }
        } else {
            AnimationId::IdleRight
        };

        // Looks like a method, ActiveAnimation::set_animation().
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
    // I fail to understand why is this system needed at all. Maybe it's me.
    for (mut sprite, mut timer, animation) in &mut query {
        if let Some(&(first, _last)) = sprites.indices.get(&animation.current) {
            if let Some(sprite) = &mut sprite.texture_atlas {
                sprite.index = first;
                timer.reset();
            }         
        }
    }
}
