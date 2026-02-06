use bevy::prelude::*;

/// Marker for all NPC entities that use navigation regions.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Npc;

/// Configuration for wandering behavior.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WanderConfig {
    /// Minimum idle duration in seconds.
    pub min_idle_time: f32,
    /// Maximum idle duration in seconds.
    pub max_idle_time: f32,
    /// Minimum walk speed.
    pub min_speed: f32,
    /// Maximum walk speed.
    pub max_speed: f32,
}

impl Default for WanderConfig {
    fn default() -> Self {
        Self {
            min_idle_time: 1.0,
            max_idle_time: 5.0,
            min_speed: 5.0,
            max_speed: 10.0,
        }
    }
}

/// Configuration for walk cycles - how many trips before going idle.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WalkCycles {
    pub min: u32,
    pub max: u32,
}

impl Default for WalkCycles {
    fn default() -> Self {
        Self { min: 2, max: 6 }
    }
}

/// Runtime state tracking current walk cycle progress.
/// Added when walking starts, removed when going idle.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct WalkCycleProgress {
    pub current: u32,
    pub target: u32,
}

/// Timer for idle state duration.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IdleTimer(pub Timer);

impl Default for IdleTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(5.0, TimerMode::Once))
    }
}
