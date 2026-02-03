use bevy::prelude::*;

/// Atlas resource for door sprites.
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct DoorAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
