use super::{
    components::{Moving, Player, PlayerAnimation},
    resources::{PlayerActionsAtlas, PlayerAtlas, PlayerDirection},
    systems::{
        apply_player_movement, detect_player_input, load_player_actions_atlas, load_player_atlas,
        on_start_moving, on_stop_moving, spawn_player_at_spawn_point, sync_player_animation,
        update_moving_state, update_walking_direction,
    },
};
use bevy::prelude::*;

/// System sets for player operations with better parallelization
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerSystemSet {
    /// Physics and movement (runs in FixedUpdate)
    Movement,
    /// Animation updates (runs after movement)
    Animation,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerAtlas>()
            .register_type::<PlayerActionsAtlas>()
            .register_type::<Player>()
            .register_type::<Moving>()
            .register_type::<PlayerAnimation>()
            .register_type::<PlayerDirection>()
            .init_resource::<PlayerDirection>()
            // Configure set ordering for FixedUpdate
            .configure_sets(
                FixedUpdate,
                (PlayerSystemSet::Movement, PlayerSystemSet::Animation).chain(),
            )
            .add_systems(Startup, (load_player_atlas, load_player_actions_atlas))
            .add_systems(Update, spawn_player_at_spawn_point)
            // Input detection runs in Update (every frame) for responsive input
            .add_systems(Update, detect_player_input)
            // Movement and animation run in FixedUpdate (synced with physics)
            .add_systems(
                FixedUpdate,
                (
                    (
                        apply_player_movement,
                        update_moving_state.run_if(resource_changed::<PlayerDirection>),
                    )
                        .chain()
                        .in_set(PlayerSystemSet::Movement),
                    (
                        on_start_moving,
                        on_stop_moving,
                        update_walking_direction.run_if(resource_changed::<PlayerDirection>),
                        sync_player_animation,
                    )
                        .chain()
                        .in_set(PlayerSystemSet::Animation),
                ),
            );
    }
}
