use bevy::prelude::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ObjectsAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

/// Tile indices for objects in basic_grass_biome_things.png (9 columns, 16x16)
pub mod tiles {
    pub const LOG: usize = 23; // row 3, col 6 (1-indexed)
}
