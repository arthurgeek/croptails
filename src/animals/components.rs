use super::resources::{ChickenAtlas, CowAtlas};
use crate::{
    core::components::{CharacterAnimation, Speed, YSort},
    npcs::components::{IdleTimer, Npc, WalkCycles, WanderConfig},
    physics::GameLayer,
};
use avian2d::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
    sprite::Anchor,
};

// ─────────────────────────────────────────────────────────────────────────────
// Chicken
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name = "Chicken",
    Npc,
    Sprite,
    ChickenAnimation,
    WanderConfig,
    WalkCycles,
    IdleTimer,
    RigidBody::Dynamic,
    LockedAxes::ROTATION_LOCKED,
    Speed,  // Set dynamically per walk cycle from WanderConfig
    YSort = YSort { offset: -4.0 },
)]
#[component(on_add = Self::on_add)]
pub struct Chicken;

impl Chicken {
    const COLLIDER_OFFSET_Y: f32 = -2.0;

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;

        // Get atlas resource
        let atlas = world
            .get_resource::<ChickenAtlas>()
            .expect("ChickenAtlas must be present");

        let texture = atlas.texture.clone();
        let layout = atlas.layout.clone();

        // Configure sprite
        if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
            sprite.image = texture;
            sprite.texture_atlas = Some(TextureAtlas { layout, index: 0 });
        }

        // Set anchor at bottom-left to match Tiled tile object placement
        world.commands().entity(entity).insert(Anchor::BOTTOM_LEFT);

        // Spawn collider as child (at chicken's feet)
        // No NavMeshObstacle - chickens use real-time separation instead
        world.commands().entity(entity).with_child((
            Collider::circle(3.0),
            Transform::from_translation(Vec3::new(0.0, Self::COLLIDER_OFFSET_Y, 0.0)),
            CollisionLayers::new(
                GameLayer::Npc,
                [GameLayer::Default, GameLayer::Player, GameLayer::Npc],
            ),
        ));
    }
}

/// Animation states for chickens.
#[derive(Component, Reflect, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum ChickenAnimation {
    #[default]
    Idle, // frames 0-1
    Walk, // frames 4-7
}

impl CharacterAnimation for ChickenAnimation {
    fn frames(self) -> (usize, usize) {
        match self {
            Self::Idle => (0, 1),
            Self::Walk => (4, 7),
        }
    }

    fn fps(self) -> u8 {
        5
    }

    fn to_idle(self) -> Self {
        Self::Idle
    }

    fn to_walk(self) -> Self {
        Self::Walk
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Cow
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name = "Cow",
    Npc,
    Sprite,
    CowAnimation,
    WanderConfig,
    WalkCycles,
    IdleTimer,
    RigidBody::Dynamic,
    LockedAxes::ROTATION_LOCKED,
    Speed,  // Set dynamically per walk cycle from WanderConfig
    YSort = YSort { offset: -8.0 },
)]
#[component(on_add = Self::on_add)]
pub struct Cow;

impl Cow {
    const COLLIDER_OFFSET_Y: f32 = 2.0;

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let entity = ctx.entity;

        // Get atlas resource
        let atlas = world
            .get_resource::<CowAtlas>()
            .expect("CowAtlas must be present");

        let texture = atlas.texture.clone();
        let layout = atlas.layout.clone();

        // Configure sprite
        if let Some(mut sprite) = world.get_mut::<Sprite>(entity) {
            sprite.image = texture;
            sprite.texture_atlas = Some(TextureAtlas { layout, index: 0 });
        }

        // Custom anchor: cow artwork is bottom-left of 32x32 frame
        world
            .commands()
            .entity(entity)
            .insert(Anchor(Vec2::new(0.0, -0.25)));

        // Spawn collider as child (at cow's feet)
        world.commands().entity(entity).with_child((
            Collider::circle(6.0),
            Transform::from_translation(Vec3::new(0.0, Self::COLLIDER_OFFSET_Y, 0.0)),
            CollisionLayers::new(
                GameLayer::Npc,
                [GameLayer::Default, GameLayer::Player, GameLayer::Npc],
            ),
        ));
    }
}

/// Animation states for cows.
#[derive(Component, Reflect, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum CowAnimation {
    #[default]
    Idle, // frames 0-2 (row 0, cols 0-2)
    Walk, // frames 5-6 (row 1, cols 0-1)
}

impl CharacterAnimation for CowAnimation {
    fn frames(self) -> (usize, usize) {
        match self {
            Self::Idle => (0, 2),
            Self::Walk => (5, 6),
        }
    }

    fn fps(self) -> u8 {
        5
    }

    fn to_idle(self) -> Self {
        Self::Idle
    }

    fn to_walk(self) -> Self {
        Self::Walk
    }
}
