use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load a map asset and retrieve its handle
    let map_handle = asset_server.load("tiled/maps/test_map_default.tmx");

    // Spawn the map centered in the view
    commands.spawn((TiledMap(map_handle), TilemapAnchor::Center));
}
