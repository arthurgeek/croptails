use super::resources::{tiles, ObjectsAtlas};
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

/// Marker for objects pending despawn (health <= 0).
/// Despawned when the hitting animation finishes.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PendingDespawn;

/// A tree that can be chopped down.
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(Name = "Tree", Health, Object)]
#[component(on_add = Self::on_add)]
pub struct Tree;

impl Tree {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;

        // Spawn ToolTarget<Axe> child with tree-specific collider
        // Transform offsets collider to align with tree trunk
        world.commands().entity(entity).with_child((
            ToolTarget::<Axe>::new(),
            Collider::rectangle(10.0, 18.0),
            Transform::from_xyz(8.0, 15.0, 0.0),
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
