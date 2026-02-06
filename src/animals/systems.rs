use super::resources::{ChickenAtlas, CowAtlas};
use crate::npcs::components::Npc;
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

/// Hides the Tiled sprite visual for NPCs (we use our own animated sprites).
pub fn hide_tiled_npc_visual(
    mut commands: Commands,
    npcs: Query<&TiledObjectVisuals, Added<Npc>>,
) {
    for visuals in &npcs {
        for visual_entity in visuals.iter() {
            commands.entity(visual_entity).insert(Visibility::Hidden);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Cow
// ─────────────────────────────────────────────────────────────────────────────

/// Loads the cow sprite sheet.
pub fn load_cow_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("game/characters/cow_sprites.png");
    // 5 columns x 2 rows, 32x32 per frame
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        5,
        2,
        None,
        None,
    ));
    commands.insert_resource(CowAtlas { texture, layout });
}
