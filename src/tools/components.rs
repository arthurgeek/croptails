use crate::{core::components::Damage, physics::GameLayer};
use avian2d::prelude::*;
use bevy::prelude::*;
use std::marker::PhantomData;

/// Trait for tool types. Bounds `ToolTarget<T>` and `Hit<T>` to only accept tools.
pub trait Tool: Component {}

/// Marker component for tool entities with shared requirements.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Collider = tool_collider(),
    Sensor,
    CollisionEventsEnabled,
    CollisionLayers = tool_collision_layers(),
)]
pub struct ToolMarker;

/// Axe tool entity - deals damage.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name = "Axe",
    ToolMarker,
    Damage = Damage(1.0),
)]
pub struct Axe;
impl Tool for Axe {}

/// Hoe tool entity.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Name = "Hoe", ToolMarker)]
pub struct Hoe;
impl Tool for Hoe {}

/// WateringCan tool entity.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Name = "WateringCan", ToolMarker)]
pub struct WateringCan;
impl Tool for WateringCan {}

fn tool_collider() -> Collider {
    Collider::rectangle(6.0, 6.0)
}

fn tool_collision_layers() -> CollisionLayers {
    // Start with no collisions - enabled when Active is added
    CollisionLayers::NONE
}

/// Marks an entity as a target for a specific tool type.
/// Collider must be provided by the object spawning this.
/// Uses Kinematic RigidBody so Tool sensors can detect it.
#[derive(Component)]
#[require(
    RigidBody::Kinematic,
    CollisionLayers = tool_target_collision_layers(),
)]
pub struct ToolTarget<T: Tool>(PhantomData<T>);

impl<T: Tool> ToolTarget<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Tool> Default for ToolTarget<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn tool_target_collision_layers() -> CollisionLayers {
    CollisionLayers::new(GameLayer::Object, GameLayer::Tool)
}
