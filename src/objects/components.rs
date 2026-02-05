use super::resources::{ObjectsAtlas, tiles};
use crate::{
    core::components::Health,
    physics::GameLayer,
    tools::components::{Axe, ToolTarget},
};
use avian2d::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

/// Marker for all object entities (Tree, Rock, Crop, etc.).
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Object;

/// Tree size variants with different properties.
#[derive(Reflect, Default, Clone, Copy, PartialEq, Eq)]
pub enum TreeVariant {
    #[default]
    Small,
    Large,
}

impl TreeVariant {
    pub fn collider_size(&self) -> Vec2 {
        match self {
            Self::Small => Vec2::new(10.0, 18.0),
            Self::Large => Vec2::new(12.0, 20.0),
        }
    }

    pub fn collider_offset(&self) -> Vec3 {
        match self {
            Self::Small => Vec3::new(8.0, 15.0, 0.0),
            Self::Large => Vec3::new(16.0, 12.0, 0.0),
        }
    }

    pub fn health(&self) -> f32 {
        match self {
            Self::Small => 3.0,
            Self::Large => 5.0,
        }
    }

    pub fn log_offsets(&self) -> &'static [Vec3] {
        const SMALL: &[Vec3] = &[Vec3::new(8.0, 8.0, 0.0)];
        const LARGE: &[Vec3] = &[Vec3::new(4.0, 20.0, 0.0), Vec3::new(28.0, 20.0, 0.0)];
        match self {
            Self::Small => SMALL,
            Self::Large => LARGE,
        }
    }
}

/// A tree that can be chopped down.
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(Name = "Tree", Health, Object)]
#[component(on_add = Self::on_add)]
pub struct Tree {
    pub variant: TreeVariant,
}

impl Tree {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;
        let variant = world
            .get::<Tree>(entity)
            .map(|t| t.variant)
            .unwrap_or_default();

        let size = variant.collider_size();
        let offset = variant.collider_offset();

        world.commands().entity(entity).with_child((
            ToolTarget::<Axe>::new(),
            Collider::rectangle(size.x, size.y),
            Transform::from_translation(offset),
        ));
    }
}

/// Marker for items that can be collected by the player.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Collectable;

/// A collectable log dropped by trees.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name = "Log",
    Collectable,
    Sensor,
    Collider = Collider::rectangle(8.0, 6.0),
    CollisionLayers = CollisionLayers::new(GameLayer::Collectable, GameLayer::Player),
    CollisionEventsEnabled,
)]
#[component(on_add = Self::on_add)]
pub struct Log;

impl Log {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let atlas = world.resource::<ObjectsAtlas>();
        let sprite = Sprite::from_atlas_image(
            atlas.texture.clone(),
            TextureAtlas {
                layout: atlas.layout.clone(),
                index: tiles::LOG,
            },
        );
        world.commands().entity(ctx.entity).insert(sprite);
    }
}

/// A rock that can be broken with an axe.
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(Name = "Rock", Health, Object)]
#[component(on_add = Self::on_add)]
pub struct Rock;

impl Rock {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().entity(ctx.entity).with_child((
            ToolTarget::<Axe>::new(),
            Collider::rectangle(14.0, 10.0),
            Transform::from_translation(Vec3::new(8.0, 6.0, 0.0)),
        ));
    }
}

/// A collectable stone dropped by rocks.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name = "Stone",
    Collectable,
    Sensor,
    Collider = Collider::rectangle(8.0, 6.0),
    CollisionLayers = CollisionLayers::new(GameLayer::Collectable, GameLayer::Player),
    CollisionEventsEnabled,
)]
#[component(on_add = Self::on_add)]
pub struct Stone;

impl Stone {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let atlas = world.resource::<ObjectsAtlas>();
        let sprite = Sprite::from_atlas_image(
            atlas.texture.clone(),
            TextureAtlas {
                layout: atlas.layout.clone(),
                index: tiles::STONE,
            },
        );
        world.commands().entity(ctx.entity).insert(sprite);
    }
}
