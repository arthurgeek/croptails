use super::components::{AnimationFinished, SpriteAnimation};
use bevy::prelude::*;

/// Ticks sprite animations and advances frames.
pub fn animate_sprites(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpriteAnimation, &mut Sprite), Without<AnimationFinished>>,
) {
    for (entity, mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index >= anim.last {
                if anim.looping {
                    atlas.index = anim.first;
                } else {
                    // One-shot finished
                    commands.entity(entity).insert(AnimationFinished);
                }
            } else {
                atlas.index += 1;
            }
        }
    }
}
