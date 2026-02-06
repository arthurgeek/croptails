use super::materials::ShakeMaterial;
use super::systems::{setup_rock_shaker, setup_tree_shaker, trigger_shake, update_shake};
use bevy::{prelude::*, sprite_render::Material2dPlugin};

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<ShakeMaterial>::default())
            .add_systems(Update, (setup_tree_shaker, setup_rock_shaker))
            .add_systems(Update, (trigger_shake, update_shake));
    }
}
