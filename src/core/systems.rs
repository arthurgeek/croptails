use super::components::{AnimationFinished, SequenceAnimation, SpriteAnimation, YSort};
use bevy::prelude::*;
use moonshine_kind::Instance;

/// Updates Z coordinate based on Y for entities in Y-sorted layers or with YSort directly.
pub fn apply_y_sort(
    y_sorted_layers: Query<(Instance<YSort>, &YSort)>,
    mut children: Query<(&ChildOf, &mut Transform), Without<YSort>>,
    mut y_sorted_entities: Query<(&mut Transform, &YSort)>,
) {
    // Y-sort children of YSort layers
    for (child_of, mut transform) in &mut children {
        if let Ok((_, y_sort)) = y_sorted_layers.get(child_of.parent()) {
            transform.translation.z = -(transform.translation.y + y_sort.offset);
        }
    }

    // Y-sort entities with YSort directly
    for (mut transform, y_sort) in &mut y_sorted_entities {
        transform.translation.z = -(transform.translation.y + y_sort.offset);
    }
}

/// Ticks sprite animations and advances frames (sequential).
pub fn animate_sprites(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Instance<Sprite>, &mut SpriteAnimation, &mut Sprite),
        Without<AnimationFinished>,
    >,
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
                    commands.entity(entity.entity()).insert(AnimationFinished);
                }
            } else {
                atlas.index += 1;
            }
        }
    }
}

/// Ticks sequence animations and advances frames (non-sequential).
pub fn animate_sequences(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Instance<Sprite>, &mut SequenceAnimation, &mut Sprite),
        Without<AnimationFinished>,
    >,
) {
    for (entity, mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            anim.current += 1;

            if let Some(frame) = anim.current_frame() {
                atlas.index = frame;
            } else if anim.looping {
                anim.current = 0;
                if let Some(frame) = anim.current_frame() {
                    atlas.index = frame;
                }
            } else {
                // One-shot finished
                commands.entity(entity.entity()).insert(AnimationFinished);
            }
        }
    }
}
