use super::resources::PlayerAtlas;
use crate::core::components::Speed;
use avian2d::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name = "Player",
    Sprite,
    PlayerAnimation,
    EquippedTool,
    RigidBody::Dynamic,
    LockedAxes::ROTATION_LOCKED,
    Collider = Collider::capsule(3.0, 5.0),
    Speed = Speed(50.0),
  )]
#[component(on_add = Self::on_add)]
pub struct Player;

impl Player {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;

        // Get the atlas resource
        let atlas = world
            .get_resource::<PlayerAtlas>()
            .expect("PlayerAtlas resource must be present");

        let texture = atlas.texture.clone();
        let layout = atlas.layout.clone();

        // Configure the sprite
        if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
            sprite.image = texture;
            sprite.texture_atlas = Some(TextureAtlas { layout, index: 0 });
        }
    }
}

#[derive(Component, Reflect, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum PlayerAnimation {
    #[default]
    IdleFront,
    IdleBack,
    IdleLeft,
    IdleRight,
    WalkingFront,
    WalkingBack,
    WalkingLeft,
    WalkingRight,
    TilingFront,
    TilingBack,
    TilingLeft,
    TilingRight,
    ChoppingFront,
    ChoppingBack,
    ChoppingLeft,
    ChoppingRight,
    WateringFront,
    WateringBack,
    WateringLeft,
    WateringRight,
}

/// Which atlas a player animation uses.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerAtlasKind {
    Base,
    Actions,
}

/// Marker: player is currently moving.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Moving;

/// Marker: player is busy (using tool, etc.) - locks movement.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[component(on_add = Self::on_add)]
pub struct Busy;

impl Busy {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;
        // Stop movement when busy
        if let Some(mut velocity) = world.get_mut::<LinearVelocity>(entity) {
            velocity.0 = Vec2::ZERO;
        }
        world.commands().entity(entity).remove::<Moving>();
    }
}

/// Marker: player is currently chopping.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Busy)]
#[component(on_remove = Self::on_remove)]
pub struct Chopping;

impl Chopping {
    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().entity(ctx.entity).remove::<Busy>();
    }
}

/// Marker: player is currently tiling.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Busy)]
#[component(on_remove = Self::on_remove)]
pub struct Tiling;

impl Tiling {
    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().entity(ctx.entity).remove::<Busy>();
    }
}

/// Marker: player is currently watering.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Busy)]
#[component(on_remove = Self::on_remove)]
pub struct Watering;

impl Watering {
    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().entity(ctx.entity).remove::<Busy>();
    }
}

/// Currently equipped tool (editable as dropdown in inspector).
#[derive(Component, Reflect, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum EquippedTool {
    #[default]
    None,
    Axe,
    Hoe,
    WateringCan,
}

impl PlayerAnimation {
    pub fn frames(self) -> (usize, usize) {
        match self {
            Self::IdleFront => (0, 1),
            Self::IdleBack => (4, 5),
            Self::IdleLeft => (8, 9),
            Self::IdleRight => (12, 13),
            Self::WalkingFront => (2, 3),
            Self::WalkingBack => (6, 7),
            Self::WalkingLeft => (10, 11),
            Self::WalkingRight => (14, 15),
            Self::TilingFront => (0, 1),
            Self::TilingBack => (2, 3),
            Self::TilingLeft => (4, 5),
            Self::TilingRight => (6, 7),
            Self::ChoppingFront => (8, 9),
            Self::ChoppingBack => (10, 11),
            Self::ChoppingLeft => (12, 13),
            Self::ChoppingRight => (14, 15),
            Self::WateringFront => (16, 17),
            Self::WateringBack => (18, 19),
            Self::WateringLeft => (20, 21),
            Self::WateringRight => (22, 23),
        }
    }

    pub fn atlas_kind(self) -> PlayerAtlasKind {
        match self {
            Self::IdleFront
            | Self::IdleBack
            | Self::IdleLeft
            | Self::IdleRight
            | Self::WalkingFront
            | Self::WalkingBack
            | Self::WalkingLeft
            | Self::WalkingRight => PlayerAtlasKind::Base,
            _ => PlayerAtlasKind::Actions,
        }
    }

    /// Whether this animation loops or plays once.
    pub fn loops(self) -> bool {
        matches!(
            self,
            Self::IdleFront
                | Self::IdleBack
                | Self::IdleLeft
                | Self::IdleRight
                | Self::WalkingFront
                | Self::WalkingBack
                | Self::WalkingLeft
                | Self::WalkingRight
        )
    }

    /// Convert to idle animation, preserving direction.
    pub fn to_idle(self) -> Self {
        match self {
            Self::WalkingFront | Self::ChoppingFront | Self::TilingFront | Self::WateringFront => {
                Self::IdleFront
            }
            Self::WalkingBack | Self::ChoppingBack | Self::TilingBack | Self::WateringBack => {
                Self::IdleBack
            }
            Self::WalkingLeft | Self::ChoppingLeft | Self::TilingLeft | Self::WateringLeft => {
                Self::IdleLeft
            }
            Self::WalkingRight | Self::ChoppingRight | Self::TilingRight | Self::WateringRight => {
                Self::IdleRight
            }
            other => other,
        }
    }

    /// Convert to chopping animation, preserving direction.
    pub fn to_chopping(self) -> Self {
        match self {
            Self::IdleFront | Self::WalkingFront => Self::ChoppingFront,
            Self::IdleBack | Self::WalkingBack => Self::ChoppingBack,
            Self::IdleLeft | Self::WalkingLeft => Self::ChoppingLeft,
            Self::IdleRight | Self::WalkingRight => Self::ChoppingRight,
            other => other,
        }
    }

    /// Convert to tiling animation, preserving direction.
    pub fn to_tiling(self) -> Self {
        match self {
            Self::IdleFront | Self::WalkingFront => Self::TilingFront,
            Self::IdleBack | Self::WalkingBack => Self::TilingBack,
            Self::IdleLeft | Self::WalkingLeft => Self::TilingLeft,
            Self::IdleRight | Self::WalkingRight => Self::TilingRight,
            other => other,
        }
    }

    /// Convert to watering animation, preserving direction.
    pub fn to_watering(self) -> Self {
        match self {
            Self::IdleFront | Self::WalkingFront => Self::WateringFront,
            Self::IdleBack | Self::WalkingBack => Self::WateringBack,
            Self::IdleLeft | Self::WalkingLeft => Self::WateringLeft,
            Self::IdleRight | Self::WalkingRight => Self::WateringRight,
            other => other,
        }
    }
}
