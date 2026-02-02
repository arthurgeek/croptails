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
    RigidBody::Dynamic,
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

    /// Convert to idle animation, preserving direction.
    pub fn to_idle(self) -> Self {
        match self {
            Self::WalkingFront => Self::IdleFront,
            Self::WalkingBack => Self::IdleBack,
            Self::WalkingLeft => Self::IdleLeft,
            Self::WalkingRight => Self::IdleRight,
            other => other,
        }
    }
}
