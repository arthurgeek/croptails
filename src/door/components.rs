use crate::interactable::{Interactable, InteractableSensor};
use avian2d::prelude::*;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

/// Door entity with open/close functionality.
/// Visual and collider are defined in Tiled, this component adds animation behavior.
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(Name = "Door", DoorState, Interactable)]
#[component(on_add = Self::on_add)]
pub struct Door;

impl Door {
    /// Frame sequences for door animations.
    pub const OPEN_FRAMES: [usize; 4] = [1, 3, 2, 0];
    pub const CLOSE_FRAMES: [usize; 4] = [0, 2, 3, 1];
    pub const FPS: u8 = 5;

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        // Spawn sensor child - taller and skinnier than door for better detection
        world.commands().entity(ctx.entity).with_child((
            InteractableSensor,
            Collider::rectangle(8.0, 36.0),
            Transform::from_xyz(8.0, 8.0, 0.0),
        ));
    }
}

/// Current state of the door.
#[derive(Component, Reflect, Default, Clone, Copy, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub enum DoorState {
    #[default]
    Closed,
    Open,
    Opening,
    Closing,
}
