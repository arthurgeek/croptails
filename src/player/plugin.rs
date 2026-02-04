use super::{
    components::{Busy, Chopping, EquippedTool, Moving, Player, PlayerAnimation, Tiling, Watering},
    resources::{PlayerActionsAtlas, PlayerAtlas, PlayerDirection},
    systems::{
        activate_tool_on_chopping, apply_player_movement, deactivate_tool_on_chopping_end,
        detect_player_input, handle_tool_action, load_player_actions_atlas, load_player_atlas,
        on_start_moving, on_start_tiling, on_start_watering, on_stop_moving, position_active_tool,
        remove_chopping_on_animation_end, remove_tiling_on_animation_end,
        remove_watering_on_animation_end, spawn_player_at_spawn_point, sync_player_animation,
        update_animation_on_chopping, update_moving_state, update_walking_direction,
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
            .register_type::<Busy>()
            .register_type::<Chopping>()
            .register_type::<Tiling>()
            .register_type::<Watering>()
            .register_type::<EquippedTool>()
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
            // Input runs in Update (every frame) for responsive input
            .add_systems(Update, (detect_player_input, handle_tool_action))
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
                        (
                            on_start_moving,
                            on_stop_moving,
                            update_animation_on_chopping,
                            activate_tool_on_chopping,
                            on_start_tiling,
                            on_start_watering,
                            remove_chopping_on_animation_end,
                            deactivate_tool_on_chopping_end,
                            remove_tiling_on_animation_end,
                            remove_watering_on_animation_end,
                            update_walking_direction.run_if(resource_changed::<PlayerDirection>),
                            position_active_tool.after(update_animation_on_chopping),
                        ),
                        sync_player_animation,
                    )
                        .chain()
                        .in_set(PlayerSystemSet::Animation),
                ),
            );
    }
}
