use bevy::{ecs::component::Mutable, prelude::*};
use std::time::Duration;

/// Sprite animation with optional looping (sequential frames).
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

/// Sprite animation with non-sequential frame list.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SequenceAnimation {
    pub frames: Vec<usize>,
    pub current: usize,
    pub timer: Timer,
    pub looping: bool,
}

impl SequenceAnimation {
    /// Looping sequence animation.
    pub fn new(frames: Vec<usize>, fps: u8) -> Self {
        Self {
            frames,
            current: 0,
            timer: Timer::new(
                Duration::from_secs_f32(1.0 / fps as f32),
                TimerMode::Repeating,
            ),
            looping: true,
        }
    }

    /// One-shot sequence animation (plays once, then adds AnimationFinished).
    pub fn once(frames: Vec<usize>, fps: u8) -> Self {
        Self {
            looping: false,
            ..Self::new(frames, fps)
        }
    }

    pub fn current_frame(&self) -> Option<usize> {
        self.frames.get(self.current).copied()
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

/// Health for damageable targets (trees, rocks, etc.).
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 3.0,
            max: 3.0,
        }
    }
}

/// Damage dealt by tools (axe, etc.).
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Damage(pub f32);

/// Marker: tool is currently active (during tool action).
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Active;

/// Marker for layers/entities with Y-sorting enabled.
/// `offset` is added to Y before calculating Z (use for foot position adjustment).
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct YSort {
    pub offset: f32,
}

/// Marker: entity is currently moving.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Moving;

/// Trait for animation enums that support idle/walk states.
pub trait CharacterAnimation: Component<Mutability = Mutable> + Copy + PartialEq {
    /// Returns (first_frame, last_frame) for this animation state.
    fn frames(self) -> (usize, usize);
    /// Whether this animation loops.
    fn loops(self) -> bool {
        true
    }
    /// Frames per second for this animation.
    fn fps(self) -> u8 {
        5
    }
    /// Convert to idle animation.
    fn to_idle(self) -> Self;
    /// Convert to walk animation.
    fn to_walk(self) -> Self;
}
