use bevy::prelude::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PlayerAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PlayerActionsAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PlayerDirection(pub Vec2);
