use super::{
    components::{
        Busy, Chopping, EquippedTool, Moving, Player, PlayerAnimation, PlayerAtlasKind, Tiling,
        Watering,
    },
    resources::{PlayerActionsAtlas, PlayerAtlas},
};
use crate::{
    core::components::{AnimationFinished, Speed, SpriteAnimation},
    player::resources::PlayerDirection,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use moonshine_kind::Instance;

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
/// Skipped when player is Busy (using tool).
pub fn apply_player_movement(
    direction: Res<PlayerDirection>,
    mut player: Query<(&mut LinearVelocity, &Speed), (With<Player>, Without<Busy>)>,
) {
    for (mut velocity, speed) in &mut player {
        velocity.0 = direction.0.normalize_or_zero() * speed.0;
    }
}

/// Adds/removes Moving marker based on input.
/// Skipped when player is Busy (using tool).
pub fn update_moving_state(
    mut commands: Commands,
    direction: Res<PlayerDirection>,
    player: Query<(Instance<Player>, Has<Moving>), Without<Busy>>,
) {
    for (player, is_moving) in &player {
        let has_direction = direction.0 != Vec2::ZERO;

        if has_direction && !is_moving {
            commands.entity(player.entity()).insert(Moving);
        } else if !has_direction && is_moving {
            commands.entity(player.entity()).remove::<Moving>();
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

/// Triggers tool action when Space is pressed.
pub fn handle_tool_action(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Query<(Instance<Player>, &EquippedTool, Has<Busy>)>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    for (player, tool, is_busy) in &player {
        // Only trigger if not already performing an action
        if is_busy {
            continue;
        }

        match tool {
            EquippedTool::Axe => {
                commands.entity(player.entity()).insert(Chopping);
            }
            EquippedTool::Hoe => {
                commands.entity(player.entity()).insert(Tiling);
            }
            EquippedTool::WateringCan => {
                commands.entity(player.entity()).insert(Watering);
            }
            EquippedTool::None => {}
        }
    }
}

/// Switches to chopping animation when Chopping is added.
pub fn on_start_chopping(mut player: Query<&mut PlayerAnimation, Added<Chopping>>) {
    for mut anim in &mut player {
        *anim = (*anim).to_chopping();
    }
}

/// Switches to tiling animation when Tiling is added.
pub fn on_start_tiling(mut player: Query<&mut PlayerAnimation, Added<Tiling>>) {
    for mut anim in &mut player {
        *anim = (*anim).to_tiling();
    }
}

/// Syncs SpriteAnimation and atlas when PlayerAnimation changes.
pub fn sync_player_animation(
    mut commands: Commands,
    base_atlas: Res<PlayerAtlas>,
    actions_atlas: Res<PlayerActionsAtlas>,
    mut player: Query<(Instance<Player>, &PlayerAnimation, &mut Sprite), Changed<PlayerAnimation>>,
) {
    for (player, anim, mut sprite) in &mut player {
        let (first, last) = anim.frames();

        // Use looping or one-shot based on animation type
        let sprite_anim = if anim.loops() {
            SpriteAnimation::new(first, last, 3)
        } else {
            SpriteAnimation::once(first, last, 3)
        };

        commands
            .entity(player.entity())
            .insert(sprite_anim)
            .remove::<AnimationFinished>(); // Clear previous finish state

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

/// Removes Chopping marker and returns to idle when animation ends.
pub fn remove_chopping_on_animation_end(
    mut commands: Commands,
    mut player: Query<(Instance<Player>, &mut PlayerAnimation), (With<Chopping>, Added<AnimationFinished>)>,
) {
    for (player, mut anim) in &mut player {
        *anim = (*anim).to_idle();
        commands.entity(player.entity()).remove::<Chopping>();
    }
}

/// Removes Tiling marker and returns to idle when animation ends.
pub fn remove_tiling_on_animation_end(
    mut commands: Commands,
    mut player: Query<(Instance<Player>, &mut PlayerAnimation), (With<Tiling>, Added<AnimationFinished>)>,
) {
    for (player, mut anim) in &mut player {
        *anim = (*anim).to_idle();
        commands.entity(player.entity()).remove::<Tiling>();
    }
}

/// Switches to watering animation when Watering is added.
pub fn on_start_watering(mut player: Query<&mut PlayerAnimation, Added<Watering>>) {
    for mut anim in &mut player {
        *anim = (*anim).to_watering();
    }
}

/// Removes Watering marker and returns to idle when animation ends.
pub fn remove_watering_on_animation_end(
    mut commands: Commands,
    mut player: Query<(Instance<Player>, &mut PlayerAnimation), (With<Watering>, Added<AnimationFinished>)>,
) {
    for (player, mut anim) in &mut player {
        *anim = (*anim).to_idle();
        commands.entity(player.entity()).remove::<Watering>();
    }
}
