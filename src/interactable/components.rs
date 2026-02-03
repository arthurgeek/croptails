use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;

/// Marks an entity as interactable. Add a Collider child for the sensor area.
/// When player enters the sensor, `InteractableActive` is added.
/// When player exits, `InteractableActive` is removed.
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Interactable;

/// Marker: player is currently in the interactable area.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct InteractableActive;

/// Sensor collider for detecting player proximity.
/// Add this as a child of an Interactable entity.
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(
    Sensor,
    CollisionEventsEnabled,
    CollisionLayers = CollisionLayers::new(GameLayer::Interactable, GameLayer::Player),
)]
pub struct InteractableSensor;
