use super::{
    components::{Collectable, Log, Tree, TreeVariant},
    resources::ObjectsAtlas,
    systems::{
        apply_axe_damage, collect_items, configure_tree_health, despawn_pending, load_objects_atlas,
    },
};
use crate::player::systems::remove_chopping_on_animation_end;
use bevy::prelude::*;

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tree>()
            .register_type::<TreeVariant>()
            .register_type::<Log>()
            .register_type::<Collectable>()
            .register_type::<ObjectsAtlas>()
            .add_systems(Startup, load_objects_atlas)
            .add_systems(Update, configure_tree_health)
            .add_systems(Update, apply_axe_damage)
            .add_systems(
                Update,
                despawn_pending.before(remove_chopping_on_animation_end),
            )
            .add_systems(Update, collect_items);
    }
}
