use super::{
    components::{Chicken, ChickenAnimation},
    resources::ChickenAtlas,
    systems::{hide_tiled_chicken_visual, load_chicken_atlas},
};
use crate::core::systems::{on_start_moving, on_stop_moving, sync_animation};
use crate::npcs::NpcSystemSet;
use bevy::prelude::*;

pub struct AnimalsPlugin;

impl Plugin for AnimalsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChickenAtlas>()
            .register_type::<Chicken>()
            .register_type::<ChickenAnimation>()
            // Startup
            .add_systems(Startup, load_chicken_atlas)
            // Animation - use generic systems from core, run after NPC movement
            .add_systems(
                FixedUpdate,
                (
                    on_start_moving::<ChickenAnimation>,
                    on_stop_moving::<ChickenAnimation>,
                    sync_animation::<ChickenAnimation>,
                )
                    .chain()
                    .after(NpcSystemSet::Movement),
            )
            // Hide Tiled visual (we use our own sprite)
            .add_systems(Update, hide_tiled_chicken_visual);
    }
}
