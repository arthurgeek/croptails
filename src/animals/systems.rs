use super::{components::Chicken, resources::ChickenAtlas};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TiledObjectVisuals;

/// Loads the chicken sprite sheet.
pub fn load_chicken_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("game/characters/chicken_sprites.png");
    // 4 columns x 2 rows, 16x16 per frame
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        4,
        2,
        None,
        None,
    ));
    commands.insert_resource(ChickenAtlas { texture, layout });
}

/// Hides the Tiled sprite visual for chickens (we use our own animated sprite).
pub fn hide_tiled_chicken_visual(
    mut commands: Commands,
    chickens: Query<&TiledObjectVisuals, Added<Chicken>>,
) {
    for visuals in &chickens {
        for visual_entity in visuals.iter() {
            commands.entity(visual_entity).insert(Visibility::Hidden);
        }
    }
}
