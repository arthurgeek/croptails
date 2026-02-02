use super::{
    components::{Moving, Player, PlayerAnimation, PlayerAtlasKind},
    resources::{PlayerActionsAtlas, PlayerAtlas},
};
use crate::{
    core::components::{Speed, SpriteAnimation},
    player::resources::PlayerDirection,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

/// Loads the base player spritesheet (idle animations).
pub fn load_player_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("game/characters/basic_character_spritesheet.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(48),
        4,
        4,
        None,
        None,
    ));
    commands.insert_resource(PlayerAtlas { texture, layout });
}

/// Loads the actions spritesheet (tiling, etc).
pub fn load_player_actions_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("game/characters/basic_character_actions.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(48),
        2,
        12,
        None,
        None,
    ));
    commands.insert_resource(PlayerActionsAtlas { texture, layout });
}

/// Spawns the player at the designated spawn point defined on Tiled.
pub fn spawn_player_at_spawn_point(
    mut commands: Commands,
    spawn_points: Query<(&TiledName, &Transform), Added<TiledObject>>,
) {
    for (name, transform) in &spawn_points {
        if name.0.as_str() == "PlayerSpawn" {
            let mut player_transform = *transform;
            player_transform.translation.z = 10.0;
            commands.spawn((Player, player_transform));
        }
    }
}

/// Reads keyboard input and updates PlayerDirection resource.
pub fn detect_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut direction: ResMut<PlayerDirection>,
) {
    // Direction - overwrite each frame
    direction.0 = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.0.y += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.0.y -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.0.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.0.x += 1.0;
    }
}

/// Applies velocity to player based on input direction and speed.
pub fn apply_player_movement(
    direction: Res<PlayerDirection>,
    mut player: Query<(&mut LinearVelocity, &Speed), With<Player>>,
) {
    for (mut velocity, speed) in &mut player {
        velocity.0 = direction.0.normalize_or_zero() * speed.0;
    }
}

/// Adds/removes Moving marker based on input.
pub fn update_moving_state(
    mut commands: Commands,
    direction: Res<PlayerDirection>,
    player: Query<(Entity, Has<Moving>), With<Player>>,
) {
    for (entity, is_moving) in &player {
        let has_direction = direction.0 != Vec2::ZERO;

        if has_direction && !is_moving {
            commands.entity(entity).insert(Moving);
        } else if !has_direction && is_moving {
            commands.entity(entity).remove::<Moving>();
        }
    }
}

/// Switches to walking animation when player starts moving.
pub fn on_start_moving(
    direction: Res<PlayerDirection>,
    mut player: Query<&mut PlayerAnimation, Added<Moving>>,
) {
    for mut anim in &mut player {
        *anim = walking_animation_for(direction.0);
    }
}

/// Switches to idle animation when player stops moving.
pub fn on_stop_moving(
    mut player: Query<&mut PlayerAnimation, (With<Player>, Without<Moving>)>,
    mut removed: RemovedComponents<Moving>,
) {
    for entity in removed.read() {
        if let Ok(mut anim) = player.get_mut(entity) {
            *anim = (*anim).to_idle();
        }
    }
}

/// Updates walking direction while moving.
pub fn update_walking_direction(
    direction: Res<PlayerDirection>,
    mut player: Query<&mut PlayerAnimation, With<Moving>>,
) {
    for mut anim in &mut player {
        let new_anim = walking_animation_for(direction.0);
        if *anim != new_anim {
            *anim = new_anim;
        }
    }
}

fn walking_animation_for(dir: Vec2) -> PlayerAnimation {
    if dir.y > 0.0 {
        PlayerAnimation::WalkingBack
    } else if dir.y < 0.0 {
        PlayerAnimation::WalkingFront
    } else if dir.x < 0.0 {
        PlayerAnimation::WalkingLeft
    } else {
        PlayerAnimation::WalkingRight
    }
}

/// Syncs SpriteAnimation and atlas when PlayerAnimation changes.
pub fn sync_player_animation(
    mut commands: Commands,
    base_atlas: Res<PlayerAtlas>,
    actions_atlas: Res<PlayerActionsAtlas>,
    mut player: Query<(Entity, &PlayerAnimation, &mut Sprite), Changed<PlayerAnimation>>,
) {
    for (entity, anim, mut sprite) in &mut player {
        let (first, last) = anim.frames();
        commands
            .entity(entity)
            .insert(SpriteAnimation::new(first, last, 3));

        // Swap texture and layout based on which atlas this animation uses
        let (texture, layout) = match anim.atlas_kind() {
            PlayerAtlasKind::Base => (base_atlas.texture.clone(), base_atlas.layout.clone()),
            PlayerAtlasKind::Actions => {
                (actions_atlas.texture.clone(), actions_atlas.layout.clone())
            }
        };

        sprite.image = texture;
        // Reset sprite to first frame of new animation
        sprite.texture_atlas = Some(TextureAtlas {
            layout,
            index: first,
        });
    }
}
