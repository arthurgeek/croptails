use super::{
    components::{Door, DoorState},
    resources::DoorAtlas,
    systems::{
        load_door_atlas, on_door_animation_finished, on_door_closing, on_door_opening,
        on_interactable_activated, on_interactable_deactivated, on_start_closing, on_start_opening,
    },
};
use bevy::prelude::*;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Door>()
            .register_type::<DoorState>()
            .register_type::<DoorAtlas>()
            .add_systems(Startup, load_door_atlas)
            .add_systems(
                Update,
                (
                    on_interactable_activated,
                    on_interactable_deactivated,
                    on_start_opening,
                    on_start_closing,
                    on_door_animation_finished,
                    on_door_opening,
                    on_door_closing,
                ),
            );
    }
}
