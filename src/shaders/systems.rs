use super::materials::ShakeMaterial;
use crate::core::messages::Hit;
use crate::objects::components::{Rock, Tree};
use crate::tools::components::Axe;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::sprite_render::MeshMaterial2d;
use bevy_ecs_tiled::prelude::TiledObjectVisuals;

/// Tracks shake state for an entity.
#[derive(Component)]
pub struct Shaker {
    pub intensity: f32,
    pub max_intensity: f32,
    pub decay_rate: f32,
    pub material: Handle<ShakeMaterial>,
}

impl Shaker {
    /// Tree shake: intensity 1.0, duration ~0.5s
    pub fn tree(material: Handle<ShakeMaterial>) -> Self {
        Self {
            intensity: 0.0,
            max_intensity: 1.0,
            decay_rate: 2.0,
            material,
        }
    }

    /// Rock shake: intensity 0.6, duration ~0.25s
    pub fn rock(material: Handle<ShakeMaterial>) -> Self {
        Self {
            intensity: 0.0,
            max_intensity: 0.6,
            decay_rate: 4.0,
            material,
        }
    }
}

/// Converts tree sprites to Mesh2d with ShakeMaterial after Tiled spawn.
pub fn setup_tree_shaker(
    mut commands: Commands,
    trees: Query<(Entity, &TiledObjectVisuals), Added<Tree>>,
    sprites: Query<(&Sprite, &Anchor, &Transform)>,
    images: Res<Assets<Image>>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ShakeMaterial>>,
) {
    for (tree_entity, visuals) in &trees {
        if let Some(material_handle) = setup_shaker_mesh(
            &mut commands,
            visuals,
            &sprites,
            &images,
            &layouts,
            &mut meshes,
            &mut materials,
        ) {
            commands
                .entity(tree_entity)
                .insert(Shaker::tree(material_handle));
        }
    }
}

/// Converts rock sprites to Mesh2d with ShakeMaterial after Tiled spawn.
pub fn setup_rock_shaker(
    mut commands: Commands,
    rocks: Query<(Entity, &TiledObjectVisuals), Added<Rock>>,
    sprites: Query<(&Sprite, &Anchor, &Transform)>,
    images: Res<Assets<Image>>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ShakeMaterial>>,
) {
    for (rock_entity, visuals) in &rocks {
        if let Some(material_handle) = setup_shaker_mesh(
            &mut commands,
            visuals,
            &sprites,
            &images,
            &layouts,
            &mut meshes,
            &mut materials,
        ) {
            commands
                .entity(rock_entity)
                .insert(Shaker::rock(material_handle));
        }
    }
}

/// Helper to convert a Tiled sprite to Mesh2d with ShakeMaterial.
/// Returns the material handle if successful.
fn setup_shaker_mesh(
    commands: &mut Commands,
    visuals: &TiledObjectVisuals,
    sprites: &Query<(&Sprite, &Anchor, &Transform)>,
    images: &Res<Assets<Image>>,
    layouts: &Res<Assets<TextureAtlasLayout>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ShakeMaterial>>,
) -> Option<Handle<ShakeMaterial>> {
    let &visual_entity = visuals.first()?;
    let (sprite, anchor, transform) = sprites.get(visual_entity).ok()?;

    let image_handle = sprite.image.clone();
    let image = images.get(&image_handle)?;
    let image_size = image.size_f32();

    // Calculate UV rect and mesh size based on whether sprite uses atlas
    let (uv_offset, uv_scale, mesh_size) = if let Some(atlas) = &sprite.texture_atlas {
        let layout = layouts.get(&atlas.layout)?;
        let rect = layout.textures.get(atlas.index)?;

        let offset = Vec2::new(
            rect.min.x as f32 / image_size.x,
            rect.min.y as f32 / image_size.y,
        );
        let scale = Vec2::new(
            (rect.max.x - rect.min.x) as f32 / image_size.x,
            (rect.max.y - rect.min.y) as f32 / image_size.y,
        );
        let size = Vec2::new(
            (rect.max.x - rect.min.x) as f32,
            (rect.max.y - rect.min.y) as f32,
        );
        (offset, scale, size)
    } else {
        // Full image, no atlas
        (Vec2::ZERO, Vec2::ONE, image_size)
    };

    // Calculate offset to compensate for anchor (Mesh2d is always centered)
    let anchor_offset = anchor.as_vec() * mesh_size;

    let mesh = meshes.add(Rectangle::from_size(mesh_size));
    let material_handle = materials.add(ShakeMaterial {
        shake_intensity: 0.0,
        shake_speed: 20.0,
        uv_offset,
        uv_scale,
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

    Some(material_handle)
}

/// Triggers shake when hit by axe.
pub fn trigger_shake(mut hits: MessageReader<Hit<Axe>>, mut shakers: Query<&mut Shaker>) {
    for hit in hits.read() {
        if let Ok(mut shaker) = shakers.get_mut(hit.target.entity()) {
            shaker.intensity = shaker.max_intensity;
        }
    }
}

/// Updates shake intensity (decay) and syncs to material.
pub fn update_shake(
    time: Res<Time>,
    mut shakers: Query<&mut Shaker>,
    mut materials: ResMut<Assets<ShakeMaterial>>,
) {
    for mut shaker in &mut shakers {
        if shaker.intensity > 0.0 {
            shaker.intensity = (shaker.intensity - time.delta_secs() * shaker.decay_rate).max(0.0);

            if let Some(mat) = materials.get_mut(&shaker.material) {
                mat.shake_intensity = shaker.intensity;
            }
        }
    }
}
