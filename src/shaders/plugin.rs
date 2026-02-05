use super::materials::TreeShakeMaterial;
use super::systems::{convert_trees_to_mesh2d, trigger_tree_shake, update_tree_shake};
use bevy::{prelude::*, sprite_render::Material2dPlugin};

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<TreeShakeMaterial>::default())
            .add_systems(Update, convert_trees_to_mesh2d)
            .add_systems(Update, (trigger_tree_shake, update_tree_shake));
    }
}
