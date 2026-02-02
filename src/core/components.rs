use bevy::prelude::*;
use std::time::Duration;

/// Sprite animation with optional looping.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpriteAnimation {
    pub first: usize,
    pub last: usize,
    pub timer: Timer,
    pub looping: bool,
}

impl SpriteAnimation {
    /// Looping animation.
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first,
            last,
            timer: Timer::new(
                Duration::from_secs_f32(1.0 / fps as f32),
                TimerMode::Repeating,
            ),
            looping: true,
        }
    }

    /// One-shot animation (plays once, then adds AnimationFinished).
    pub fn once(first: usize, last: usize, fps: u8) -> Self {
        Self {
            looping: false,
            ..Self::new(first, last, fps)
        }
    }
}

/// Marker: animation has finished (for one-shot animations).
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AnimationFinished;

/// Speed component to define movement speed of an entity.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Speed(pub f32);
