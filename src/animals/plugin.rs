use super::{
    components::{Chicken, ChickenAnimation, Cow, CowAnimation},
    resources::{ChickenAtlas, CowAtlas},
    systems::{hide_tiled_npc_visual, load_chicken_atlas, load_cow_atlas},
};
use crate::core::systems::{on_start_moving, on_stop_moving, sync_animation};
use crate::npcs::NpcSystemSet;
use bevy::prelude::*;

pub struct AnimalsPlugin;

impl Plugin for AnimalsPlugin {
    fn build(&self, app: &mut App) {
        app // Chicken
            .register_type::<ChickenAtlas>()
            .register_type::<Chicken>()
            .register_type::<ChickenAnimation>()
            // Cow
            .register_type::<CowAtlas>()
            .register_type::<Cow>()
            .register_type::<CowAnimation>()
            // Startup - load atlases
            .add_systems(Startup, (load_chicken_atlas, load_cow_atlas))
            // Animation - use generic systems from core, run after NPC movement
            .add_systems(
                FixedUpdate,
                (
                    // Chicken animation
                    on_start_moving::<ChickenAnimation>,
                    on_stop_moving::<ChickenAnimation>,
                    sync_animation::<ChickenAnimation>,
                    // Cow animation
                    on_start_moving::<CowAnimation>,
                    on_stop_moving::<CowAnimation>,
                    sync_animation::<CowAnimation>,
                )
                    .after(NpcSystemSet::Movement),
            )
            // Hide Tiled visual for all NPCs (we use our own sprites)
            .add_systems(Update, hide_tiled_npc_visual);
    }
}
