use bevy::prelude::*;


fn animate_sprite(
    time: Res<Time>,
    sprites: Res<SpriteSheetIndices>,
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