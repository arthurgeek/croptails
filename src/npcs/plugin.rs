use super::{
    components::{IdleTimer, Npc, WalkCycleProgress, WalkCycles, WanderConfig},
    systems::{
        apply_npc_movement, flip_npc_sprite, idle_to_walk_transition, on_waypoint_arrival,
        stop_npc_movement, tick_idle_timer,
    },
};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum NpcSystemSet {
    /// AI state machine updates
    StateMachine,
    /// Movement
    Movement,
}

pub struct NpcsPlugin;

impl Plugin for NpcsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Npc>()
            .register_type::<WanderConfig>()
            .register_type::<IdleTimer>()
            .register_type::<WalkCycles>()
            .register_type::<WalkCycleProgress>()
            // Configure system sets
            .configure_sets(
                FixedUpdate,
                (NpcSystemSet::StateMachine, NpcSystemSet::Movement).chain(),
            )
            // State machine (FixedUpdate for determinism)
            .add_systems(
                FixedUpdate,
                (
                    tick_idle_timer,
                    idle_to_walk_transition,
                    on_waypoint_arrival,
                )
                    .chain()
                    .in_set(NpcSystemSet::StateMachine),
            )
            // Movement (FixedUpdate, synced with physics)
            .add_systems(
                FixedUpdate,
                (apply_npc_movement, stop_npc_movement, flip_npc_sprite)
                    .in_set(NpcSystemSet::Movement),
            );
    }
}
