use super::materials::TreeShakeMaterial;
use crate::core::messages::Hit;
use crate::objects::components::Tree;
use crate::tools::components::Axe;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::sprite_render::MeshMaterial2d;
use bevy_ecs_tiled::prelude::TiledObjectVisuals;

/// Tracks tree shake state.
#[derive(Component, Default)]
pub struct TreeShake {
    pub intensity: f32,
    pub material: Handle<TreeShakeMaterial>,
}

/// Converts tree sprites to Mesh2d with TreeShakeMaterial after Tiled spawn.
pub fn convert_trees_to_mesh2d(
    mut commands: Commands,
    trees: Query<(Entity, &TiledObjectVisuals), Added<Tree>>,
    sprites: Query<(&Sprite, &Anchor, &Transform)>,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TreeShakeMaterial>>,
) {
    for (tree_entity, visuals) in &trees {
        let Some(&visual_entity) = visuals.first() else {
            continue;
        };

        let Ok((sprite, anchor, transform)) = sprites.get(visual_entity) else {
            continue;
        };

        // Get image handle from sprite
        let image_handle = sprite.image.clone();

        // Get image dimensions for mesh size
        let Some(image) = images.get(&image_handle) else {
            continue;
        };
        let size = image.size_f32();

        // Calculate offset to compensate for anchor (Mesh2d is always centered)
        let anchor_offset = anchor.as_vec() * size;

        // Create quad mesh matching sprite size
        let mesh = meshes.add(Rectangle::from_size(size));

        // Create material with tree texture
        let material_handle = materials.add(TreeShakeMaterial {
            shake_intensity: 0.0,
            shake_speed: 20.0,
            texture: image_handle,
        });

        // Apply anchor offset to transform
        let mut new_transform = *transform;
        new_transform.translation.x -= anchor_offset.x;
        new_transform.translation.y -= anchor_offset.y;

        // Replace Sprite with Mesh2d + Material on visual child
        commands
            .entity(visual_entity)
            .remove::<(Sprite, Anchor)>()
            .insert((
                Mesh2d(mesh),
                MeshMaterial2d(material_handle.clone()),
                new_transform,
            ));

        // Add TreeShake to tree parent for hit tracking
        commands.entity(tree_entity).insert(TreeShake {
            intensity: 0.0,
            material: material_handle,
        });
    }
}

/// Triggers tree shake when hit by axe.
pub fn trigger_tree_shake(mut hits: MessageReader<Hit<Axe>>, mut shakes: Query<&mut TreeShake>) {
    for hit in hits.read() {
        if let Ok(mut shake) = shakes.get_mut(hit.target.entity()) {
            shake.intensity = 1.0;
        }
    }
}

/// Updates shake intensity (decay) and syncs to material.
pub fn update_tree_shake(
    time: Res<Time>,
    mut shakes: Query<&mut TreeShake>,
    mut materials: ResMut<Assets<TreeShakeMaterial>>,
) {
    for mut shake in &mut shakes {
        if shake.intensity > 0.0 {
            // Decay (~0.5s duration)
            shake.intensity = (shake.intensity - time.delta_secs() * 2.0).max(0.0);

            // Sync to material
            if let Some(mat) = materials.get_mut(&shake.material) {
                mat.shake_intensity = shake.intensity;
            }
        }
    }
}
