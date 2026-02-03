use super::{
    components::{Interactable, InteractableActive, InteractableSensor},
    systems::{on_player_enter_interactable, on_player_exit_interactable},
};
use bevy::prelude::*;

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Interactable>()
            .register_type::<InteractableActive>()
            .register_type::<InteractableSensor>()
            .add_systems(
                Update,
                (on_player_enter_interactable, on_player_exit_interactable),
            );
    }
}
