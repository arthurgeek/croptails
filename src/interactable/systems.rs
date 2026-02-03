use super::components::{Interactable, InteractableActive, InteractableSensor};
use crate::player::Player;
use avian2d::prelude::*;
use bevy::prelude::*;

/// Adds InteractableActive when player enters an interactable sensor.
pub fn on_player_enter_interactable(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    sensors: Query<&ChildOf, With<InteractableSensor>>,
    players: Query<(), With<Player>>,
    interactables: Query<(), (With<Interactable>, Without<InteractableActive>)>,
) {
    for evt in collision_events.read() {
        // Check which is sensor and which is player
        let (sensor_collider, player_body) = if sensors.contains(evt.collider1) {
            (evt.collider1, evt.body2)
        } else if sensors.contains(evt.collider2) {
            (evt.collider2, evt.body1)
        } else {
            continue;
        };

        // Check if other body is player
        let Some(player_body) = player_body else {
            continue;
        };
        if !players.contains(player_body) {
            continue;
        }

        // Get the interactable parent
        let Ok(child_of) = sensors.get(sensor_collider) else {
            continue;
        };
        let interactable_entity = child_of.parent();

        // Add active marker if not already active
        if interactables.contains(interactable_entity) {
            commands
                .entity(interactable_entity)
                .insert(InteractableActive);
        }
    }
}

/// Removes InteractableActive when player exits an interactable sensor.
pub fn on_player_exit_interactable(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEnd>,
    sensors: Query<&ChildOf, With<InteractableSensor>>,
    players: Query<(), With<Player>>,
    interactables: Query<(), (With<Interactable>, With<InteractableActive>)>,
) {
    for evt in collision_events.read() {
        // Check which is sensor and which is player
        let (sensor_collider, player_body) = if sensors.contains(evt.collider1) {
            (evt.collider1, evt.body2)
        } else if sensors.contains(evt.collider2) {
            (evt.collider2, evt.body1)
        } else {
            continue;
        };

        // Check if other body is player
        let Some(player_body) = player_body else {
            continue;
        };
        if !players.contains(player_body) {
            continue;
        }

        // Get the interactable parent
        let Ok(child_of) = sensors.get(sensor_collider) else {
            continue;
        };
        let interactable_entity = child_of.parent();

        // Remove active marker
        if interactables.contains(interactable_entity) {
            commands
                .entity(interactable_entity)
                .remove::<InteractableActive>();
        }
    }
}
